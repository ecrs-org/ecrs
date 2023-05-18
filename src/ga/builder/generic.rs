//! Fully configurable builder implementation for any type of chromosome

use crate::ga::builder::FitnessFn;
use crate::ga::individual::IndividualTrait;
use crate::ga::operators::fitness::{Fitness, FnBasedFitness};
use crate::ga::operators::replacement::ReplacementOperator;
use crate::ga::{
    operators::{crossover::CrossoverOperator, mutation::MutationOperator, selection::SelectionOperator},
    population::PopulationGenerator,
    GeneticSolver, Probe,
};

use super::{DefaultParams, GAConfigOpt};

/// [GenericBuilder] must be fully configured. Only defaults for params are provided,
/// altough it is preffered to override them.
pub struct GenericBuilder<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>
where
    IndividualT: IndividualTrait,
    MutOpT: MutationOperator<IndividualT::ChromosomeT>,
    CrossOpT: CrossoverOperator<IndividualT::ChromosomeT>,
    SelOpT: SelectionOperator<IndividualT::ChromosomeT>,
    ReplOpT: ReplacementOperator<IndividualT::ChromosomeT>,
    PopGenT: PopulationGenerator<IndividualT::ChromosomeT>,
    FitnessT: Fitness<IndividualT::ChromosomeT>,
    ProbeT: Probe<IndividualT::ChromosomeT>,
{
    config: GAConfigOpt<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>,
}

impl<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, ProbeT>
    GenericBuilder<
        IndividualT,
        MutOpT,
        CrossOpT,
        SelOpT,
        ReplOpT,
        PopGenT,
        FnBasedFitness<IndividualT::ChromosomeT>,
        ProbeT,
    >
where
    IndividualT: IndividualTrait,
    MutOpT: MutationOperator<IndividualT::ChromosomeT>,
    CrossOpT: CrossoverOperator<IndividualT::ChromosomeT>,
    SelOpT: SelectionOperator<IndividualT::ChromosomeT>,
    ReplOpT: ReplacementOperator<IndividualT::ChromosomeT>,
    PopGenT: PopulationGenerator<IndividualT::ChromosomeT>,
    ProbeT: Probe<IndividualT::ChromosomeT>,
{
    pub fn set_fitness_fn(self, fitness_fn: FitnessFn<IndividualT::ChromosomeT>) -> Self {
        self.set_fitness(FnBasedFitness::new(fitness_fn))
    }
}

impl<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>
    GenericBuilder<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>
where
    IndividualT: IndividualTrait,
    MutOpT: MutationOperator<IndividualT::ChromosomeT>,
    CrossOpT: CrossoverOperator<IndividualT::ChromosomeT>,
    SelOpT: SelectionOperator<IndividualT::ChromosomeT>,
    PopGenT: PopulationGenerator<IndividualT::ChromosomeT>,
    ReplOpT: ReplacementOperator<IndividualT::ChromosomeT>,
    FitnessT: Fitness<IndividualT::ChromosomeT>,
    ProbeT: Probe<IndividualT::ChromosomeT>,
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
    pub fn set_fitness(mut self, fitness: FitnessT) -> Self {
        self.config.fitness_fn = Some(fitness);
        self
    }

    /// Sets mutation operator
    ///
    /// ## Arguments
    ///
    /// * `mutation_op` - struct implementing [MutationOperator](crate::ga::operators::mutation::MutationOperator) trait
    pub fn set_mutation_operator(mut self, mutation_op: MutOpT) -> Self {
        self.config.mutation_operator = Some(mutation_op);
        self
    }

    /// Sets crossover operator
    ///
    /// ## Arguments
    ///
    /// * `crossover_op` - struct implementing [CrossoverOperator](crate::ga::operators::crossover::CrossoverOperator) trait
    pub fn set_crossover_operator(mut self, crossover_op: CrossOpT) -> Self {
        self.config.crossover_operator = Some(crossover_op);
        self
    }

    /// Sets selection operator
    ///
    /// ## Arguments
    ///
    /// * `selection_op` - struct implementing [SelectionOperator](crate::ga::operators::selection::SelectionOperator) trait
    pub fn set_selection_operator(mut self, selection_op: SelOpT) -> Self {
        self.config.selection_operator = Some(selection_op);
        self
    }

    /// Sets replacement operator
    ///
    /// ## Arguments
    ///
    /// * `replacement_op` - struct implementing [ReplacementOperator](crate::ga::operators::replacement::ReplacementOperator) trait
    pub fn set_replacement_operator(mut self, replacement_op: ReplOpT) -> Self {
        self.config.replacement_operator = Some(replacement_op);
        self
    }

    /// Sets population generator
    ///
    /// ## Arguments
    ///
    /// * `generator` - struct implementing [PopulationGenerator](crate::ga::population::PopulationGenerator) trait
    pub fn set_population_generator(mut self, generator: PopGenT) -> Self {
        self.config.population_factory = Some(generator);
        self
    }

    /// Sets probe
    ///
    /// ## Arguments
    ///
    /// * `probe` - struct implementing [Probe](crate::ga::probe::Probe) trait
    pub fn set_probe(mut self, probe: ProbeT) -> Self {
        self.config.probe = Some(probe);
        self
    }

    /// Returns new instance of [GeneticAlgorithm](crate::ga::GeneticAlgorithm)
    ///
    /// ## Panics
    ///
    /// Iff any of the parameters has invalid value.
    pub fn build(
        mut self,
    ) -> GeneticSolver<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT> {
        self.config.params.fill_from(&Self::DEFAULT_PARAMS);

        let config = match self.config.try_into() {
            Ok(config) => config,
            Err(err) => panic!("Builder panicked with error: {err}"),
        };

        GeneticSolver::new(config)
    }
}

impl<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT> DefaultParams
    for GenericBuilder<IndividualT, MutOpT, CrossOpT, SelOpT, ReplOpT, PopGenT, FitnessT, ProbeT>
where
    IndividualT: IndividualTrait,
    MutOpT: MutationOperator<IndividualT::ChromosomeT>,
    CrossOpT: CrossoverOperator<IndividualT::ChromosomeT>,
    SelOpT: SelectionOperator<IndividualT::ChromosomeT>,
    PopGenT: PopulationGenerator<IndividualT::ChromosomeT>,
    ReplOpT: ReplacementOperator<IndividualT::ChromosomeT>,
    FitnessT: Fitness<IndividualT::ChromosomeT>,
    ProbeT: Probe<IndividualT::ChromosomeT>,
{
}
