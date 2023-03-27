use crate::types::*;
use crate::error::*;

#[derive(Clone)]
pub struct SimpleFitnessFunction<F, P, Fit, B>
where
    F: Fn(&P) -> Result<Fit>,
    P: Phenotype,
    Fit: Fitness,
    B: FitnessBehavior,
{
    fitness_function: F,
    _marker: std::marker::PhantomData<(P, Fit, B)>,
}

impl<F, P, Fit, B> SimpleFitnessFunction<F, P, Fit, B>
where
    F: Fn(&P) -> Result<Fit>,
    P: Phenotype,
    Fit: Fitness,
    B: FitnessBehavior,
{
    pub fn new(fitness_function: F) -> Self {
        Self {
            fitness_function,
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct SimpleFitness<F, P, Fit> {
    fitness_function: F,
    _marker: std::marker::PhantomData<(P, Fit)>,
}

impl<F, P, Fit> SimpleFitness<F, P, Fit>
where
    F: Fn(&P) -> Result<Fit> ,
    P: Phenotype,
    Fit: Fitness,
{
    pub fn new(fitness_function: F) -> Self {
        Self {
            fitness_function,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn use_existing_fitness(self) -> SimpleFitnessFunction<F, P, Fit, UseExistingFitness> {
        SimpleFitnessFunction::new(self.fitness_function)
    }

    pub fn recalculate_fitness(self) -> SimpleFitnessFunction<F, P, Fit, RecalculateFitness> {
        SimpleFitnessFunction::new(self.fitness_function)
    }
}

pub trait FitnessBehavior {
    fn handle_existing_fitness<'a, Fit: Fitness>(
        existing_fitness: &'a Option<&'a Fit>,
        fitness_function: impl FnOnce() -> Result<Fit>,
    ) -> Result<Fit>;
}

pub struct UseExistingFitness;
pub struct RecalculateFitness;

impl FitnessBehavior for UseExistingFitness {
    fn handle_existing_fitness<'a, Fit: Fitness>(
        existing_fitness: &'a Option<&'a Fit>,
        fitness_function: impl FnOnce() -> Result<Fit>,
    ) -> Result<Fit> {
        match existing_fitness {
            Some(fitness) => Ok((*fitness).clone()),
            None => fitness_function(),
        }
    }
}

impl FitnessBehavior for RecalculateFitness {
    fn handle_existing_fitness<'a, Fit: Fitness>(
        _existing_fitness: &'a Option<&'a Fit>,
        fitness_function: impl FnOnce() -> Result<Fit>,
    ) -> Result<Fit> {
        fitness_function()
    }
}

impl<F, P, Fit, B> FitnessFunction for SimpleFitnessFunction<F, P, Fit, B>
where
    F: Fn(&P) -> Result<Fit> + Send + Sync,
    P: Phenotype,
    Fit: Fitness,
    B: FitnessBehavior + Send + Sync,
{
    type Phenotype = P;
    type Fitness = Fit;

    fn evaluate(
        &self,
        phenotypes_with_fitnesses: &[(&Self::Phenotype, Option<&Self::Fitness>)],
    ) -> Result<Vec<Self::Fitness>> {
        phenotypes_with_fitnesses
            .iter()
            .map(|(phenotype, existing_fitness)| {
                B::handle_existing_fitness(
                    existing_fitness, 
                    || (self.fitness_function)(phenotype)
                )
            }).collect()
    }
}
