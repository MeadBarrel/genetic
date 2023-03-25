use crate::population::*;
use crate::types::*;
use crate::error::*;

pub struct ElitistReinserter;

impl ReinsertOperator for ElitistReinserter {
    fn reinsert<G, F>(&mut self, population: SortedPopulation<G, F>) -> Result<UnsortedPopulation<G, F>>
            where 
                G: Genotype,
                F: Fitness 
    {
        let pop_size = population.previous_generation_size();
        let result = population.truncate(pop_size);
        Ok(result)
    }
}
