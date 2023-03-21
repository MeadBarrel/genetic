use std::marker::PhantomData;
use rayon::join;

use crate::types::*;
use crate::error::Result;

impl<F1, F2> Fitness for (F1, F2)
    where 
        F1: Fitness,
        F2: Fitness
{}


impl<F1, F2, F3> Fitness for (F1, F2, F3)
    where 
        F1: Fitness,
        F2: Fitness,
        F3: Fitness,
{}


pub struct MultiObjectiveFitness<P: Phenotype> {
    phenotype: PhantomData<P>
}

pub struct MultiObjectiveFitness1<P, F1, F1F> 
    where
        P: Phenotype,
        F1: Fitness,
        F1F: FitnessFunction<Phenotype = P, Fitness = F1>,
{
    function_1: F1F,
}


pub struct MultiObjectiveFitness2<P, F1, F1F, F2, F2F> 
    where
        P: Phenotype,
        F1: Fitness,
        F2: Fitness,
        F1F: FitnessFunction<Phenotype = P, Fitness = F1>,
        F2F: FitnessFunction<Phenotype = P, Fitness = F2>
{
    function_1: F1F,
    function_2: F2F,
}

pub struct MultiObjectiveFitness3<P, F1, F1F, F2, F2F, F3, F3F> 
    where
        P: Phenotype,
        F1: Fitness,
        F2: Fitness,
        F3: Fitness,
        F1F: FitnessFunction<Phenotype = P, Fitness = F1>,
        F2F: FitnessFunction<Phenotype = P, Fitness = F2>,
        F3F: FitnessFunction<Phenotype = P, Fitness = F3>,
{
    function_1: F1F,
    function_2: F2F,
    function_3: F3F,
}

impl<P> MultiObjectiveFitness<P>
    where P: Phenotype
{
    pub fn new() -> Self {
        Self {
            phenotype: PhantomData
        }
    }

    pub fn with_fitness<F, FF>(self, func: FF) -> MultiObjectiveFitness1<P, F, FF>
        where
            F: Fitness,
            FF: FitnessFunction<Phenotype = P, Fitness = F>
    {
        MultiObjectiveFitness1 { function_1: func }
    }
}

impl<P, F1, F1F> MultiObjectiveFitness1<P, F1, F1F> 
    where
        P: Phenotype,
        F1: Fitness,
        F1F: FitnessFunction<Phenotype = P, Fitness = F1>,
{
    pub fn with_fitness<F, FF>(self, func: FF) -> MultiObjectiveFitness2<P, F1, F1F, F, FF>
        where
            F: Fitness,
            FF: FitnessFunction<Phenotype = P, Fitness = F>
    {
        MultiObjectiveFitness2 { function_1: self.function_1, function_2: func }
    }   
}

impl<P, F1, F1F, F2, F2F> MultiObjectiveFitness2<P, F1, F1F, F2, F2F>
    where
        P: Phenotype,
        F1: Fitness,
        F1F: FitnessFunction<Phenotype = P, Fitness = F1>,
        F2: Fitness,
        F2F: FitnessFunction<Phenotype = P, Fitness = F2>
{
    pub fn with_fitness<F, FF>(self, func: FF) -> MultiObjectiveFitness3<P, F1, F1F, F2, F2F, F, FF>
        where
            F: Fitness,
            FF: FitnessFunction<Phenotype = P, Fitness = F>
    {
        MultiObjectiveFitness3 { 
            function_1: self.function_1, 
            function_2: self.function_2, 
            function_3: func 
        }
    }

}

impl<P, F1, F1F> FitnessFunction for MultiObjectiveFitness1<P, F1, F1F> 
    where
        P: Phenotype,
        F1: Fitness,
        F1F: FitnessFunction<Phenotype = P, Fitness = F1>
{
    type Phenotype = P;
    type Fitness = F1;

    fn evaluate<'a, T>(&'a self, fitnesses: T) -> Result<Vec<F1>>
            where
                T: Iterator<Item = (&'a Self::Phenotype, Option<&'a Self::Fitness>)> {
        self.function_1.evaluate(fitnesses)
    }
}

impl<P, F1, F1F, F2, F2F> FitnessFunction for MultiObjectiveFitness2<P, F1, F1F, F2, F2F>
    where
        P: Phenotype,
        F1: Fitness,
        F1F: FitnessFunction<Phenotype = P, Fitness = F1>,
        F2: Fitness,
        F2F: FitnessFunction<Phenotype = P, Fitness = F2>
{
    type Phenotype = P;
    type Fitness = (F1, F2);

    fn evaluate<'a, T>(&'a self, fitnesses: T) -> Result<Vec<Self::Fitness>>
        where
            T: Iterator<Item = (&'a Self::Phenotype, Option<&'a Self::Fitness>)>,
        {
            let (vecs_1, vecs_2): (Vec<_>, Vec<_>) = fitnesses
                .map(|(genome, f)| match f {
                    Some(x) => (
                            (genome, Some(&x.0)),
                            (genome, Some(&x.1)),
                    ),
                    None => ((genome, None), (genome, None)),
                })
                .unzip();
    
            let (fitnesses_1, fitnesses_2) = join(
                || self.function_1.evaluate(vecs_1.into_iter()),
                || self.function_2.evaluate(vecs_2.into_iter())
            );

            let fitnesses_1 = fitnesses_1?;
            let fitnesses_2 = fitnesses_2?;

            let result = fitnesses_1
                .into_iter()
                .zip(fitnesses_2.into_iter())
                .collect();

            Ok(result)
    }

}

impl<P, F1, F1F, F2, F2F, F3, F3F> FitnessFunction for MultiObjectiveFitness3<P, F1, F1F, F2, F2F, F3, F3F>
    where
        P: Phenotype,
        F1: Fitness,
        F1F: FitnessFunction<Phenotype = P, Fitness = F1>,
        F2: Fitness,
        F2F: FitnessFunction<Phenotype = P, Fitness = F2>,
        F3: Fitness,
        F3F: FitnessFunction<Phenotype = P, Fitness = F3>,
{
    type Phenotype = P;
    type Fitness = (F1, F2, F3);

    fn evaluate<'a, T>(&'a self, fitnesses: T) -> Result<Vec<Self::Fitness>>
    where
        T: Iterator<Item = (&'a Self::Phenotype, Option<&'a Self::Fitness>)>,
    {
        let (vecs_1, vecs_2, vecs_3): (Vec<_>, Vec<_>, Vec<_>) = fitnesses
            .fold(
                (Vec::new(), Vec::new(), Vec::new()),
                |(mut v1, mut v2, mut v3), (genome, f)| match f {
                    Some(x) => {
                        v1.push((genome, Some(&x.0)));
                        v2.push((genome, Some(&x.1)));
                        v3.push((genome, Some(&x.2)));
                        (v1, v2, v3)
                    }
                    None => {
                        v1.push((genome, None));
                        v2.push((genome, None));
                        v3.push((genome, None));
                        (v1, v2, v3)
                    }
                },
            );

        let (
            fitnesses_1,
            fitnesses_2,
        ) = join(
            || self.function_1.evaluate(vecs_1.into_iter()),
            || self.function_2.evaluate(vecs_2.into_iter())
        );
        let fitnesses_3 = self.function_3.evaluate(vecs_3.into_iter());

        let fitnesses_1 = fitnesses_1?;
        let fitnesses_2 = fitnesses_2?;
        let fitnesses_3 = fitnesses_3?;

        let result = fitnesses_1
            .into_iter()
            .zip(fitnesses_2.into_iter())
            .zip(fitnesses_3.into_iter())
            .map(|((f1, f2), f3)| (f1, f2, f3))
            .collect();

        Ok(result)
    }
}
