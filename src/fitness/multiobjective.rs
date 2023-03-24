use crate::types::*;
use crate::error::*;

impl<F1: Fitness, F2: Fitness> Fitness for (F1, F2) {}

pub struct MultiFitness2<F1, F2, P>
    where
        P: Phenotype,
        F1: FitnessFunction<Phenotype = P>,
        F2: FitnessFunction<Phenotype = P>,
{
    fitness_function1: F1,
    fitness_function2: F2,
}

impl<F1, F2, P> MultiFitness2<F1, F2, P>
    where
        P: Phenotype,
        F1: FitnessFunction<Phenotype = P>,
        F2: FitnessFunction<Phenotype = P>,
{
    pub fn new(fitness_function1: F1, fitness_function2: F2) -> Self {
        Self {
            fitness_function1,
            fitness_function2,
        }
    }
}

impl<F1, F2, P> FitnessFunction for MultiFitness2<F1, F2, P>
    where
        P: Phenotype,
        F1: FitnessFunction<Phenotype = P>,
        F2: FitnessFunction<Phenotype = P>,
{
    type Phenotype = P;
    type Fitness = (F1::Fitness, F2::Fitness);

    fn evaluate(
        &self,
        phenotypes_with_fitnesses: &[(&Self::Phenotype, Option<&Self::Fitness>)],
    ) -> Result<Vec<Self::Fitness>> {
        let phenotypes_with_fitnesses1: Vec<(&Self::Phenotype, Option<&F1::Fitness>)> =
            phenotypes_with_fitnesses
                .iter()
                .map(|(phenotype, fitness)| (*phenotype, fitness.as_ref().map(|f| &f.0)))
                .collect();
    
        let phenotypes_with_fitnesses2: Vec<(&Self::Phenotype, Option<&F2::Fitness>)> =
            phenotypes_with_fitnesses
                .iter()
                .map(|(phenotype, fitness)| (*phenotype, fitness.as_ref().map(|f| &f.1)))
                .collect();
    
        let fitnesses1 = self.fitness_function1.evaluate(&phenotypes_with_fitnesses1)?;
        let fitnesses2 = self.fitness_function2.evaluate(&phenotypes_with_fitnesses2)?;
    
        fitnesses1
            .into_iter()
            .zip(fitnesses2.into_iter())
            .map(|(f1, f2)| Ok((f1, f2)))
            .collect()
    }
}

// use std::marker::PhantomData;
// use std::ops::Mul;
// use rayon::join;

// use crate::types::*;
// use crate::error::Result;

// // impl<F1, F2> Fitness for (F1, F2)
// //     where 
// //         F1: Fitness,
// //         F2: Fitness
// // {}


// // impl<F1, F2, F3> Fitness for (F1, F2, F3)
// //     where 
// //         F1: Fitness,
// //         F2: Fitness,
// //         F3: Fitness,
// // {}


// #[derive(Clone)]
// pub struct MultiObjectiveFitnessValue<P, F> {
//     prev: P,
//     this: F,
// }


// pub struct MultiObjectiveFitness<P, F> {
//     previous: P,
//     this: F,
// }

// impl MultiObjectiveFitness<(), ()> {
//     fn new() -> MultiObjectiveFitness<(), ()> {
//         MultiObjectiveFitness { previous: (), this: () }
//     }
// }

// impl<P, F> MultiObjectiveFitness<P, F> {
//     fn with_fitness<N>(self, func: N) -> MultiObjectiveFitness<F, N> {
//         MultiObjectiveFitness { 
//             previous: self.this,
//             this: func
//         }
//     }
// }

// impl<F: Fitness> Fitness for MultiObjectiveFitnessValue<(), F> {}

// impl<P: Fitness, F: Fitness> Fitness for MultiObjectiveFitnessValue<P, F> {}

// impl<F: Fitness> Ord for MultiObjectiveFitnessValue<(), F> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.this.cmp(&other.this)
//     }
// }

// impl<P: Fitness, F: Fitness> Ord for MultiObjectiveFitnessValue<P, F> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         (self.prev.clone(), self.this.clone()).cmp(&(other.prev.clone(), other.this.clone()))
//     }
// }

// impl<F: Fitness> Eq for MultiObjectiveFitnessValue<(), F> {}
// impl<P: Fitness, F: Fitness> Eq for MultiObjectiveFitnessValue<P, F> {}

// impl<F: Fitness> PartialOrd for MultiObjectiveFitnessValue<(), F> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.this.partial_cmp(&other.this)
//     }
// }

// impl<P: Fitness, F: Fitness> PartialOrd for MultiObjectiveFitnessValue<P, F> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         (self.prev.clone(), self.this.clone()).partial_cmp(&(other.prev.clone(), other.this.clone()))
//     }
// }

// impl<F: Fitness> PartialEq for MultiObjectiveFitnessValue<(), F> {
//     fn eq(&self, other: &Self) -> bool {
//         self.this.eq(&other.this)
//     }
// }

// impl<P: Fitness, F: Fitness> PartialEq for MultiObjectiveFitnessValue<P, F> {
//     fn eq(&self, other: &Self) -> bool {
//         self.this.eq(&other.this) && self.prev.eq(&other.prev)
//     }
// }

// impl<F> FitnessFunction for MultiObjectiveFitness<(), F> 
//     where F: FitnessFunction
// {
//     type Phenotype = F::Phenotype;
//     type Fitness = F::Fitness;

//     fn evaluate(&self, phenotypes_with_fitnesses: &[(&Self::Phenotype, Option<&Self::Fitness>)]) -> Result<Vec<Self::Fitness>> {
//         todo!()
//     }
// }

// impl<P, F> FitnessFunction for MultiObjectiveFitness<P, F> 
//     where
//         P: FitnessFunction,
//         F: for<'a> FitnessFunction<Phenotype = P::Phenotype>
// {
//     type Phenotype = F::Phenotype;
//     type Fitness = MultiObjectiveFitnessValue<P::Fitness, F::Fitness>;

//     fn evaluate(&self, phenotypes_with_fitnesses: &[(&Self::Phenotype, Option<&Self::Fitness>)]) -> Result<Vec<Self::Fitness>> {
//         todo!()
//     }
// }

// // pub struct MultiObjectiveFitness1<F> {
// //     function_1: F,
// // }

// // pub struct MultiObjectiveFitness2<FP, F> {
// //     prev: FP,
// //     function_1: F,
// // }

// // impl MultiObjectiveFitness {
// //     pub fn with_fitness<F>(self, func: F) -> MultiObjectiveFitness1<F> {
// //         MultiObjectiveFitness1 { function_1: func }
// //     }
// // }

// // impl<F> FitnessFunction for MultiObjectiveFitness1<F>
// //     where
// //         F: FitnessFunction
// // {
// //     type Phenotype<'a> = F::Phenotype<'a>;
// //     type Fitness = F::Fitness;

// //     fn evaluate(&self, phenotypes_with_fitnesses: &[(&Self::Phenotype<'_>, Option<&Self::Fitness>)]) -> Result<Vec<Self::Fitness>> {
// //         todo!()
// //     }
// // }



// // pub struct MultiObjectiveFitness {
// // }

// // pub struct MultiObjectiveFitness1<'a, P, F1, F1F> 
// //     where
// //         P: Phenotype<'a>,
// //         F1: Fitness,
// //         F1F: FitnessFunction<Phenotype<'a> = P, Fitness = F1>,
// // {
// //     function_1: F1F,
// //     _phantom: PhantomData<&'a ()>
// // }


// // pub struct MultiObjectiveFitness2<P, F1, F1F, F2, F2F> 
// //     where
// //         P: for <'a> Phenotype<'a>,
// //         F1: Fitness,
// //         F2: Fitness,
// //         F1F: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F1>,
// //         F2F: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F2>
// // {
// //     function_1: F1F,
// //     function_2: F2F,
// // }

// // pub struct MultiObjectiveFitness3<P, F1, F1F, F2, F2F, F3, F3F> 
// //     where
// //         P: for <'a> Phenotype<'a>,
// //         F1: Fitness,
// //         F2: Fitness,
// //         F3: Fitness,
// //         F1F: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F1>,
// //         F2F: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F2>,
// //         F3F: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F3>,
// // {
// //     function_1: F1F,
// //     function_2: F2F,
// //     function_3: F3F,
// // }

// // impl MultiObjectiveFitness
// // {
// //     pub fn new() -> Self {
// //         Self {
// //         }
// //     }

// //     pub fn with_fitness<'a, P, F, FF>(self, func: FF) -> MultiObjectiveFitness1<P, F, FF>
// //         where
// //             P: Phenotype<'a>,
// //             F: Fitness,
// //             FF: FitnessFunction<Phenotype<'a> = P, Fitness = F>,
// //     {
// //         MultiObjectiveFitness1 { function_1: func, _phantom:PhantomData }
// //     }
// // }

// // impl<P, F1, F1F> MultiObjectiveFitness1<P, F1, F1F> 
// //     where
// //         P: for <'a> Phenotype<'a>,
// //         F1: Fitness,
// //         F1F: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F1>,
// // {
// //     pub fn with_fitness<F, FF>(self, func: FF) -> MultiObjectiveFitness2<P, F1, F1F, F, FF>
// //         where
// //             F: Fitness,
// //             FF: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F>
// //     {
// //         MultiObjectiveFitness2 { function_1: self.function_1, function_2: func }
// //     }   
// // }

// // impl<P, F1, F1F, F2, F2F> MultiObjectiveFitness2<P, F1, F1F, F2, F2F>
// //     where
// //         P: for <'a> Phenotype<'a>,
// //         F1: Fitness,
// //         F1F: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F1>,
// //         F2: Fitness,
// //         F2F: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F2>
// // {
// //     pub fn with_fitness<F, FF>(self, func: FF) -> MultiObjectiveFitness3<P, F1, F1F, F2, F2F, F, FF>
// //         where
// //             F: Fitness,
// //             FF: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F>
// //     {
// //         MultiObjectiveFitness3 { 
// //             function_1: self.function_1, 
// //             function_2: self.function_2, 
// //             function_3: func 
// //         }
// //     }

// // }

// // impl<'a, P, F1, F1F> FitnessFunction for MultiObjectiveFitness1<'a, P, F1, F1F> 
// //     where
// //         P: Phenotype,
// //         F1: Fitness,
// //         F1F: FitnessFunction<Phenotype<'a> = P, Fitness = F1>
// // {
// //     type Phenotype<'b> = P;
// //     type Fitness = F1;

// //     fn evaluate(&self, phenotypes_with_fitnesses: &[(&Self::Phenotype<'_>, Option<&Self::Fitness>)]) -> Result<Vec<Self::Fitness>> {
// //         self.function_1.evaluate(phenotypes_with_fitnesses)
// //     }

// //     // fn evaluate<'a, T>(&'a self, fitnesses: T) -> Result<Vec<F1>>
// //     //         where
// //     //             T: Iterator<Item = (&'a Self::Phenotype<'a>, Option<&'a Self::Fitness>)> {
// //     //     self.function_1.evaluate(fitnesses)
// //     // }
// // }

// // impl<P, F1, F1F, F2, F2F> FitnessFunction for MultiObjectiveFitness2<P, F1, F1F, F2, F2F>
// //     where
// //         P: for <'a> Phenotype<'a>,
// //         F1: Fitness,
// //         F1F: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F1>,
// //         F2: Fitness,
// //         F2F: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F2>
// // {
// //     type Phenotype<'a> = P;
// //     type Fitness = (F1, F2);

// //     fn evaluate(&self, phenotypes_with_fitnesses: &[(&Self::Phenotype<'_>, Option<&Self::Fitness>)]) -> Result<Vec<Self::Fitness>> {
// //         let (vecs_1, vecs_2): (Vec<_>, Vec<_>) = phenotypes_with_fitnesses
// //             .into_iter()
// //             .map(|(genome, f)| match f {
// //                 Some(x) => (
// //                         (*genome, Some(&x.0)),
// //                         (*genome, Some(&x.1)),
// //                 ),
// //                 None => ((*genome, None), (*genome, None)),
// //             })
// //             .unzip();

// //         let (fitnesses_1, fitnesses_2) = join(
// //             || self.function_1.evaluate(&vecs_1),
// //             || self.function_2.evaluate(&vecs_2)
// //         );

// //         let fitnesses_1 = fitnesses_1?;
// //         let fitnesses_2 = fitnesses_2?;

// //         let result = fitnesses_1
// //             .into_iter()
// //             .zip(fitnesses_2.into_iter())
// //             .collect();

// //         Ok(result)        
// //     }

// //     // fn evaluate<'a, T>(&'a self, fitnesses: T) -> Result<Vec<Self::Fitness>>
// //     //     where
// //     //         T: Iterator<Item = (&'a Self::Phenotype<'a>, Option<&'a Self::Fitness>)>,
// //     //     {
// //     //         let (vecs_1, vecs_2): (Vec<_>, Vec<_>) = fitnesses
// //     //             .map(|(genome, f)| match f {
// //     //                 Some(x) => (
// //     //                         (genome, Some(&x.0)),
// //     //                         (genome, Some(&x.1)),
// //     //                 ),
// //     //                 None => ((genome, None), (genome, None)),
// //     //             })
// //     //             .unzip();
    
// //     //         let (fitnesses_1, fitnesses_2) = join(
// //     //             || self.function_1.evaluate(vecs_1.into_iter()),
// //     //             || self.function_2.evaluate(vecs_2.into_iter())
// //     //         );

// //     //         let fitnesses_1 = fitnesses_1?;
// //     //         let fitnesses_2 = fitnesses_2?;

// //     //         let result = fitnesses_1
// //     //             .into_iter()
// //     //             .zip(fitnesses_2.into_iter())
// //     //             .collect();

// //     //         Ok(result)
// //     // }

// // }

// // impl<P, F1, F1F, F2, F2F, F3, F3F> FitnessFunction for MultiObjectiveFitness3<P, F1, F1F, F2, F2F, F3, F3F>
// //     where
// //         P: for <'a> Phenotype<'a>,
// //         F1: Fitness,
// //         F1F: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F1>,
// //         F2: Fitness,
// //         F2F: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F2>,
// //         F3: Fitness,
// //         F3F: for<'a> FitnessFunction<Phenotype<'a> = P, Fitness = F3>,
// // {
// //     type Phenotype<'a> = P;
// //     type Fitness = (F1, F2, F3);

// //     fn evaluate(&self, phenotypes_with_fitnesses: &[(&Self::Phenotype<'_>, Option<&Self::Fitness>)]) -> Result<Vec<Self::Fitness>> {
// //         let (vecs_1, vecs_2, vecs_3): (Vec<_>, Vec<_>, Vec<_>) = phenotypes_with_fitnesses
// //             .into_iter()
// //             .fold(
// //                 (Vec::new(), Vec::new(), Vec::new()),
// //                 |(mut v1, mut v2, mut v3), (genome, f)| match f {
// //                     Some(x) => {
// //                         v1.push((*genome, Some(&x.0)));
// //                         v2.push((*genome, Some(&x.1)));
// //                         v3.push((*genome, Some(&x.2)));
// //                         (v1, v2, v3)
// //                     }
// //                     None => {
// //                         v1.push((*genome, None));
// //                         v2.push((*genome, None));
// //                         v3.push((*genome, None));
// //                         (v1, v2, v3)
// //                     }
// //                 },
// //             );

// //         let (
// //             fitnesses_1,
// //             fitnesses_2,
// //         ) = join(
// //             || self.function_1.evaluate(&vecs_1),
// //             || self.function_2.evaluate(&vecs_2)
// //         );
// //         let fitnesses_3 = self.function_3.evaluate(&vecs_3);

// //         let fitnesses_1 = fitnesses_1?;
// //         let fitnesses_2 = fitnesses_2?;
// //         let fitnesses_3 = fitnesses_3?;

// //         let result = fitnesses_1
// //             .into_iter()
// //             .zip(fitnesses_2.into_iter())
// //             .zip(fitnesses_3.into_iter())
// //             .map(|((f1, f2), f3)| (f1, f2, f3))
// //             .collect();

// //         Ok(result)        
// //     }

// //     // fn evaluate<'a, T>(&'a self, fitnesses: T) -> Result<Vec<Self::Fitness>>
// //     // where
// //     //     T: Iterator<Item = (&'a Self::Phenotype<'a>, Option<&'a Self::Fitness>)>,
// //     // {
// //     //     let (vecs_1, vecs_2, vecs_3): (Vec<_>, Vec<_>, Vec<_>) = fitnesses
// //     //         .fold(
// //     //             (Vec::new(), Vec::new(), Vec::new()),
// //     //             |(mut v1, mut v2, mut v3), (genome, f)| match f {
// //     //                 Some(x) => {
// //     //                     v1.push((genome, Some(&x.0)));
// //     //                     v2.push((genome, Some(&x.1)));
// //     //                     v3.push((genome, Some(&x.2)));
// //     //                     (v1, v2, v3)
// //     //                 }
// //     //                 None => {
// //     //                     v1.push((genome, None));
// //     //                     v2.push((genome, None));
// //     //                     v3.push((genome, None));
// //     //                     (v1, v2, v3)
// //     //                 }
// //     //             },
// //     //         );

// //     //     let (
// //     //         fitnesses_1,
// //     //         fitnesses_2,
// //     //     ) = join(
// //     //         || self.function_1.evaluate(vecs_1.into_iter()),
// //     //         || self.function_2.evaluate(vecs_2.into_iter())
// //     //     );
// //     //     let fitnesses_3 = self.function_3.evaluate(vecs_3.into_iter());

// //     //     let fitnesses_1 = fitnesses_1?;
// //     //     let fitnesses_2 = fitnesses_2?;
// //     //     let fitnesses_3 = fitnesses_3?;

// //     //     let result = fitnesses_1
// //     //         .into_iter()
// //     //         .zip(fitnesses_2.into_iter())
// //     //         .zip(fitnesses_3.into_iter())
// //     //         .map(|((f1, f2), f3)| (f1, f2, f3))
// //     //         .collect();

// //     //     Ok(result)
// //     // }
// // }


// // mod temp {
// //     use super::*;
// //     use crate::{types::*, prelude::PrimitiveFitness};
// //     use super::super::SimpleFitnessFunction;

// //     #[derive(Clone)]
// //     struct MyPhenotype<'a>(&'a str);

// //     impl<'a> Phenotype<'a> for MyPhenotype<'a> {}

// //     impl Fitness for usize {}

// //     struct TestFitnessFunction;

// //     impl PrimitiveFitness for TestFitnessFunction {
// //         type Phenotype<'a> = MyPhenotype<'a>;
// //         type Fitness = usize;

// //         fn evaluate_phenotype(&self, phenotype: &Self::Phenotype<'_>) -> Result<Self::Fitness> {
// //             Ok(phenotype.0.len())
// //         }
// //     }

// //     // fn a() {
// //     //     // let fitness = MultiObjectiveFitness::new()
// //     //     //     .with_fitness(func)
// //     //     let fit = SimpleFitnessFunction(TestFitnessFunction);
// //     //     let fitness = MultiObjectiveFitness::new()
// //     //         .with_fitness(SimpleFitnessFunction())
// //     //         .with_fitness(SimpleFitnessFunction(TestFitnessFunction))
// //     //         .with_fitness(SimpleFitnessFunction(TestFitnessFunction))
// //     //         .with_fitness(SimpleFitnessFunction(TestFitnessFunction));

// //     //     let a= fitness.evaluate(&Vec::default()).unwrap();
// //     //     let b= fitness.evaluate(&Vec::default()).unwrap();
// //     //     let i = a > b;
// //     //         // .with_fitness(
// //     //         //     SimpleFitnessFunction(TestFitnessFunction)
// //     //         // );

// //     // }
// // }