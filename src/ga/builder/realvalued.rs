//! Builder implementation with default values for problems with real valued chromosome

use crate::ga::builder::FitnessFn;
use crate::ga::individual::{IndividualTrait, RealValueIndividual};
use crate::ga::operators::fitness::{Fitness, FnBasedFitness};
use crate::ga::operators::replacement::BothParents;
use crate::ga::{
    operators::{crossover::SinglePoint, mutation::Interchange, selection::Tournament},
    population::RandomPoints,
    probe::StdoutProbe,
};
use crate::ga::{GeneticSolver, Individual};

use super::{DefaultParams, GAConfigOpt};

pub(super) type Rvc = Vec<f64>;

/// [RealValuedBuilder] provides all the operators by default. These can not be modified,
/// altough all the parameters can.
///
/// If more configuration options are desired please see [GenericBuilder](super::generic::GenericBuilder).
pub struct RealValuedBuilder<F: Fitness<RealValueIndividual>> {
    config: GAConfigOpt<
        Individual<Rvc>,
        Interchange<rand::rngs::ThreadRng>,
        SinglePoint<rand::rngs::ThreadRng>,
        Tournament<usize, rand::rngs::ThreadRng>,
        BothParents,
        RandomPoints<rand::rngs::ThreadRng>,
        F,
        StdoutProbe,
    >,
    dim: Option<usize>,
}

impl RealValuedBuilder<FnBasedFitness<RealValueIndividual>> {
    /// Specify fitness function as pointer to a normal function.
    ///
    /// ## Arguments
    ///
    /// * `fitness_fn` - pointer to function with appropriate signature for fitness function
    pub fn fitness_fn(
        self,
        fitness_fn: FitnessFn<Rvc, <RealValueIndividual as IndividualTrait>::FitnessValueT>,
    ) -> Self {
        self.set_fitness(FnBasedFitness::new(fitness_fn))
    }
}

impl<F: Fitness<RealValueIndividual>> RealValuedBuilder<F> {
    /// Returns new instance of [BitStringBuilder]
    pub(super) fn new() -> Self {
        RealValuedBuilder {
            config: GAConfigOpt::new(),
            dim: None,
        }
    }

    /// Sets max duration. If exceeded, the algorithm halts.
    ///
    /// ## Arguments
    ///
    /// * `max_duration` - Maximal duration of computations
    pub fn set_max_duration(mut self, max_duration: std::time::Duration) -> Self {
        self.config.params.max_duration = Some(max_duration);
        self
    }

    /// Sets maximal generation count. If exceeded, the algorithm halts.
    ///
    /// ## Arguments
    ///
    /// * `max_gen_count` - Maximal number of generations to conduct computations for.
    pub fn set_max_generation_count(mut self, max_gen_count: usize) -> Self {
        debug_assert!(max_gen_count >= 1);
        self.config.params.generation_limit = Some(max_gen_count);
        self
    }

    /// Sets population size. Must be positive.
    ///
    /// ## Arguments
    ///
    /// * `size` - Strength of population (number of individuals)
    pub fn set_population_size(mut self, size: usize) -> Self {
        debug_assert!(size > 0);
        self.config.params.population_size = Some(size);
        self
    }

    /// Sets dimension of the problem. Must be > 0.
    ///
    /// ## Arguments
    ///
    /// * `dim` - Problem dimension. Must be > 0.
    pub fn dim(mut self, dim: usize) -> Self {
        assert!(dim > 0, "Dimension of a problem must be > 0");
        self.dim = Some(dim);
        self
    }

    /// Sets fitness
    ///
    /// ## Arguments
    ///
    /// * `fitness` - Structure implementing the Fitness trait. See its documentation for details.
    pub fn set_fitness(mut self, fitness: F) -> Self {
        self.config.fitness_fn = Some(fitness);
        self
    }

    /// If all parameters were configured correctly then it returns ready-to-run instance
    /// of genetic algorithm. Else it panics.
    ///
    /// ## Panics
    ///
    /// Iff:
    ///
    /// * fitness function is not specified
    /// * any of the params has invalid value
    /// * problem dimension is not set
    pub fn build(
        mut self,
    ) -> GeneticSolver<
        Individual<Rvc>,
        Interchange<rand::rngs::ThreadRng>,
        SinglePoint<rand::rngs::ThreadRng>,
        Tournament<usize, rand::rngs::ThreadRng>,
        BothParents,
        RandomPoints<rand::rngs::ThreadRng>,
        F,
        StdoutProbe,
    > {
        self.config.params.fill_from(&Self::DEFAULT_PARAMS);

        if self.config.fitness_fn.is_none() {
            panic!("Fitness function must be set");
        }

        if self.dim.is_none() {
            panic!("Problem dimension must be set");
        }

        self.config
            .crossover_operator
            .get_or_insert_with(SinglePoint::new);
        self.config
            .mutation_operator
            .get_or_insert_with(|| Interchange::new(0.05));
        self.config
            .selection_operator
            .get_or_insert_with(|| Tournament::new(0.2, self.config.params.population_size.unwrap()));
        self.config
            .replacement_operator
            .get_or_insert_with(BothParents::new);
        self.config
            .population_factory
            .get_or_insert_with(|| RandomPoints::new(self.dim.unwrap()));
        self.config.probe.get_or_insert_with(StdoutProbe::new);

        let config = match self.config.try_into() {
            Ok(config) => config,
            Err(err) => panic!("Builder panicked with error: {err}"),
        };

        GeneticSolver::new(config)
    }
}

impl<F: Fitness<RealValueIndividual>> DefaultParams for RealValuedBuilder<F> {}
