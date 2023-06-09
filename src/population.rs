use std::ops::Index;
use std::marker::PhantomData;

use crate::types::*;
use crate::individual::*;
use crate::error::*;

pub struct Sorted;
pub struct Unsorted;

pub struct Population<G: Genotype, F: Fitness, S> {
    pub(crate) individuals: Vec<Individual<G, F>>,
    pub(crate) generation: u64,
    pub(crate) sorted: PhantomData<S>,
    pub(crate) num_children: usize,
}

pub type UnsortedPopulation<G, F> = Population<G, F, Unsorted>;
pub type SortedPopulation<G, F> = Population<G, F, Sorted>;

impl<G, F, S> Population<G, F, S> 
    where
        G: Genotype,
        F: Fitness,
{
    pub fn add_children(mut self, genomes: Vec<G>) -> UnsortedPopulation<G, F> {
        let num_children = genomes.len();
        self.individuals.extend(
            genomes
                .into_iter()
                .map(|genome| Individual { generation: self.generation, genome, fitness: None })
        );
        
        UnsortedPopulation {
            individuals: self.individuals,
            generation: self.generation,
            num_children: self.num_children + num_children,
            sorted: PhantomData,
        }
    }

    pub fn truncate(mut self, len: usize) -> UnsortedPopulation<G, F> {
        self.individuals.truncate(len);
        UnsortedPopulation {
            individuals: self.individuals,
            generation: self.generation,
            sorted: PhantomData,
            num_children: self.num_children,
        }
    }

    pub fn get_num_children(&self) -> usize {
        self.num_children
    }

    pub fn previous_generation_size(&self) -> usize {
        self.individuals.len() - self.num_children
    }

    pub fn next_generation(&mut self) {
        self.generation += 1;
        self.num_children = 0;
    }

    pub fn take_current_generation(&mut self) -> Vec<Individual<G, F>> {
        self.individuals
            .drain_filter(|individual| individual.generation == self.generation)
            .collect()
    }

    pub fn fitnesses(&self) -> impl Iterator<Item=Option<&F>> {
        self.individuals
            .iter()
            .map(|individual| individual.fitness.as_ref())
    }
}

impl<G, F> Default for UnsortedPopulation<G, F> 
    where
        G: Genotype,
        F: Fitness,
{

    fn default() -> Self {
        Self {
            individuals: Vec::default(),
            generation: 0,
            num_children: 0,
            sorted: PhantomData
        }
        
    }
}

impl<G, F> UnsortedPopulation<G, F> 
    where
        G: Genotype,
        F: Fitness,
{

    pub fn sort<P, I, FF>(mut self, incubator: &I, fitness_function: &FF) -> Result<SortedPopulation<G, F>>
        where
            I: Incubator<Genotype = G, Phenotype = P>,
            P: Phenotype,
            FF: FitnessFunction<Fitness = F, Phenotype = P>
    {
        let phenotypes = self.individuals
            .iter()
            .map(|individual| incubator.grow(&individual.genome))
            .collect::<Result<Vec<_>>>()?;

        let phenotypes_with_fitnesses: Vec<_> = phenotypes
            .iter()
            .zip(self.fitnesses())
            .collect();

        let new_fitnesses = fitness_function.evaluate(&phenotypes_with_fitnesses)?;

        self.individuals
            .iter_mut()
            .zip(new_fitnesses.into_iter())
            .for_each(|(mut individual, fitness)| individual.fitness = Some(fitness));

        self.individuals.sort_by(|individual1, individual2| {
            let a = individual1.fitness.as_ref().unwrap();
            let b = individual2.fitness.as_ref().unwrap();
            b.cmp(a)
        });
   
        let result = SortedPopulation {
            individuals: self.individuals,
            generation: self.generation,
            num_children: self.num_children,
            sorted: PhantomData
        };

        Ok(result)
    }
}

impl<G, F, S> Index<usize> for Population<G, F, S>
where
    G: Genotype,
    F: Fitness,
{
    type Output = Individual<G, F>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.individuals[index]
    }
}

impl<G, F> SortedPopulation<G, F> 
    where
        G: Genotype,
        F: Fitness,
{
    pub fn best(&self) -> &Individual<G, F> {
        &self.individuals[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::population;

    use super::*;

    impl Genotype for usize {}
    //impl Fitness for usize {}
    impl Phenotype for String {}

    pub struct UsizeFitnessFunction<'a>(PhantomData<&'a ()>);

    impl<'a> FitnessFunction for UsizeFitnessFunction<'a> {
        type Phenotype = String;
        type Fitness = usize;

        fn evaluate(&self, phenotypes_with_fitnesses: &[(&Self::Phenotype, Option<&Self::Fitness>)]) -> Result<Vec<Self::Fitness>> {
            let result = phenotypes_with_fitnesses
                        .into_iter()
                        .map(|(phenotype, fitness)| fitness.cloned().unwrap_or_else(|| phenotype.len()))
                        .collect();
            Ok(result)            
        }
    }

    #[derive(Clone)]
    pub struct UsizeIncubator(Vec<String>);

    impl Incubator for UsizeIncubator {
        type Genotype = usize;
        type Phenotype = String;

        fn grow(&self, genome: &Self::Genotype) -> Result<Self::Phenotype> {
            Ok(self.0[*genome].to_string())
        }
    }

    #[test]
    fn test_population_sort() {
        let fitness_function = UsizeFitnessFunction(PhantomData);
        let incubator = UsizeIncubator(vec![
            "apples".into(),
            "oranges".into(),
            "wheat".into(),
            "coconuts".into(),
            "stuff".into(),
            "grapes".into()
        ]);

        let genomes = vec![
            5, 3, 2, 4, 1
        ];

        let population = UnsortedPopulation::default();
        let population = population.add_children(genomes);
        let population = population.sort(&incubator, &fitness_function).unwrap();

        let individuals: Vec<usize> = population.individuals
            .into_iter()
            .map(|x| x.fitness.unwrap())
            .collect();

        let expected: Vec<usize> = vec![8, 7, 6, 5, 5];

        assert_eq!(individuals, expected);
    }
}