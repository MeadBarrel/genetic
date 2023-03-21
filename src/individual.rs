use crate::types::*;

pub struct Individual<G: Genotype, F: Fitness> {
    pub generation: u64,
    pub genome: G,
    pub fitness: Option<F>,
}
