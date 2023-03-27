#![feature(drain_filter)]

pub mod error;
pub mod types;
pub mod fitness;
pub mod crossover;
pub mod gabuilder;
pub mod ga;
pub mod individual;
pub mod population;
pub mod reinsert;
pub mod select;
pub mod fitnesses;
pub mod genotypes;

pub mod prelude {
    //pub use super::error::*;
    pub use super::types::*;
    pub use super::fitness::*;
    pub use super::crossover::*;
    pub use super::gabuilder::*;
    pub use super::ga::*;
    pub use super::individual::*;
    pub use super::population::*;
    pub use super::reinsert::*;
    pub use super::select::*;
    pub use super::fitnesses::*;
    pub use super::genotypes::*;
}