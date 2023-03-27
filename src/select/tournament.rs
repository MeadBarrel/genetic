use rand::*;
use crate::types::*;
use crate::error::Result;
use crate::population::*;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(default)]
pub struct TournamentSelectionBuilder<R> {
    pub tournament_size: usize,
    pub num_children: usize,
    pub num_parents: usize,
    pub selection_probability: f64,
    rng: R
}

impl Default for TournamentSelectionBuilder<()> {
    fn default() -> Self {
        Self {
            tournament_size: 12,
            num_children: 4,
            num_parents: 2,
            selection_probability: 0.8,
            rng: ()
        }
    }
}

impl<R> TournamentSelectionBuilder<R> {
    pub fn with_tournament_size(mut self, size: usize) -> Self {
        self.tournament_size = size;
        self
    }

    pub fn with_num_children(mut self, num_children: usize) -> Self {
        self.num_children = num_children;
        self
    }

    pub fn with_num_parents(mut self, num_parents: usize) -> Self {
        self.num_parents = num_parents;
        self
    }

    pub fn with_selection_probability(mut self, probability: f64) -> Self {
        self.selection_probability = probability;
        self
    }

    pub fn with_rng<RNG: Rng>(self, rng: RNG) -> TournamentSelectionBuilder<RNG> {
        TournamentSelectionBuilder { 
            tournament_size: self.tournament_size,
            num_children: self.num_children,
            num_parents: self.num_parents,
            selection_probability: self.selection_probability,
            rng
        }
    }
}

impl<R> TournamentSelectionBuilder<R> where R: Rng {
    pub fn build(self) -> TournamentSelection<R> {
        TournamentSelection { 
            tournament_size: self.tournament_size,
            num_children: self.num_children,
            num_parents: self.num_parents,
            selection_probability: self.selection_probability,
            rng: self.rng
        }
    }
}


pub struct TournamentSelection<R: Rng> {
    pub tournament_size: usize,
    pub num_children: usize,
    pub num_parents: usize,
    pub selection_probability: f64,
    rng: R,
}

impl<R: Rng> SelectOperator for TournamentSelection<R> {
    fn select<'a, G, F>(&mut self, population: &'a SortedPopulation<G, F>) -> Result<Vec<Vec<&'a G>>>
    where
        G: Genotype,
        F: Fitness,
    {
        let population_size = population.individuals.len();
        let mut selected_parents: Vec<Vec<&'a G>> = vec![];

        for _ in 0..self.num_children {
            let mut parents: Vec<&'a G> = vec![];
            for _ in 0..self.num_parents {
                let mut tournament = (0..self.tournament_size)
                    .map(|_| {
                        let idx = self.rng.gen_range(0..population_size);
                        &population.individuals[idx]
                    })
                    .collect::<Vec<_>>();

                tournament.sort_by(|i1, i2| i2.fitness.as_ref().unwrap().cmp(i1.fitness.as_ref().unwrap()));

                let mut accumulated_probability = 0.0;
                let mut selected_parent = None;
                let rand_value = self.rng.gen_range(0.0..1.0);

                for individual in tournament.iter() {
                    accumulated_probability += self.selection_probability * (1.0 - accumulated_probability);
                    if rand_value <= accumulated_probability {
                        selected_parent = Some(individual);
                        break;
                    }
                }

                let selected_parent = selected_parent.unwrap_or_else(|| {
                    tournament
                        .last()
                        .expect("Tournament should have at least one participant")
                });

                parents.push(&selected_parent.genome);
            }
            selected_parents.push(parents);
        }

        Ok(selected_parents)
    }
}

#[cfg(test)]
mod tests {
    use crate::population::*;
    use crate::individual::*;
    use std::marker::PhantomData;
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn create_test_population() -> SortedPopulation<usize, usize> {
        let genomes: Vec<usize> = (0..1000).collect();
        let individuals: Vec<_> = genomes
            .into_iter()
            .map(|x| Individual {
                generation: 0,
                genome: x,
                fitness: Some(x)
            })
            .collect();

        SortedPopulation {
            individuals,
            generation: 0,
            num_children: 0,
            sorted: PhantomData,
        }
    }

    #[test]
    fn test_tournament_selection() {
        let mut rng = StdRng::seed_from_u64(0);

        let population = create_test_population();

        let mut selection = TournamentSelection {
            tournament_size: 3,
            num_children: 2,
            num_parents: 2,
            selection_probability: 0.5,
            rng: &mut rng,
        };

        let selected_parents = selection.select(&population).unwrap();

        // Check if the correct number of children and parents are selected
        assert_eq!(selected_parents.len(), 2);
        assert_eq!(selected_parents[0].len(), 2);
        assert_eq!(selected_parents[1].len(), 2);

        // Check the actual selected parents (may vary depending on RNG seed)
        assert_eq!(selected_parents[0], vec![&731, &262]);
        assert_eq!(selected_parents[1], vec![&996, &964]);
    }
}