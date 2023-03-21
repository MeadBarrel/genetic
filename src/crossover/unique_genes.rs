use std::marker::PhantomData;
use serde::Deserialize;

use rand::Rng;
use rand::seq::IteratorRandom;

use crate::types::*;
use crate::error::*;

#[derive(Deserialize)]
#[serde(default)]
pub struct UniquenessPreservativeCrossoverBuilder<R> {
    pub num_children: usize,
    pub rng: R,
}

impl Default for UniquenessPreservativeCrossoverBuilder<()> {
    fn default() -> Self {
        Self {
            num_children: 2,
            rng: ()
        }
    }
}

impl<R> UniquenessPreservativeCrossoverBuilder<R> {
    pub fn with_num_children(mut self, num_children: usize) -> Self {
        self.num_children = num_children;
        self
    }

    pub fn with_rng<RN: Rng>(mut self, rng: RN) -> UniquenessPreservativeCrossoverBuilder<RN> {
        UniquenessPreservativeCrossoverBuilder {
            num_children: self.num_children,
            rng
        }
    }
}

impl<R> UniquenessPreservativeCrossoverBuilder<R> 
    where R: Rng
{
    pub fn build<G>(self) -> UniquenessPreservativeCrossover<R, G> 
        where G: AsRef<usize> + Clone + Send + Sync
    {
        UniquenessPreservativeCrossover { 
            num_children: self.num_children, 
            rng: self.rng, 
            gene: PhantomData 
        }
    }
}

pub struct UniquenessPreservativeCrossover<R, G> 
    where R: Rng, G: Clone + Send + Sync
{
    pub num_children: usize,
    pub rng: R,
    pub gene: PhantomData<G>
}


impl<R, G> CrossoverOperator for UniquenessPreservativeCrossover<R, G> 
    where R: Rng, G: AsRef<usize> + Clone + Send + Sync
{
    type Genotype = VectorEncoded<G>;

    fn crossover(&mut self, genomes: &[&Self::Genotype]) -> Result<Vec<Self::Genotype>> {
        let genome_length = genomes[0].len();
        let combined = genomes.into_iter()
            .map(|x| x.into_iter().cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .concat();
        let max_index: usize = combined.iter().map(|x| *x.as_ref()).max().unwrap();

        let result = (0..self.num_children)
            .map(|_| {
                let mut gene_pool = combined.clone();
                let mut index_map = vec![false; max_index + 1];
                let mut child = Vec::default();
                while child.len() < genome_length {
                    let selected_index = (0..gene_pool.len()).choose(&mut self.rng).unwrap();
                    let selected_gene = gene_pool.remove(selected_index);
                    let selected_gene_usize = *selected_gene.as_ref();
                    if index_map[selected_gene_usize] { continue; }
                    child.push(selected_gene.clone());
                    index_map[selected_gene_usize] = true;
                }

                child
            })
            .collect();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;
    use std::collections::HashSet;

    #[derive(Clone, Debug)]
    struct TestGene(usize);
    impl Genotype for TestGene {}

    impl AsRef<usize> for TestGene {
        fn as_ref(&self) -> &usize {
            &self.0
        }
    }

    type TestCrossover = UniquenessPreservativeCrossover<StdRng, TestGene>;

    fn test_crossover_instance(num_children: usize) -> TestCrossover {
        let rng = StdRng::from_entropy();
        UniquenessPreservativeCrossoverBuilder::default()
            .with_num_children(num_children)
            .with_rng(rng)
            .build::<TestGene>()
    }

    fn unique_random_genes(size: usize, max_value: usize) -> Vec<TestGene> {
        let mut rng = rand::thread_rng();
        let mut gene_set = HashSet::new();
        while gene_set.len() < size {
            gene_set.insert(rng.gen_range(0..max_value));
        }
        gene_set.into_iter().map(TestGene).collect()
    }

    proptest! {
        #[test]
        fn test_crossover_genome_properties(
            num_children in 1usize..10,
            genome_size in 1usize..50,
            num_genomes in 2usize..10,
            max_gene_value in 50usize..51
        ) {
            // Generate genomes with unique and random genes within each parent
            let genomes = (0..num_genomes)
                .map(|_| {
                    unique_random_genes(genome_size, max_gene_value).into()
                })
                .collect::<Vec<VectorEncoded<TestGene>>>();

            let genomes_refs: Vec<_> = genomes.iter().collect();

            let mut crossover = test_crossover_instance(num_children);

            let children = crossover.crossover(&genomes_refs).unwrap();

            // Test: number of children
            prop_assert_eq!(children.len(), num_children);

            // Test: children genome length
            for child in &children {
                prop_assert_eq!(child.len(), genome_size);
            }

            // Test: unique genes
            for child in children {
                let mut gene_set = std::collections::HashSet::new();
                for gene in child {
                    prop_assert!(gene_set.insert(*gene.as_ref()));
                }
            }
        }
    }
}