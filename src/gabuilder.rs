use std::marker::PhantomData;

use crate::population::*;
use crate::types::*;
use crate::ga::*;
use crate::error::*;

#[derive(Default)]
pub struct GeneticAlgorithmBuilder<I, F, S, C, M, R>
{
    pub incubator: I,
    pub fitness_function: F,
    pub select: S,
    pub crossover: C,
    pub mutate: M,
    pub reinsert: R,
    _phantom: PhantomData<()>
}

impl GeneticAlgorithmBuilder<(), (), (), (), (), ()> {
    pub fn with_incubator<I>(self, incubator: I) -> GeneticAlgorithmBuilder<I, (), (), (), (), ()> {
        GeneticAlgorithmBuilder {
            incubator,
            fitness_function: (),
            select: (),
            crossover: (),
            mutate: (),
            reinsert: (),
            _phantom: PhantomData,
        }
    }
}

impl<I> GeneticAlgorithmBuilder<I, (), (), (), (), ()> 
    where 
        I: Incubator
{
    pub fn with_fitness_function<F>(self, fitness_function: F) -> GeneticAlgorithmBuilder<I, F, (), (), (), ()> {
        GeneticAlgorithmBuilder {
            incubator: self.incubator,
            fitness_function,
            select: (),
            crossover: (),
            mutate: (),
            reinsert: (),
            _phantom: PhantomData,
        }
    }
}

impl<I, F, S, C, M, R> GeneticAlgorithmBuilder<I, F, S, C, M, R> 
{

    pub fn with_select<SNEW>(self, select: SNEW) -> GeneticAlgorithmBuilder<I, F, SNEW, C, M, R> {
        GeneticAlgorithmBuilder {
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select,
            crossover: self.crossover,
            mutate: self.mutate,
            reinsert: self.reinsert,
            _phantom: PhantomData,
        }
    }

    pub fn with_crossover<CNEW>(self, crossover: CNEW) -> GeneticAlgorithmBuilder<I, F, S, CNEW, M, R> {
        GeneticAlgorithmBuilder {
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select: self.select,
            crossover,
            mutate: self.mutate,
            reinsert: self.reinsert,
            _phantom: PhantomData,
        }        
    }

    pub fn with_mutate<MNEW>(self, mutate: MNEW) -> GeneticAlgorithmBuilder<I, F, S, C, MNEW, R> {
        GeneticAlgorithmBuilder {
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select: self.select,
            crossover: self.crossover,
            mutate,
            reinsert: self.reinsert,
            _phantom: PhantomData,
        }        
    }

    pub fn with_reinsert<RNEW>(self, reinsert: RNEW) -> GeneticAlgorithmBuilder<I, F, S, C, M, RNEW> {
        GeneticAlgorithmBuilder {
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select: self.select,
            crossover: self.crossover,
            mutate: self.mutate,
            reinsert,
            _phantom: PhantomData,
        }        
    }
}

impl<I, F, S, C, M, R> GeneticAlgorithmBuilder<I, F, S, C, M, R>
    where
        I: Incubator,
        F: FitnessFunction<Phenotype = I::Phenotype>,
        S: SelectOperator,
        C: CrossoverOperator<Genotype = I::Genotype>,
        M: MutateOperator<Genotype = I::Genotype>,
        R: ReinsertOperator
{
    pub fn create_population(&self, genomes: Vec<I::Genotype>) -> Result<SortedPopulation<I::Genotype, F::Fitness>> 
    {
        Population::default()
            .add_children(genomes)
            .sort(&self.incubator, &self.fitness_function)
    }

    pub fn build(self) -> GeneticAlgorithm<I, F, S, C, M, R> {
        GeneticAlgorithm {
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select: self.select,
            crossover: self.crossover,
            mutate: self.mutate,
            reinsert: self.reinsert,
        }
    }
}
