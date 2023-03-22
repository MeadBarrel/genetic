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

impl<'a, I, F, S, C, M, R> GeneticAlgorithm<I, F, S, C, M, R>
    where 
        I: Incubator + 'a,
        F: FitnessFunction<Phenotype<'a> = I::Phenotype<'a>>,
        S: SelectOperator,
        C: CrossoverOperator<Genotype = I::Genotype>,
        M: MutateOperator<Genotype = I::Genotype>,
        R: ReinsertOperator
{
    pub fn advance(&'a mut self, population: SortedPopulation<I::Genotype, F::Fitness>) -> Result<SortedPopulation<I::Genotype, F::Fitness>> {
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
        let population = population.sort(&self.incubator, &self.fitness_function)?;

        let population = self.reinsert.reinsert(population)?;
        
        population.sort(&self.incubator, &self.fitness_function)
    }    
}

// use crate::types::*;
// use crate::population::*;
// use crate::error::*;
// use std::marker::PhantomData;


// pub struct GeneticAlgorithm
// <
//     'a, 
//     G: Genotype,
//     P: for<'b> Phenotype<'b>,
//     F: Fitness,
//     I: for<'b> Incubator<Genotype = G, Phenotype<'b> = P> + 'a,
//     FF: FitnessFunction<Phenotype = P, Fitness = F>,
//     S: SelectOperator,
//     C: CrossoverOperator<Genotype = G>,
//     M: MutateOperator<Genotype = G>,
//     R: ReinsertOperator,
// > {
//     pub(crate) genotype: PhantomData<G>,
//     pub(crate) phenotype: PhantomData<P>,
//     pub(crate) fitness: PhantomData<F>,
//     pub(crate) incubator: I,
//     pub(crate) fitness_function: FF,
//     pub(crate) select: S,
//     pub(crate) crossover: C,
//     pub(crate) mutate: M,
//     pub(crate) reinsert: R,
//     _phantom: PhantomData<&'a ()>
// }

// impl<'a, G, P, F, I, FF, S, C, M, R> GeneticAlgorithm<'a, G, P, F, I, FF, S, C, M, R> 
//     where
//         G: Genotype,
//         P: for<'b> Phenotype<'b>,
//         F: Fitness,
//         I: for<'b> Incubator<Genotype = G, Phenotype<'b> = P>,
//         FF: FitnessFunction<Phenotype = P, Fitness = F>,
//         S: SelectOperator,
//         C: CrossoverOperator<Genotype = G>,
//         M: MutateOperator<Genotype = G>,
//         R: ReinsertOperator
// {
//     pub fn advance(&'a mut self, population: SortedPopulation<G, F>) -> Result<SortedPopulation<G, F>> {
//         let parents = self.select.select(&population)?;

//         let mut offsprings = parents
//             .into_iter()
//             .map(|p| self.crossover.crossover(&p))
//             .collect::<Result<Vec<Vec<G>>>>()?
//             .concat();

//         for genome in offsprings.iter_mut() {
//             self.mutate.mutate(genome)?;
//         }
        
//         let population = population.add_children(offsprings);
//         let population = population.sort(&self.incubator, &self.fitness_function)?;

//         let population = self.reinsert.reinsert(population)?;
        
//         population.sort(&self.incubator, &self.fitness_function)
//     }
// }