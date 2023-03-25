// Fitness implementations for common types
pub use ordered_float::NotNan;
use crate::types::*;

impl Fitness for NotNan<f64> {}
impl Fitness for NotNan<f32> {}
impl Fitness for usize {}
impl Fitness for i32 {}
impl Fitness for u32 {}
impl Fitness for i64 {}
impl Fitness for u64 {}
impl<F1, F2> Fitness for (F1, F2) 
    where F1: Fitness, F2: Fitness {}
impl<F1, F2, F3> Fitness for (F1, F2, F3) 
    where F1: Fitness, F2: Fitness, F3: Fitness {}
impl<F1, F2, F3, F4> Fitness for (F1, F2, F3, F4) 
    where F1: Fitness, F2: Fitness, F3: Fitness, F4: Fitness {}