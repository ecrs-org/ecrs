pub mod builder;
pub mod individual;
pub mod operators;
pub mod population;
pub mod probe;

pub use builder::*;
pub use individual::Individual;
pub use probe::CsvProbe;
pub use probe::JsonProbe;
pub use probe::Probe;
pub use probe::StdoutProbe;

use self::{
  individual::Chromosome,
  operators::{crossover::CrossoverOperator, mutation::MutationOperator, selection::SelectionOperator},
  population::PopulationGenerator,
};

type FitnessFn<S> = fn(&S) -> f64;

/// Parameters for the genetic algorithm
pub struct GAParams {
  /// **Unused for now**
  pub selection_rate: f64,

  /// Describes chance of a individual gene to be mutated
  ///
  /// This parameter is passed down to a mutation operator
  ///
  /// Must be in range [0, 1]
  pub mutation_rate: f64,

  /// Number of individuals in population
  ///
  /// In current implementation of the algorithm the size of population
  /// is constant throughout generations
  pub population_size: usize,

  /// Maximum number of generations (search termination)
  ///
  /// This works interchangeably with `max_duration` - first limit hit terminates the algorithm
  pub generation_limit: usize,

  /// Maximum duration of computations (search termination)
  ///
  /// This works interchangeably with `generation_limit` - first limit hit terminates the algorithm
  pub max_duration: std::time::Duration,
}

/// Configuration for the genetic algorithm
///
/// It describes initial parameters & operators for the algorithm
///
/// Configuration of genetic algorithm is done via builder interface though
pub struct GAConfig<T, M, C, S, P, Pr>
where
  T: Chromosome,
  M: MutationOperator<T>,
  C: CrossoverOperator<T>,
  S: SelectionOperator<T>,
  P: PopulationGenerator<T>,
  Pr: Probe<T>,
{
  /// Set of parameters for the genetic algorithm. See [GAParams] for details.
  pub params: GAParams,

  /// Fitness function. It must operate on a solution representation - chromosome. Should return `f64`.
  /// Please note that some of the genetic operators require fitness function to be positive.
  pub fitness_fn: FitnessFn<T>,

  /// Mutation operator. See [MutationOperator](crate::ga::operators::mutation::MutationOperator) for details.
  pub mutation_operator: M,

  /// Crossover operator. See [CrossoverOperator](create::ga::operators::crossover::CrossoverOperator) for details.
  pub crossover_operator: C,

  /// Selection operator. See [SelectionOperator](crate::ga::operators::selection::SelectionOperator) for details.
  pub selection_operator: S,

  /// Population generator. See [PopulationGenerator](crate::ga::population::PopulationGenerator) for details.
  pub population_factory: P,

	/// Probe. See [Probe](crate::ga::probe::Probe) for details.
  pub probe: Pr,
}

/// Structure representing advancement of the computations
///
/// This struct is passed to probes, so they can make better decisions whether to log or not.
#[derive(Default)]
pub struct GAMetadata {
	/// Computations start time. This field is filled just after GA's `run` method starts executing.
	/// You can expect it to be always defined (the value is `Some`).
  pub start_time: Option<std::time::Instant>,

	/// Duration of the computations. Measured in the very beginning of main loop (each iteration).
  pub duration: Option<std::time::Duration>,

	/// Current generation number. Before entering the main loop it is equal to `0`.
  pub generation: usize,
}

impl GAMetadata {
  pub fn new(
    start_time: Option<std::time::Instant>,
    duration: Option<std::time::Duration>,
    generation: usize,
  ) -> Self {
    GAMetadata {
      start_time,
      duration,
      generation,
    }
  }
}

/// Custom implementation of genetic algorithm.
///
/// TODO: Extensive description
pub struct GeneticAlgorithm<T, M, C, S, P, Pr>
where
  T: Chromosome,
  M: MutationOperator<T>,
  C: CrossoverOperator<T>,
  S: SelectionOperator<T>,
  P: PopulationGenerator<T>,
  Pr: Probe<T>,
{
  config: GAConfig<T, M, C, S, P, Pr>,
  metadata: GAMetadata,
}

impl<T, M, C, S, P, Pr> GeneticAlgorithm<T, M, C, S, P, Pr>
where
  T: Chromosome,
  M: MutationOperator<T>,
  C: CrossoverOperator<T>,
  S: SelectionOperator<T>,
  P: PopulationGenerator<T>,
  Pr: Probe<T>,
{
  pub fn new(config: GAConfig<T, M, C, S, P, Pr>) -> Self {
    GeneticAlgorithm {
      config,
      metadata: GAMetadata::new(None, None, 0),
    }
  }

  fn find_best_individual(population: &Vec<Individual<T>>) -> &Individual<T> {
    debug_assert!(!population.is_empty());
    let mut best_individual = &population[0];
    for idv in population.iter().skip(1) {
      if *idv > *best_individual {
        best_individual = idv;
      }
    }
    best_individual
  }

  fn evaluate_fitness_in_population(&self, population: &mut Vec<Individual<T>>) {
    for idv in population {
      let fitness = (self.config.fitness_fn)(idv);
      idv.fitness = fitness;
    }
  }

  pub fn run(&mut self) -> Option<Individual<T>> {
    self.metadata.start_time = Some(std::time::Instant::now());
    self.config.probe.on_start(&self.metadata);

    // 1. Create initial random population.
    let mut population = self
      .config
      .population_factory
      .generate(self.config.params.population_size);

    // 2. Evaluate fitness for each individual.
    self.evaluate_fitness_in_population(&mut population);

    self.config.probe.on_initial_population_created(&population);

    // 3. Store best individual.
    let mut best_individual_all_time =
      GeneticAlgorithm::<T, M, C, S, P, Pr>::find_best_individual(&population).clone();
    // self.config.probe.on_new_best(&self.metadata, best_individual);

    for generation_no in 1..=self.config.params.generation_limit {
      self.metadata.generation = generation_no;
      self.metadata.duration = Some(self.metadata.start_time.unwrap().elapsed());

      self.config.probe.on_iteration_start(&self.metadata);

      // 2. Evaluate fitness for each individual.
      self.evaluate_fitness_in_population(&mut population);

      // 4. Create mating pool by applying selection operator.
      let mating_pool: Vec<&Individual<T>> =
        self
          .config
          .selection_operator
          .apply(&self.metadata, &population, population.len());

      // 5. From mating pool create new generation (apply crossover & mutation).
      let mut children: Vec<Individual<T>> = Vec::with_capacity(self.config.params.population_size);

      // FIXME: Do not assume that population size is an even number.
      for i in (0..mating_pool.len()).step_by(2) {
        let crt_children = self
          .config
          .crossover_operator
          .apply(mating_pool[i], mating_pool[i + 1]);

        children.push(crt_children.0);
        children.push(crt_children.1);
      }

      // 5.1 Here we should apply the mutations on children?
      (0..children.len()).for_each(|i| {
        self
          .config
          .mutation_operator
          .apply(&mut children[i], self.config.params.mutation_rate)
      });

      // TODO
      // 6. Replacement - merge new generation with old one
      // As for now I'm replacing old population with the new one, but this must be
      // reimplemented. See p. 58 Introduction to Genetic Algorithms.
      population = children;

      // 6. Check for stop condition (Is good enough individual found)? If not goto 2.
      self.evaluate_fitness_in_population(&mut population);

      self.config.probe.on_new_generation(&self.metadata, &population);

      let best_individual = GeneticAlgorithm::<T, M, C, S, P, Pr>::find_best_individual(&population);
      self
        .config
        .probe
        .on_best_fit_in_generation(&self.metadata, best_individual);

      if *best_individual > best_individual_all_time {
        best_individual_all_time = best_individual.clone();
        self
          .config
          .probe
          .on_new_best(&self.metadata, &best_individual_all_time);
      }

      self.config.probe.on_iteration_end(&self.metadata);

      if self.metadata.start_time.unwrap().elapsed() >= self.config.params.max_duration {
        break;
      }
    }

    self
      .config
      .probe
      .on_end(&self.metadata, &population, &best_individual_all_time);
    Some(best_individual_all_time)
  }
}
