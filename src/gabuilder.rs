use std::marker::PhantomData;

use crate::population::Population;
use crate::population::SortedPopulation;
use crate::types::*;
use crate::ga::*;
use crate::error::*;

pub struct GeneticAlgorithmBuilder;

pub struct GeneticAlgorithmBuilderIncubator<'a, G, P, I> {
    genotype: PhantomData<G>,
    phenotype: PhantomData<P>,
    incubator: I,
    lifetime: PhantomData<&'a ()>
}

pub struct GeneticAlgorithmBuilderFitnessFunction<'a, G, P, I, F, FF, S, C, M, R> {
    genotype: PhantomData<G>,
    phenotype: PhantomData<P>,
    fitness: PhantomData<F>,
    incubator: I,
    fitness_function: FF,    
    select: S,
    crossover: C,
    mutate: M,
    reinsert: R,
    lifetime: PhantomData<&'a ()>
}

impl GeneticAlgorithmBuilder {
    pub fn new() -> Self { Self }
    pub fn with_incubator<'a, G, P, I>(self, incubator: I) -> GeneticAlgorithmBuilderIncubator<'a, G, P, I>
        where
            G: Genotype,
            P: Phenotype<'a>,
            I: Incubator<'a, Genotype = G, Phenotype = P>
    {
        GeneticAlgorithmBuilderIncubator {
            genotype: PhantomData,
            phenotype: PhantomData,
            incubator,
            lifetime: PhantomData,
        }
    }

}

impl<'a, G, P, I> GeneticAlgorithmBuilderIncubator<'a, G, P, I> 
    where 
        P: Phenotype<'a>
{
    pub fn with_fitness<F, FF>(self, fitness: FF) -> GeneticAlgorithmBuilderFitnessFunction<'a, G, P, I, F, FF, (), (), (), ()> 
        where
            F: Fitness,
            FF: FitnessFunction<Fitness = F, Phenotype = P>
    {
        GeneticAlgorithmBuilderFitnessFunction {
            genotype: PhantomData,
            phenotype: PhantomData,
            fitness: PhantomData,
            incubator: self.incubator,
            fitness_function: fitness,
            select: (),
            crossover: (),
            mutate: (),
            reinsert: (),
            lifetime: PhantomData,
        }
    }
}

impl<'a, G, P, I, F, FF, C, M, R> GeneticAlgorithmBuilderFitnessFunction<'a, G, P, I, F, FF, (), C, M, R> 
    where
        G: Genotype,
        F: Fitness,
{
    pub fn with_select<S>(self, select: S) -> GeneticAlgorithmBuilderFitnessFunction<'a, G, P, I, F, FF, S, C, M, R>
        where
            S: SelectOperator
    {
        GeneticAlgorithmBuilderFitnessFunction {
            genotype: PhantomData,
            phenotype: PhantomData,
            fitness: PhantomData,
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select,
            crossover: self.crossover,
            mutate: self.mutate,
            reinsert: self.reinsert,
            lifetime: PhantomData,
        }
    }

}

impl<'a, G, P, I, F, FF, S, M, R> GeneticAlgorithmBuilderFitnessFunction<'a, G, P, I, F, FF, S, (), M, R> 
    where
        G: Genotype
{
    pub fn with_crossver<C>(self, crossover: C) -> GeneticAlgorithmBuilderFitnessFunction<'a, G, P, I, F, FF, S, C, M, R> 
        where
            C: CrossoverOperator<Genotype = G>
    {
        GeneticAlgorithmBuilderFitnessFunction { 
            genotype: PhantomData,
            phenotype: PhantomData,
            fitness: PhantomData,
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select: self.select,
            crossover,
            mutate: self.mutate,
            reinsert: self.reinsert,
            lifetime: PhantomData,
        }
    }
}

impl<'a, G, P, I, F, FF, S, C, R> GeneticAlgorithmBuilderFitnessFunction<'a, G, P, I, F, FF, S, C, (), R> 
    where
        G: Genotype,
{
    pub fn with_mutate<M>(self, mutate: M) -> GeneticAlgorithmBuilderFitnessFunction<'a, G, P, I, F, FF, S, C, M, R> 
        where
            M: MutateOperator<Genotype=G>
    {
        GeneticAlgorithmBuilderFitnessFunction {
            genotype: PhantomData,
            phenotype: PhantomData,
            fitness: PhantomData,
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select: self.select,
            crossover: self.crossover,
            mutate,
            reinsert: self.reinsert,
            lifetime: PhantomData,
        }
    }
}

impl<'a, G, P, I, F, FF, S, C, M> GeneticAlgorithmBuilderFitnessFunction<'a, G, P, I, F, FF, S, C, M, ()> 
    where
        G: Genotype,
        F: Fitness,
{
    pub fn with_reinsert<R>(self, reinsert: R) -> GeneticAlgorithmBuilderFitnessFunction<'a, G, P, I, F, FF, S, C, M, R>
        where
            R: ReinsertOperator
    {
        GeneticAlgorithmBuilderFitnessFunction { 
            genotype: PhantomData,
            fitness: PhantomData,
            phenotype: PhantomData,
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select: self.select,
            crossover: self.crossover,
            mutate: self.mutate,
            reinsert,
            lifetime: PhantomData,
        }
    }
}

impl<'a, G, P, I, F, FF, S, C, M, R> GeneticAlgorithmBuilderFitnessFunction<'a, G, P, I, F, FF, S, C, M, R> 
    where
        G: Genotype,
        P: for<'b> Phenotype<'b>,
        I: Incubator<'a, Genotype = G, Phenotype = P>,
        F: Fitness,
        FF: FitnessFunction<Phenotype = P, Fitness = F>,
        S: SelectOperator,
        C: CrossoverOperator<Genotype = G>,
        M: MutateOperator<Genotype = G>,
        R: ReinsertOperator
{
    pub fn create_population(&'a self, genomes: Vec<G>) -> Result<SortedPopulation<G, F>> {
        Population::new()
            .add_children(genomes)
            .sort(&self.incubator, &self.fitness_function)
    }

    pub fn build(self) -> GeneticAlgorithm<'a, G, P, F, I, FF, S, C, M, R> {
        GeneticAlgorithm { 
            genotype: PhantomData,
            phenotype: PhantomData,
            fitness: PhantomData,
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select: self.select,
            crossover: self.crossover,
            mutate: self.mutate,
            reinsert: self.reinsert,
            _phantom: PhantomData,
        }
    }
}