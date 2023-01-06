//! Implementation of genetic algorithm and genetic operators
//!
//! #### Description
//!
//! Evolutionary computation can be perceived as group of optimization
//! algorithms behaviour of which is mainly based on naturally occuring
//! processes. In this case, the process is Darwin's evolution and it's
//! "the strongest is the most likely to survive" (apologies to all biologists)
//! rule. This is the basis behind evolutionary (in particular - genetic) algorithms.
//!
//!	For better grasp of theory we recommend taking a look into literature such as
//! "Introduction to Genetic Algorithms" by S. N. Sivanandam & S. N. Deepa
//! (there are plenty more papers & books on this topic though). Here, below
//! we explain shortly only the most important terminology.
//!
//! The basic term used by genetic algorithm is `individual` (see our [`Individual`](crate::ga::individual::Individual) type).
//! It describes any point (usually called `solution`) from problem domain. Its encoded version
//! e.g. to bitstring is called `chromosome` (see [`Chromosome`](crate::ga::individual::Chromosome) type).
//! The `chromosome` is comprised of `genes`. The possible set of values that gene can take is called `allele`.
//!
//! Let's look at example.
//!
//! Let's say that we want to optimize $f(x) = x^2$ where
//! $x \in {0, 1, 2, \ldots, 7}$. The possible solutions (individuals) would be any of the values
//! from domain - 0, 1, 2, 3. Let 1 be an individual then, and `001` be its `chromosome` (enconding).
//! Then the `allele` would be ${0, 1}$ for each genee (set of possible gene values).
//!
//! The overall structure of genetic algorithm:
//!
//! 1. Generate initial population
//! 2. Evalute population (what is the value of fitness function for each individual?)
//! 3. Apply selection operator
//! 4. Apply crossover operator
//! 5. Apply mutation operator
//! 6. Apply replacement operator
//! 7. Termination condition satisfied? Yes -> END, no -> go to 2
//!
//! The `population` is a set of feasible solutions to the problem (individuals). Usually initial
//! `population` is created by random generation (see our [population generators](crate::ga::population)).
//!
//! Such `population` is later transformed (evolves) by applying series of transformers (operators).
//!
//! For description of each operator we recommend reading their docs.
//!
//!
//! * See [selection operators](crate::ga::operators::selection)
//! * See [crossover operators](crate::ga::operators::crossover)
//! * See [mutation operators](crate::ga::operators::mutation)
//! * See [replacement operators](crate::ga::operators::replacement)
//!
//! #### Basic usage
//!
//! The instance of genetic algorithm is created with usage of its builder (see [Builder](crate::ga::builder::Builder)).
//!
//! ```no_run
//! use ecrs::prelude::*;
//!
//! # fn rastrigin_fitness(chromosome: &Vec<f64>) -> f64 {
//! #   1000.0 * f64::exp(-ecrs::test_functions::rastrigin(chromosome))
//! # }
//!
//! let mut res = ga::Builder::new()
//!   .set_max_generation_count(50_000)
//!   .set_population_size(100)
//!   .set_fitness_fn(rastrigin_fitness)
//!   .set_crossover_operator(ops::crossover::SinglePoint::new())
//!   .set_replacement_operator(ops::replacement::WeakParent::new())
//!   .set_mutation_operator(ops::mutation::Identity::new())
//!   .set_population_generator(population::RandomPoints::with_constraints(
//!     3,
//!     vec![-5.12..5.12, -5.12..5.12, -5.12..5.12],
//!   ))
//!   .set_selection_operator(ga::operators::selection::Boltzmann::new(0.05, 80.0, 500, false))
//!   .set_probe(
//!     ga::probe::AggregatedProbe::new()
//!       .add_probe(ga::probe::PolicyDrivenProbe::new(
//!         ga::probe::ElapsedTime::new(std::time::Duration::from_millis(200), std::time::Duration::ZERO),
//!         ga::probe::StdoutProbe,
//!       ))
//!       .add_probe(ga::probe::PolicyDrivenProbe::new(
//!         ga::probe::GenerationInterval::new(500, 100),
//!         ga::probe::StdoutProbe,
//!       )),
//!   )
//!   .build();
//!
//! // Run the algorithm
//!	let result = res.run();
//! ```
//!
//! Hella, so many configuration steps?! Let me be clear: there are evem more configuration possibilites. But they are **possibilities**!
//! The minimum you must specify:
//!
//! 1. Fitness function (the algorithm must know what it is optimizing)
//! 2. Problem dimension
//! 3. Population generator (the algorithm must be able to create initial population)
//! 4. Probe (the logging object -- if you don't want to see any logs other than final result just pass [Empty probe](crate::ga::probes::Empty))
//!
//! The defaults for operators and parameters are provided for two types of chromosomes: bit string and real valued vector (see docs of [Builder](crage::ga::builder::Builder)),
//! but keep in mind that these default options might not be even good for your particular problem as the operators & parameters should be
//! tailored individually for each problem.
//!
//! * See [probes & configuration](ecrs::ga::probe)
//! * See [population generators](ecrs::ga::population)
//! * See [fitness & configuration](ecrs::ga::fitness)
//! * See [available params](self::GAParams)

pub mod builder;
pub mod individual;
pub mod operators;
pub mod population;
pub mod probe;

use crate::ga::operators::fitness::Fitness;
pub use builder::*;
pub use individual::Individual;
pub use probe::CsvProbe;
pub use probe::JsonProbe;
pub use probe::Probe;
pub use probe::StdoutProbe;
use std::marker::PhantomData;

use self::{
  individual::Chromosome,
  operators::{
    crossover::CrossoverOperator, mutation::MutationOperator, replacement::ReplacementOperator,
    selection::SelectionOperator,
  },
  population::PopulationGenerator,
};

pub struct GAParams {
  pub selection_rate: f64,
  pub mutation_rate: f64,
  pub population_size: usize,
  pub generation_limit: usize,
  pub max_duration: std::time::Duration,
}

pub struct GAConfig<T, M, C, S, R, P, F, Pr>
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
  pub params: GAParams,
  pub fitness_fn: F,
  pub mutation_operator: M,
  pub crossover_operator: C,
  pub selection_operator: S,
  pub replacement_operator: R,
  pub population_factory: P,
  pub probe: Pr,
  _phantom: PhantomData<T>,
}

#[derive(Default)]
pub struct GAMetadata {
  pub start_time: Option<std::time::Instant>,
  pub duration: Option<std::time::Duration>,
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

pub struct GeneticAlgorithm<T, M, C, S, R, P, F, Pr>
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
  config: GAConfig<T, M, C, S, R, P, F, Pr>,
  metadata: GAMetadata,
}

impl<T, M, C, S, R, P, F, Pr> GeneticAlgorithm<T, M, C, S, R, P, F, Pr>
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
  pub fn new(config: GAConfig<T, M, C, S, R, P, F, Pr>) -> Self {
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

  #[inline(always)]
  fn evaluate_population(&mut self, population: &mut [Individual<T>]) {
    population
      .iter_mut()
      .for_each(|idv| idv.fitness = (self.config.fitness_fn).apply(idv));
  }

  #[inline(always)]
  fn generate_population(&mut self) -> Vec<Individual<T>> {
    self
      .config
      .population_factory
      .generate(self.config.params.population_size)
  }

  pub fn run(&mut self) -> Option<Individual<T>> {
    self.metadata.start_time = Some(std::time::Instant::now());
    self.config.probe.on_start(&self.metadata);

    let mut population = self.generate_population();

    self.evaluate_population(&mut population);

    self.config.probe.on_initial_population_created(&population);

    let mut best_individual_all_time = Self::find_best_individual(&population).clone();

    self
      .config
      .probe
      .on_new_best(&self.metadata, &best_individual_all_time);

    for generation_no in 1..=self.config.params.generation_limit {
      self.metadata.generation = generation_no;
      self.metadata.duration = Some(self.metadata.start_time.unwrap().elapsed());

      self.config.probe.on_iteration_start(&self.metadata);

      // 2. Evaluate fitness for each individual.
      self.evaluate_population(&mut population);

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

      (0..children.len()).for_each(|i| {
        self
          .config
          .mutation_operator
          .apply(&mut children[i], self.config.params.mutation_rate)
      });

      if self.config.replacement_operator.requires_children_fitness() {
        self.evaluate_population(&mut children);
      }

      // 6. Replacement - merge new generation with old one
      population = self.config.replacement_operator.apply(population, children);

      // 7. Check for stop condition (Is good enough individual found)? If not goto 2.
      self.evaluate_population(&mut population);

      self.config.probe.on_new_generation(&self.metadata, &population);

      let best_individual = Self::find_best_individual(&population);
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

#[cfg(test)]
mod tests {
  use super::GAMetadata;

  #[test]
  fn gametadata_can_be_constructed_with_new_fn() {
    GAMetadata::new(None, None, 0);
  }
}
