use crate::types::*;
use crate::error::*;

pub struct MultiFitness2<F1, F2, P>
    where
        P: Phenotype,
        F1: FitnessFunction<Phenotype = P>,
        F2: FitnessFunction<Phenotype = P>,
{
    fitness_function1: F1,
    fitness_function2: F2,
}

impl<F1, F2, P> MultiFitness2<F1, F2, P>
    where
        P: Phenotype,
        F1: FitnessFunction<Phenotype = P>,
        F2: FitnessFunction<Phenotype = P>,
{
    pub fn new(fitness_function1: F1, fitness_function2: F2) -> Self {
        Self {
            fitness_function1,
            fitness_function2,
        }
    }
}

impl<F1, F2, P> FitnessFunction for MultiFitness2<F1, F2, P>
    where
        P: Phenotype,
        F1: FitnessFunction<Phenotype = P>,
        F2: FitnessFunction<Phenotype = P>,
{
    type Phenotype = P;
    type Fitness = (F1::Fitness, F2::Fitness);

    fn evaluate(
        &self,
        phenotypes_with_fitnesses: &[(&Self::Phenotype, Option<&Self::Fitness>)],
    ) -> Result<Vec<Self::Fitness>> {
        let phenotypes_with_fitnesses1: Vec<(&Self::Phenotype, Option<&F1::Fitness>)> =
            phenotypes_with_fitnesses
                .iter()
                .map(|(phenotype, fitness)| (*phenotype, fitness.as_ref().map(|f| &f.0)))
                .collect();
    
        let phenotypes_with_fitnesses2: Vec<(&Self::Phenotype, Option<&F2::Fitness>)> =
            phenotypes_with_fitnesses
                .iter()
                .map(|(phenotype, fitness)| (*phenotype, fitness.as_ref().map(|f| &f.1)))
                .collect();
    
        let fitnesses1 = self.fitness_function1.evaluate(&phenotypes_with_fitnesses1)?;
        let fitnesses2 = self.fitness_function2.evaluate(&phenotypes_with_fitnesses2)?;
    
        fitnesses1
            .into_iter()
            .zip(fitnesses2.into_iter())
            .map(|(f1, f2)| Ok((f1, f2)))
            .collect()
    }
}
