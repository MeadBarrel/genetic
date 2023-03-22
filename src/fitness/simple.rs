use crate::types::*;
use crate::error::Result;

pub struct SimpleFitnessFunction<F>(F);

impl<F> FitnessFunction for SimpleFitnessFunction<F>
    where
        F: for<'a> Fn(&Self::Phenotype<'a>) -> Result<Self::Fitness>
{
    fn evaluate(&self, phenotypes_with_fitnesses: &[(&Self::Phenotype<'_>, Option<&Self::Fitness>)]) -> Result<Vec<Self::Fitness>> {
        let result = phenotypes_with_fitnesses
            .into_iter()
            .map(|(phenotype, previous_fitness)| {
                match previous_fitness {
                    Some(ex) => {let a = Ok((*ex).clone()); a},
                    None => (self.function)(phenotype)
                }
            })
            .collect::<Result<Vec<F>>>();
        result
                
    }
}
