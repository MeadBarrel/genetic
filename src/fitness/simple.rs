use std::marker::PhantomData;

use crate::types::*;
use crate::error::Result;

pub struct SimpleFitnessFunction<P, F, FF> 
    where
        FF: Fn(&P) -> Result<F> + Send + Sync,
{
    function: FF,
    phenotype: PhantomData<P>,
    fitness: PhantomData<F>,
}

impl<P, F, FF> SimpleFitnessFunction<P, F, FF>
    where
        FF: Fn(&P) -> Result<F> + Send + Sync,
{
    pub fn new(function: FF) -> SimpleFitnessFunction<P, F, FF> {
        Self {
            function,
            phenotype: PhantomData,
            fitness: PhantomData
        }
    }
}

impl<P, F, FF> FitnessFunction for SimpleFitnessFunction<P, F, FF> 
    where
        F: Fitness,
        P: for<'a> Phenotype<'a>,
        FF: Fn(&P) -> Result<F> + Send + Sync,
{
    type Phenotype = P;
    type Fitness = F;

    fn evaluate<'a, T>(&'a self, phenotypes_with_fitnesses: T) -> Result<Vec<Self::Fitness>>
            where
                T: Iterator<Item = (&'a Self::Phenotype, Option<&'a Self::Fitness>)> {
        let result = phenotypes_with_fitnesses
            .map(|(phenotype, previous_fitness)| {
                match previous_fitness {
                    Some(ex) => Ok(ex.clone()),
                    None => (self.function)(phenotype)
                }
            })
            .collect::<Result<Vec<F>>>();
        result
    }
}
