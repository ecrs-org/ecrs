//! Fully configurable builder implementation for any type of chromosome

use crate::ga::builder::FitnessFn;
use crate::ga::operators::fitness::{Fitness, FnBasedFitness};
use crate::ga::operators::replacement::ReplacementOperator;
use crate::ga::{
    individual::Chromosome,
    operators::{crossover::CrossoverOperator, mutation::MutationOperator, selection::SelectionOperator},
    population::PopulationGenerator,
    GeneticSolver, Probe,
};

use super::{DefaultParams, GAConfigOpt};

/// [GenericBuilder] must be fully configured. Only defaults for params are provided,
/// altough it is preffered to override them.
pub struct GenericBuilder<T, M, C, S, R, P, F, Pr>
where
    T: Chromosome,
    M: MutationOperator<T>,
    C: CrossoverOperator<T>,
    S: SelectionOperator<T>,
    R: ReplacementOperator<T>,
    P: PopulationGenerator<T>,
    F: Fitness<T>,
    Pr: Probe<T>,
{
    config: GAConfigOpt<T, M, C, S, R, P, F, Pr>,
}

impl<T, M, C, S, R, P, Pr> GenericBuilder<T, M, C, S, R, P, FnBasedFitness<T>, Pr>
where
    T: Chromosome,
    M: MutationOperator<T>,
    C: CrossoverOperator<T>,
    S: SelectionOperator<T>,
    R: ReplacementOperator<T>,
    P: PopulationGenerator<T>,
    Pr: Probe<T>,
{
    pub fn set_fitness_fn(self, fitness_fn: FitnessFn<T>) -> Self {
        self.set_fitness(FnBasedFitness::new(fitness_fn))
    }
}

impl<T, M, C, S, R, P, F, Pr> GenericBuilder<T, M, C, S, R, P, F, Pr>
where
    T: Chromosome,
    M: MutationOperator<T>,
    C: CrossoverOperator<T>,
    S: SelectionOperator<T>,
    P: PopulationGenerator<T>,
    R: ReplacementOperator<T>,
    F: Fitness<T>,
    Pr: Probe<T>,
{
    /// Returns new instance of [GenericBuilder]
    pub fn new() -> Self {
        GenericBuilder {
            config: GAConfigOpt::new(),
        }
    }

    /// Sets selection rate
    ///
    /// ## Arguments
    ///
    /// * `selection_rate` - Selection rate; must be in [0, 1] interval
    ///
    /// ## Panics
    ///
    /// If the `selection_rate` param has invalid value.
    pub fn set_selection_rate(mut self, selection_rate: f64) -> Self {
        assert!((0f64..=1f64).contains(&selection_rate));
        self.config.params.selection_rate = Some(selection_rate);
        self
    }

    /// Sets mutation rate
    ///
    /// ## Arguments
    ///
    /// * `mutation_rate` - Mutation rate; must be in [0, 1] interval
    ///
    /// ## Panics
    ///
    /// If the parameter has invalid value.
    pub fn set_mutation_rate(mut self, mutation_rate: f64) -> Self {
        assert!((0.0..=1.0).contains(&mutation_rate));
        self.config.params.mutation_rate = Some(mutation_rate);
        self
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
    ///
    /// ## Panics
    ///
    /// If the parameter has invalid value.
    pub fn set_max_generation_count(mut self, max_gen_count: usize) -> Self {
        assert!(max_gen_count >= 1);
        self.config.params.generation_limit = Some(max_gen_count);
        self
    }

    /// Sets population size. Must be positive.
    ///
    /// ## Arguments
    ///
    /// * `size` - Strength of population (number of individuals)
    ///
    /// ## Panics
    ///
    /// If the parameter has invalid value.
    pub fn set_population_size(mut self, size: usize) -> Self {
        assert!(size > 0);
        self.config.params.population_size = Some(size);
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

    /// Sets mutation operator
    ///
    /// ## Arguments
    ///
    /// * `mutation_op` - struct implementing [MutationOperator](crate::ga::operators::mutation::MutationOperator) trait
    pub fn set_mutation_operator(mut self, mutation_op: M) -> Self {
        self.config.mutation_operator = Some(mutation_op);
        self
    }

    /// Sets crossover operator
    ///
    /// ## Arguments
    ///
    /// * `crossover_op` - struct implementing [CrossoverOperator](crate::ga::operators::crossover::CrossoverOperator) trait
    pub fn set_crossover_operator(mut self, crossover_op: C) -> Self {
        self.config.crossover_operator = Some(crossover_op);
        self
    }

    /// Sets selection operator
    ///
    /// ## Arguments
    ///
    /// * `selection_op` - struct implementing [SelectionOperator](crate::ga::operators::selection::SelectionOperator) trait
    pub fn set_selection_operator(mut self, selection_op: S) -> Self {
        self.config.selection_operator = Some(selection_op);
        self
    }

    /// Sets replacement operator
    ///
    /// ## Arguments
    ///
    /// * `replacement_op` - struct implementing [ReplacementOperator](crate::ga::operators::replacement::ReplacementOperator) trait
    pub fn set_replacement_operator(mut self, replacement_op: R) -> Self {
        self.config.replacement_operator = Some(replacement_op);
        self
    }

    /// Sets population generator
    ///
    /// ## Arguments
    ///
    /// * `generator` - struct implementing [PopulationGenerator](crate::ga::population::PopulationGenerator) trait
    pub fn set_population_generator(mut self, generator: P) -> Self {
        self.config.population_factory = Some(generator);
        self
    }

    /// Sets probe
    ///
    /// ## Arguments
    ///
    /// * `probe` - struct implementing [Probe](crate::ga::probe::Probe) trait
    pub fn set_probe(mut self, probe: Pr) -> Self {
        self.config.probe = Some(probe);
        self
    }

    /// Returns new instance of [GeneticAlgorithm](crate::ga::GeneticAlgorithm)
    ///
    /// ## Panics
    ///
    /// Iff any of the parameters has invalid value.
    pub fn build(mut self) -> GeneticSolver<T, M, C, S, R, P, F, Pr> {
        self.config.params.fill_from(&Self::DEFAULT_PARAMS);

        let config = match self.config.try_into() {
            Ok(config) => config,
            Err(err) => panic!("Builder panicked with error: {err}"),
        };

        GeneticSolver::new(config)
    }
}

impl<T, M, C, S, R, P, F, Pr> DefaultParams for GenericBuilder<T, M, C, S, R, P, F, Pr>
where
    T: Chromosome,
    M: MutationOperator<T>,
    C: CrossoverOperator<T>,
    S: SelectionOperator<T>,
    P: PopulationGenerator<T>,
    R: ReplacementOperator<T>,
    F: Fitness<T>,
    Pr: Probe<T>,
{
}
