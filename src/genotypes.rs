use crate::types::*;

impl<T> Genotype for Vec<T> where T: Clone + Send + Sync {}
impl<T> Phenotype for Vec<T> where T: Clone + Send + Sync {}