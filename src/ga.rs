use crate::types::*;
use crate::population::*;
use crate::error::*;
use std::marker::PhantomData;


pub struct GeneticAlgorithm
<
    G: Genotype,
    P: Phenotype,
    F: Fitness,
    I: Incubator<Genotype = G, Phenotype = P>,
    FF: FitnessFunction<Phenotype = P, Fitness = F>,
    S: SelectOperator,
    C: CrossoverOperator<Genotype = G>,
    M: MutateOperator<Genotype = G>,
    R: ReinsertOperator,
> {
    pub(crate) genotype: PhantomData<G>,
    pub(crate) phenotype: PhantomData<P>,
    pub(crate) fitness: PhantomData<F>,
    pub(crate) incubator: I,
    pub(crate) fitness_function: FF,
    pub(crate) select: S,
    pub(crate) crossover: C,
    pub(crate) mutate: M,
    pub(crate) reinsert: R,
}

impl<G, P, F, I, FF, S, C, M, R> GeneticAlgorithm<G, P, F, I, FF, S, C, M, R> 
    where
        G: Genotype,
        P: Phenotype,
        F: Fitness,
        I: Incubator<Genotype = G, Phenotype = P>,
        FF: FitnessFunction<Phenotype = P, Fitness = F>,
        S: SelectOperator,
        C: CrossoverOperator<Genotype = G>,
        M: MutateOperator<Genotype = G>,
        R: ReinsertOperator
{
    pub fn advance(&mut self, population: SortedPopulation<G, F>) -> Result<SortedPopulation<G, F>> {
        let parents = self.select.select(&population)?;

        let mut offsprings = parents
            .into_iter()
            .map(|p| self.crossover.crossover(&p))
            .collect::<Result<Vec<Vec<G>>>>()?
            .concat();

        for genome in offsprings.iter_mut() {
            self.mutate.mutate(genome)?;
        }
        
        let population = population.add_children(offsprings);
        let population = population.sort(&self.incubator, &self.fitness_function)?;

        let population = self.reinsert.reinsert(population)?;
        
        population.sort(&self.incubator, &self.fitness_function)
    }
}