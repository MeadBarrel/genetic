//! This module provides an exhaustive crossover function for genetic algorithms.
//!
//! The exhaustive crossover function takes two parent genotypes and returns
//! all possible combinations of their genes while keeping the genes' locus fixed.

use std::fmt::Debug;
use std::marker::PhantomData;
use crate::error::*;
use crate::types::*;
use serde::Deserialize;



#[derive(Debug, Clone, Deserialize)]
pub struct ExhausiveCrossover<G> { _phantom: PhantomData<G> }

impl<G> Default for ExhausiveCrossover<G>
{
    fn default() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<G> CrossoverOperator for ExhausiveCrossover<G> 
    where G: Send + Sync + Copy + Debug
{
    type Genotype = Vec<G>;

    fn crossover(&mut self, genomes: &[&Self::Genotype]) -> Result<Vec<Self::Genotype>> {
        if genomes.len() != 2 {
            return Err(Error::Genetic("Exhausive crossover only works with 2 parents".into()))
        };

        exhaustive_crossover(genomes[0], genomes[1])
    }
}

/// Perform an exhaustive crossover between two parent genotypes.
///
/// Given two parent genotypes of equal length, this function generates all
/// possible offspring by combining the genes from the parents at each locus.
///
/// # Arguments
///
/// * `parent1` - The first parent genotype.
/// * `parent2` - The second parent genotype.
///
/// # Returns
///
/// A vector containing all possible offspring genotypes.
///
/// # Panics
///
/// Panics if the lengths of the parent genotypes are not equal.
///
/// # Example
///
/// ```
/// use genetic::crossover::exhaustive::exhaustive_crossover;
///
/// let parent1 = vec![1.0, 2.0, 3.0];
/// let parent2 = vec![4.0, 5.0, 6.0];
///
/// let children = exhaustive_crossover(&parent1, &parent2);
/// println!("{:?}", children);
/// ```
pub fn exhaustive_crossover<T: Copy + Debug>(parent1: &[T], parent2: &[T]) -> Result<Vec<Vec<T>>> {
    if parent1.len() != parent2.len() {
        return Err(Error::Genetic("Parent genotypes must have the same length".to_string()))
    }

    let n = parent1.len();
    let combinations = 2_usize.pow(n as u32);

    let mut offspring = Vec::with_capacity(combinations);

    for i in 0..combinations {
        let mut child = Vec::with_capacity(n);
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                child.push(parent1[j]);
            } else {
                child.push(parent2[j]);
            }
        }
        offspring.push(child);
    }

    Ok(offspring)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exhaustive_crossover() {
        let parent1 = vec![1.0, 2.0, 3.0];
        let parent2 = vec![4.0, 5.0, 6.0];

        let children = exhaustive_crossover(&parent1, &parent2).unwrap();

        let expected_children = vec![
            vec![4.0, 5.0, 6.0],
            vec![1.0, 5.0, 6.0],
            vec![4.0, 2.0, 6.0],
            vec![1.0, 2.0, 6.0],
            vec![4.0, 5.0, 3.0],
            vec![1.0, 5.0, 3.0],
            vec![4.0, 2.0, 3.0],
            vec![1.0, 2.0, 3.0],
        ];

        assert_eq!(children, expected_children);
    }

    #[test]
    #[should_panic(expected = "Parent genotypes must have the same length")]
    fn test_exhaustive_crossover_different_lengths() {
        let parent1 = vec![1.0, 2.0, 3.0];
        let parent2 = vec![4.0, 5.0];

        exhaustive_crossover(&parent1, &parent2).unwrap();
    }
}