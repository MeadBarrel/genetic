use std::marker::PhantomData;

use crate::types::*;
use rayon::slice::ParallelSliceMut;
use crate::error::*;

pub type ObjectiveFunction<P> = Box<dyn Fn(&P)->f64>;

#[derive(Clone)]
pub struct ParetoFitness {
    rank: usize,
    crowding_distance: f64,
    objectives: Vec<f64>
}

impl Fitness for ParetoFitness {}

pub struct ParetoFitnessFunction<P, F> 
{
    _phantom: PhantomData<P>,
    objectives: F,
}

impl Default for ParetoFitnessFunction<(), ()>
{
    fn default() -> Self {
        Self { objectives: (),  _phantom: PhantomData}
    }
}

impl ParetoFitnessFunction<(), ()>
{
    pub fn with_objective<P>(self, objective: Box<dyn Fn(&P)->f64>) -> ParetoFitnessFunction<P, Vec<ObjectiveFunction<P>>> 
        where 
            P: Phenotype,
    {
        let objectives = vec![objective];
        ParetoFitnessFunction {
            objectives,
            _phantom: PhantomData
        }
    }

    pub fn with_objectives<P, F>(self, objective: F) -> ParetoFitnessFunction<P, F>
        where 
            P: Phenotype,
            F: Fn(&P)->Vec<f64>
    {
        ParetoFitnessFunction { _phantom: PhantomData, objectives: objective }
    }
}

impl<P, F> ParetoFitnessFunction<P, F>
{
    pub fn evaluate_objectives(&self, objectives: Vec<Vec<f64>>) -> Result<Vec<ParetoFitness>> {
        let ranks = pareto_ranks(&objectives);
        let crowding_dists = crowding_distances(&objectives);
        let result = ranks.
            into_iter()
            .zip(
                crowding_dists.into_iter().zip(objectives.into_iter())                
            )
            .map(|(rank, (crowding_distance, objectives))| ParetoFitness {
                rank, crowding_distance, objectives
            })
            .collect();
        Ok(result)        

    }
}

impl<P> ParetoFitnessFunction<P, Vec<ObjectiveFunction<P>>> 
    where P: Phenotype
{
    pub fn with_objective(mut self, objective: Box<dyn Fn(&P)->f64>) -> ParetoFitnessFunction<P, Vec<ObjectiveFunction<P>>> {
        self.objectives.push(objective);
        self
    }
}

impl<P> FitnessFunction for ParetoFitnessFunction<P, Vec<ObjectiveFunction<P>>> 
    where 
        P: Phenotype,
{
    type Phenotype = P;
    type Fitness = ParetoFitness;

    fn evaluate(&self, phenotypes_with_fitnesses: &[(&Self::Phenotype, Option<&Self::Fitness>)]) -> Result<Vec<Self::Fitness>> {
        let objectives: Vec<Vec<f64>> = phenotypes_with_fitnesses
                    .iter()
                    .map(|(phenotype, fitness)| {
                        let objectives = fitness.map(|x| x.objectives.clone());
                        objectives.unwrap_or_else(
                            || self.objectives.iter().map(|f| f(*phenotype)).collect()
                        )
                    })
                    .collect();
        self.evaluate_objectives(objectives)
    }

}

impl<P, F> FitnessFunction for ParetoFitnessFunction<P, F>
    where
        P: Phenotype,
        F: Fn(&P)->Vec<f64>
{
    type Fitness = ParetoFitness;
    type Phenotype = P;

    fn evaluate(&self, phenotypes_with_fitnesses: &[(&Self::Phenotype, Option<&Self::Fitness>)]) -> Result<Vec<Self::Fitness>> {
        let objectives = phenotypes_with_fitnesses
            .iter()
            .map(|(phenotype, fitness)| {
                let objectives = fitness.map(|x| x.objectives.clone());
                objectives.unwrap_or_else(
                    || (self.objectives)(phenotype)
                )
            })
            .collect();
        self.evaluate_objectives(objectives)
    }
}


impl PartialEq for ParetoFitness {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.crowding_distance == other.crowding_distance
            && self.objectives == other.objectives
    }
}

impl Eq for ParetoFitness {}

impl PartialOrd for ParetoFitness {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.rank.partial_cmp(&other.rank) {
            Some(std::cmp::Ordering::Equal) => other.crowding_distance.partial_cmp(&self.crowding_distance),
            // compare based on rank first
            Some(o) => Some(o),
            None => None,
        }
    }
}

impl Ord for ParetoFitness {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn pareto_ranks(vectors: &[Vec<f64>]) -> Vec<usize> {
    let mut indices: Vec<_> = (0..vectors.len()).collect();
    let mut ranks = vec![0; vectors.len()];
    let mut cur_rank = 0;

    let dominates = |this: &[f64], other: &[f64]| {
        other.iter().enumerate().all(|(i, &a)| a >= this[i])
        && other.iter().enumerate().any(|(i, &a)| a > this[i])
    };

    while !indices.is_empty() {
        let eff_indices = indices.clone();
        indices.retain(|&i| {
            if eff_indices.iter().any(|&j| j != i && dominates(&vectors[i], &vectors[j])) {
                true
            } else {
                ranks[i] = cur_rank;
                false
            }
        });
        cur_rank += 1;
    }

    ranks
}

fn crowding_distances(vectors: &[Vec<f64>]) -> Vec<f64> {
    let len = vectors.len();
    let mut dist = vec![0.0; len];
    let num_objectives = vectors[0].len();

    for i in 0..num_objectives {
        let mut sorted_indices: Vec<_> = (0..len).collect();
        sorted_indices.par_sort_unstable_by(|&a, &b| vectors[a][i].partial_cmp(&vectors[b][i]).unwrap());

        dist[sorted_indices[0]] = std::f64::INFINITY;
        dist[sorted_indices[len-1]] = std::f64::INFINITY;

        let min = vectors[sorted_indices[0]][i];
        let max = vectors[sorted_indices[len - 1]][i];

        if max == min {
            continue;
        }

        let diff = max - min;

        for j in 1..(len-1) {
            let prev_index = sorted_indices[j - 1];
            let next_index = sorted_indices[j + 1];

            let d = (vectors[next_index][i] - vectors[prev_index][i]) / diff;

            dist[sorted_indices[j]] += d;
        }
    };

    dist
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::*;

    impl Phenotype for usize {}

    #[test]
    fn test_pareto_fitness_objectives() {
        let points = vec![
            vec![0.7873616773923351, 0.8092306552000161],
            vec![0.21173326011754878, 0.6339482992398732],
            vec![0.01725675132713511, 0.9881718237619325],
            vec![0.5330575947747812, 0.9857357852889478],
            vec![0.5829186417619112, 0.5495024479309618],
            vec![0.3521920654953825, 0.9142557605053708],
            vec![0.3692810621902112, 0.08987228791660551],
            vec![0.7478009420313325, 0.3523304812577952],
            vec![0.5212182747402428, 0.41024277235906326],
            vec![0.6000844913877189, 0.3594561767427774],
            vec![0.21823414269097896, 0.8820946957442006],
            vec![0.4550299655344954, 0.6162078310693472],
            vec![0.17710113749892753, 0.006050443424864049],
            vec![0.744808216764824, 0.11893987805784223],
            vec![0.08517607238714664, 0.5755688995187869],
            vec![0.0311175093718834, 0.14680352435542987],
            vec![0.9406975842111823, 0.36328027015743847],
            vec![0.49042703806432253, 0.21626967830636024],
            vec![0.11508201721525246, 0.9030739711618478],
            vec![0.7212068364234988, 0.1843117185801686],
            vec![0.4720653136444348, 0.32004342948828035],
            vec![0.7285032162065087, 0.38694809427377175],
            vec![0.47187397860969094, 0.7384091109267817],
            vec![0.17896648902209567, 0.779927706301946],
            vec![0.2441719609991977, 0.8338028399022548],
            vec![0.6138092170178797, 0.096676922062096],
            vec![0.03926807017744294, 0.6405796332697564],
            vec![0.3597415915757063, 0.7480627116447116],
            vec![0.8332102156679112, 0.23308833651764094],
            vec![0.7571160739197182, 0.9997153582193037],
        ];

        let expected_ranks = vec![
            0,
            4,
            1,
            1,
            1,
            2,
            4, 
            1,
            2,
            2,
            3,
            3,
            5,
            2,
            5,
            6,
            0, 
            3, 
            3, 
            2, 
            3, 
            1, 
            2, 
            4, 
            3, 
            3,
            5,
            2,
            1,
            0,
        ];   

        let objectives_func = |index: &usize| points[*index].clone();

        let phenotypes_with_fitnesses_static: Vec<(usize, Option<ParetoFitness>)> = points
            .iter()
            .enumerate()
            .map(|(i, _)| (i, None))
            .collect();
        
        let phenotypes_with_fitnesses: Vec<_> = phenotypes_with_fitnesses_static
            .iter()
            .map(|(index, opt)| (index, opt.as_ref()))
            .collect();

        let fitness_function = ParetoFitnessFunction::default().with_objectives(objectives_func);
        let fitnesses = fitness_function.evaluate(&phenotypes_with_fitnesses).unwrap();

        let ranks: Vec<_> = fitnesses
            .into_iter()
            .map(|f| f.rank)
            .collect();

        assert_eq!(ranks, expected_ranks);
    }

    #[test]
    fn test_crowding_distance() {
        let points = vec![
            vec![0.7873616773923351, 0.8092306552000161],
            vec![0.21173326011754878, 0.6339482992398732],
            vec![0.01725675132713511, 0.9881718237619325],
            vec![0.5330575947747812, 0.9857357852889478],
            vec![0.5829186417619112, 0.5495024479309618],
            vec![0.3521920654953825, 0.9142557605053708],
            vec![0.3692810621902112, 0.08987228791660551],
            vec![0.7478009420313325, 0.3523304812577952],
            vec![0.5212182747402428, 0.41024277235906326],
            vec![0.6000844913877189, 0.3594561767427774],
            vec![0.21823414269097896, 0.8820946957442006],
            vec![0.4550299655344954, 0.6162078310693472],
            vec![0.17710113749892753, 0.006050443424864049],
            vec![0.744808216764824, 0.11893987805784223],
            vec![0.08517607238714664, 0.5755688995187869],
            vec![0.0311175093718834, 0.14680352435542987],
            vec![0.9406975842111823, 0.36328027015743847],
            vec![0.49042703806432253, 0.21626967830636024],
            vec![0.11508201721525246, 0.9030739711618478],
            vec![0.7212068364234988, 0.1843117185801686],
            vec![0.4720653136444348, 0.32004342948828035],
            vec![0.7285032162065087, 0.38694809427377175],
            vec![0.47187397860969094, 0.7384091109267817],
            vec![0.17896648902209567, 0.779927706301946],
            vec![0.2441719609991977, 0.8338028399022548],
            vec![0.6138092170178797, 0.096676922062096],
            vec![0.03926807017744294, 0.6405796332697564],
            vec![0.3597415915757063, 0.7480627116447116],
            vec![0.8332102156679112, 0.23308833651764094],
            vec![0.7571160739197182, 0.9997153582193037],
        ];

        let inf = f64::INFINITY;

        let expected = vec![
            0.13662144722870032,
            0.06705037798426683,
            inf,
            0.14120303635919387,
            0.23896401583642363,
            0.2083399165100112,
            0.1943926566786755,
            0.05299227965872994,
            0.20975561802766074,
            0.04447120104564087,
            0.10484084479780645,
            0.16985012321216233,
            inf,
            0.07134381421489303,
            0.14923007104520658,
            0.08962481535931327,
            inf,
            0.10231564880634747,
            0.13191234933490628,
            0.1941119008950743,
            0.1400935970439103,
            0.07281999655957663,
            0.12661602170405725,
            0.09906126589340619,
            0.2183924770512246,
            0.16041707402003105,
            0.16366716348478308,
            0.060289080229606086,
            0.2704837949183645,
            inf,
        ];



        let actual = crowding_distances(&points);

        for (a, e) in actual.into_iter().zip(expected.into_iter()) {
            assert!(approx_eq!(f64, a, e, epsilon = 0.001))
        }    
    }

    #[test]
    fn test_pareto_ranks_long() {
        let points = vec![
            vec![0.7873616773923351, 0.8092306552000161],
            vec![0.21173326011754878, 0.6339482992398732],
            vec![0.01725675132713511, 0.9881718237619325],
            vec![0.5330575947747812, 0.9857357852889478],
            vec![0.5829186417619112, 0.5495024479309618],
            vec![0.3521920654953825, 0.9142557605053708],
            vec![0.3692810621902112, 0.08987228791660551],
            vec![0.7478009420313325, 0.3523304812577952],
            vec![0.5212182747402428, 0.41024277235906326],
            vec![0.6000844913877189, 0.3594561767427774],
            vec![0.21823414269097896, 0.8820946957442006],
            vec![0.4550299655344954, 0.6162078310693472],
            vec![0.17710113749892753, 0.006050443424864049],
            vec![0.744808216764824, 0.11893987805784223],
            vec![0.08517607238714664, 0.5755688995187869],
            vec![0.0311175093718834, 0.14680352435542987],
            vec![0.9406975842111823, 0.36328027015743847],
            vec![0.49042703806432253, 0.21626967830636024],
            vec![0.11508201721525246, 0.9030739711618478],
            vec![0.7212068364234988, 0.1843117185801686],
            vec![0.4720653136444348, 0.32004342948828035],
            vec![0.7285032162065087, 0.38694809427377175],
            vec![0.47187397860969094, 0.7384091109267817],
            vec![0.17896648902209567, 0.779927706301946],
            vec![0.2441719609991977, 0.8338028399022548],
            vec![0.6138092170178797, 0.096676922062096],
            vec![0.03926807017744294, 0.6405796332697564],
            vec![0.3597415915757063, 0.7480627116447116],
            vec![0.8332102156679112, 0.23308833651764094],
            vec![0.7571160739197182, 0.9997153582193037],
        ];        
        let expected = vec![
            0,
            4,
            1,
            1,
            1,
            2,
            4, 
            1,
            2,
            2,
            3,
            3,
            5,
            2,
            5,
            6,
            0, 
            3, 
            3, 
            2, 
            3, 
            1, 
            2, 
            4, 
            3, 
            3,
            5,
            2,
            1,
            0,
        ];        
        assert_eq!(pareto_ranks(&points), expected);
    }

}