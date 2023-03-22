use std::ops::Index;
use std::marker::PhantomData;

use rayon::slice::ParallelSliceMut;

use crate::types::*;
use crate::individual::*;
use crate::error::*;

pub struct Sorted;

pub struct Population<G: Genotype, F: Fitness, S> {
    pub(crate) individuals: Vec<Individual<G, F>>,
    pub(crate) generation: u64,
    pub(crate) sorted: PhantomData<S>,
    pub(crate) num_children: usize,
}

pub type UnsortedPopulation<G, F> = Population<G, F, ()>;
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

impl<G, F> UnsortedPopulation<G, F> 
    where
        G: Genotype,
        F: Fitness,
{
    pub fn new() -> Self {
        Self {
            individuals: Vec::default(),
            generation: 0,
            num_children: 0,
            sorted: PhantomData
        }
    }

    pub fn sort<P, I, FF>(mut self, incubator: &I, fitness_function: &FF) -> Result<SortedPopulation<G, F>>
        where
            P: for<'a> Phenotype<'a>,
            I: Incubator<Genotype = G, Phenotype = P>,
            FF: FitnessFunction<Phenotype = P, Fitness = F>
    {
        let phenotypes = self.individuals
            .iter()
            .map(|individual| incubator.grow(&individual.genome))
            .collect::<Result<Vec<_>>>()?;

        let phenotypes_with_fitnesses = phenotypes
            .iter()
            .zip(self.fitnesses());

        let new_fitnesses = fitness_function.evaluate(phenotypes_with_fitnesses)?;

        self.individuals
            .iter_mut()
            .zip(new_fitnesses.into_iter())
            .for_each(|(mut individual, fitness)| individual.fitness = Some(fitness));

        self.individuals.par_sort_by(|individual1, individual2| {
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

#[cfg(test)]
mod tests {
    use crate::population;

    use super::*;

    impl Genotype for usize {}
    //impl Fitness for usize {}
    impl<'a> Phenotype<'a> for usize {}

    pub struct UsizeFitnessFunction;

    impl FitnessFunction for UsizeFitnessFunction {
        type Phenotype = usize;
        type Fitness = usize;

        fn evaluate<'a, T>(&'a self, phenotypes_with_fitnesses: T) -> Result<Vec<Self::Fitness>>
                where
                    T: Iterator<Item = (&'a Self::Phenotype, Option<&'a Self::Fitness>)> {
            let result = phenotypes_with_fitnesses
                        .map(|(phenotype, fitness)| fitness.cloned().unwrap_or_else(|| phenotype*2))
                        .collect();
            Ok(result)
        }
    }

    pub struct UsizeIncubator;

    impl Incubator for UsizeIncubator {
        type Genotype = usize;
        type Phenotype = usize;

        fn grow(&self, genome: &Self::Genotype) -> Result<Self::Phenotype> {
            Ok(*genome)
        }
    }

    #[test]
    fn test_population_sort() {
        let fitness_function = UsizeFitnessFunction;
        let incubator = UsizeIncubator;

        let genomes = vec![
            1231,
            918,
            71,
            991,
            15,
            71,
            22,
            9,
            912
        ];

        let population = UnsortedPopulation::new();
        let population = population.add_children(genomes);
        let population = population.sort(&incubator, &fitness_function).unwrap();

        let individuals: Vec<usize> = population.individuals
            .into_iter()
            .map(|x| x.fitness.unwrap())
            .collect();

        let expected: Vec<usize> = vec![1231, 991, 918, 912, 71, 71, 22, 15, 9]
            .into_iter().map(|x| x*2).collect();

        assert_eq!(individuals, expected);
    }
}