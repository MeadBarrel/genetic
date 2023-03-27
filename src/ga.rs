use crate::types::*;
use crate::population::*;
use crate::error::*;

pub struct GeneticAlgorithm<I, F, S, C, M, R>
{
    pub incubator: I,
    pub fitness_function: F,
    pub select: S,
    pub crossover: C,
    pub mutate: M,
    pub reinsert: R,    
}

impl<I, F, S, C, M, R> GeneticAlgorithm<I, F, S, C, M, R>
    where 
        I: Incubator,
        F: FitnessFunction<Phenotype = I::Phenotype>,
        S: SelectOperator,
        C: CrossoverOperator<Genotype = I::Genotype>,
        M: MutateOperator<Genotype = I::Genotype>,
        R: ReinsertOperator
{
    pub fn advance(&mut self, population: SortedPopulation<I::Genotype, F::Fitness>) -> Result<SortedPopulation<I::Genotype, F::Fitness>> {
        let parents = self.select.select(&population)?;

        let mut offsprings = parents
            .into_iter()
            .map(|p| self.crossover.crossover(&p))
            .collect::<Result<Vec<Vec<I::Genotype>>>>()?
            .concat();

        for genome in offsprings.iter_mut() {
            self.mutate.mutate(genome)?;
        }

        
        
        let population = population.add_children(offsprings);
        dbg!(population.individuals.len());
        let population = population.sort(&self.incubator, &self.fitness_function)?;

        

        let population = self.reinsert.reinsert(population)?;
        
        population.sort(&self.incubator, &self.fitness_function)
    }    
}
