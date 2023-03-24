use std::marker::PhantomData;

use crate::population::*;
use crate::types::*;
use crate::ga::*;
use crate::error::*;

pub struct GeneticAlgorithmBuilder<I, F, S, C, M, R>
{
    pub incubator: I,
    pub fitness_function: F,
    pub select: S,
    pub crossover: C,
    pub mutate: M,
    pub reinsert: R,
    _phantom: PhantomData<()>
}

impl GeneticAlgorithmBuilder<(), (), (), (), (), ()> {

    pub fn new() -> Self {
        Self { incubator: (), fitness_function: (), select: (), crossover: (), mutate: (), reinsert: (), _phantom: PhantomData }
    }

    pub fn with_incubator<I>(self, incubator: I) -> GeneticAlgorithmBuilder<I, (), (), (), (), ()> {
        GeneticAlgorithmBuilder {
            incubator,
            fitness_function: (),
            select: (),
            crossover: (),
            mutate: (),
            reinsert: (),
            _phantom: PhantomData,
        }
    }
}

impl<I> GeneticAlgorithmBuilder<I, (), (), (), (), ()> 
    where 
        I: Incubator
{
    fn with_fitness_function<F>(self, fitness_function: F) -> GeneticAlgorithmBuilder<I, F, (), (), (), ()> {
        GeneticAlgorithmBuilder {
            incubator: self.incubator,
            fitness_function,
            select: (),
            crossover: (),
            mutate: (),
            reinsert: (),
            _phantom: PhantomData,
        }
    }
}

impl<I, F, S, C, M, R> GeneticAlgorithmBuilder<I, F, S, C, M, R> 
{

    pub fn with_select<SNEW>(self, select: SNEW) -> GeneticAlgorithmBuilder<I, F, SNEW, C, M, R> {
        GeneticAlgorithmBuilder {
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select,
            crossover: self.crossover,
            mutate: self.mutate,
            reinsert: self.reinsert,
            _phantom: PhantomData,
        }
    }

    pub fn with_crossover<CNEW>(self, crossover: CNEW) -> GeneticAlgorithmBuilder<I, F, S, CNEW, M, R> {
        GeneticAlgorithmBuilder {
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select: self.select,
            crossover,
            mutate: self.mutate,
            reinsert: self.reinsert,
            _phantom: PhantomData,
        }        
    }

    pub fn with_mutate<MNEW>(self, mutate: MNEW) -> GeneticAlgorithmBuilder<I, F, S, C, MNEW, R> {
        GeneticAlgorithmBuilder {
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select: self.select,
            crossover: self.crossover,
            mutate,
            reinsert: self.reinsert,
            _phantom: PhantomData,
        }        
    }

    pub fn with_reinsert<RNEW>(self, reinsert: RNEW) -> GeneticAlgorithmBuilder<I, F, S, C, M, RNEW> {
        GeneticAlgorithmBuilder {
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select: self.select,
            crossover: self.crossover,
            mutate: self.mutate,
            reinsert,
            _phantom: PhantomData,
        }        
    }
}

impl<'a, P, I, F, S, C, M, R> GeneticAlgorithmBuilder<I, F, S, C, M, R>
    where
        I: Incubator<Phenotype<'a> = P> + 'a,
        P: Phenotype,
        F: FitnessFunction<Phenotype = P>,
        S: SelectOperator,
        C: CrossoverOperator<Genotype = I::Genotype>,
        M: MutateOperator<Genotype = I::Genotype>,
        R: ReinsertOperator
{
    fn create_population(&'a self, genomes: Vec<I::Genotype>) -> Result<SortedPopulation<I::Genotype, F::Fitness>> 
    {
        Population::new()
            .add_children(genomes)
            .sort(&self.incubator, &self.fitness_function)
    }

    fn build(self) -> GeneticAlgorithm<P, I, F, S, C, M, R> {
        GeneticAlgorithm {
            incubator: self.incubator,
            fitness_function: self.fitness_function,
            select: self.select,
            crossover: self.crossover,
            mutate: self.mutate,
            reinsert: self.reinsert,
            _phantom: PhantomData
        }
    }
}


pub mod testing {
    use super::*;
    use crate::{types::*, prelude::{SimpleFitnessFunction, ElitistReinserter, MultiFitness2, ParetoFitnessFunction, SimpleFitness}};

    #[derive(Debug, Clone)]
    pub struct MyGenotype(usize);
    #[derive(Debug, Clone)]
    pub struct MyPhenotype<'a> (&'a str);
    impl<'a> Phenotype for MyPhenotype<'a> {}


    impl Genotype for MyGenotype {}

    #[derive(Clone)]
    pub struct MyIncubator;

    impl Incubator for MyIncubator {
        type Genotype = MyGenotype;
        type Phenotype<'a> = MyPhenotype<'a> where Self: 'a;

        fn grow<'a: 'b, 'b>(&'a self, genome: &Self::Genotype) -> crate::prelude::Result<Self::Phenotype<'b>> {
            todo!()
        }
    }

    pub struct MyMutator;

    impl MutateOperator for MyMutator {
        type Genotype = MyGenotype;

        fn mutate(&mut self, genome: &mut Self::Genotype) -> crate::prelude::Result<()> {
            todo!()
        }
    }

    pub struct MyCrossover;

    impl CrossoverOperator for MyCrossover {
        type Genotype = MyGenotype;

        fn crossover(&mut self, genomes: &[&Self::Genotype]) -> Result<Vec<Self::Genotype>> {
            todo!()
        }
    }

    pub struct MySelect;

    impl SelectOperator for MySelect {
        fn select<'a, G, F>(&mut self, population: &'a SortedPopulation<G, F>) -> Result<Vec<Vec<&'a G>>>
                where
                    G: Genotype,
                    F: Fitness, {
            todo!()
        }
    }

    // pub struct ABC<I, F> 
    // {
    //     incubator: I,
    //     fitness: F,
    // }

    // impl<I, F> ABC<I, F>
    //     where 
    //         I: Incubator,
    //         F: for<'b> FitnessFunction<Phenotype = I::Phenotype<'b>>
    // {
    //     pub fn ok(&self) { todo!() }
    // }

    impl Fitness for usize {}

    fn run() {
        // let fitness = MultiObjectiveFitness::new()
        //     .with_fitness(SimpleFitnessFunction::new(|x: &MyPhenotype| Ok(1usize)))
        //     .with_fitness(SimpleFitnessFunction::new(|x: &MyPhenotype| Ok(2usize)));
        //let fitness = SimpleFitnessFunction::new(|x: &MyPhenotype| Ok(1usize));
        let fitness1 = SimpleFitness::new(|x: &MyPhenotype| Ok(1usize)).use_existing_fitness();
        let fitness2 = SimpleFitness::new(|x: &MyPhenotype| Ok(1usize)).use_existing_fitness();
        let fitness = MultiFitness2::new(fitness1, fitness2);
        // let fitness3 = ParetoFitnessFunction::new()
        //     .with_objective(Box::new(|x: &MyPhenotype| Ok(0.5)))
        //     .with_objective(Box::new(|x: &MyPhenotype| Ok(0.5)));
        // let fitness = MultiObjectiveFitness::new()
        //     .with_fitness(fitness1)
        //     .with_fitness(fitness2)
        //     .with_fitness(fitness3);

        let builder = GeneticAlgorithmBuilder::new()
            .with_incubator(MyIncubator)
            .with_fitness_function(fitness)
            .with_crossover(MyCrossover)
            .with_mutate(MyMutator)
            .with_reinsert(ElitistReinserter)
            .with_select(MySelect);

        let a = builder.create_population(Vec::default());
        //let c = builder.incubator;

        //builder.build();

        // let abc = ABC {
        //     incubator: MyIncubator,
        //     fitness
        // };
        // abc.ok()
    }


}