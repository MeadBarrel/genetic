use criterion::{criterion_group, criterion_main, Criterion};
use genetic::crossover::UniquenessPreservativeCrossover;
use genetic::types::{VectorEncoded, CrossoverOperator};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[derive(Clone, Debug, Eq, PartialEq)]
struct TestGene(usize);


impl AsRef<usize> for TestGene {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

fn unique_deterministic_genes(size: usize, max_value: usize, seed: [u8; 32]) -> Vec<TestGene> {
    let mut rng = SmallRng::from_seed(seed);
    let mut gene_set = std::collections::HashSet::new();
    while gene_set.len() < size {
        gene_set.insert(rng.gen_range(0..max_value));
    }
    gene_set.into_iter().map(TestGene).collect()
}

fn crossover_benchmark(c: &mut Criterion) {
    let num_children = 10;
    let genome_size = 16;
    let num_genomes = 5;
    let max_gene_value = 30;

    let genomes = (0..num_genomes)
        .map(|i| unique_deterministic_genes(genome_size, max_gene_value, [i as u8 + 1; 32]).into())
        .collect::<Vec<VectorEncoded<TestGene>>>();

    let mut crossover = UniquenessPreservativeCrossover {
        num_children,
        rng: SmallRng::from_seed([0; 32]),
        gene: std::marker::PhantomData,
    };

    let genomes_vec: Vec<_> = genomes.iter().collect();

    c.bench_function("crossover", |b| {
        b.iter(|| crossover.crossover(&genomes_vec).unwrap())
    });
}

criterion_group!(benches, crossover_benchmark);
criterion_main!(benches);