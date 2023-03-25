use crate::error::Result;
use crate::population::*;

pub trait Genotype: Clone + Send + Sync {}
pub trait Phenotype: Clone + Send + Sync {}
pub trait Fitness: Clone + Ord + Send + Sync {}

pub trait FitnessFunction: Send + Sync {
    type Phenotype: Phenotype;
    type Fitness: Fitness;

    fn evaluate(&self, phenotypes_with_fitnesses: &[(&Self::Phenotype, Option<&Self::Fitness>)]) -> Result<Vec<Self::Fitness>>;
}

pub trait Incubator: Clone {
    type Genotype: Genotype;
    type Phenotype: Phenotype;

    fn grow(&self, genome: &Self::Genotype) -> Result<Self::Phenotype>;
}

pub trait MutateOperator {
    type Genotype: Genotype;
    
    fn mutate(&mut self, genome: &mut Self::Genotype) -> Result<()>;
}

pub trait CrossoverOperator {
    type Genotype: Genotype;

    fn crossover(&mut self, genomes: &[&Self::Genotype]) -> Result<Vec<Self::Genotype>>;
}

pub trait SelectOperator {
    fn select<'a, G, F>(&mut self, population: &'a SortedPopulation<G, F>) -> Result<Vec<Vec<&'a G>>>
        where
            G: Genotype,
            F: Fitness,
    ;
}

pub trait ReinsertOperator {
    fn reinsert<G, F>(&mut self, population: SortedPopulation<G, F>) -> Result<UnsortedPopulation<G, F>>
        where 
            G: Genotype,
            F: Fitness
    ;
}
