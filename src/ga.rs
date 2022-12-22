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

// impl Default for GAParams {
//   fn default() -> Self {
//     Self {
//       selection_rate: 0.5f64,
//       mutation_rate: 0.05,
//       population_size: 100,
//       generation_limit: 200,
//       max_duration: None,
//     }
//   }
// }

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
  // pub ops: GAOps<S>,
  pub fitness_fn: F,
  pub mutation_operator: M,
  pub crossover_operator: C,
  pub selection_operator: S,
  pub replacement_operator: R,
  pub population_factory: P,
  pub probe: Pr,
  phantom: PhantomData<T>,
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

  fn evaluate_population(&self, population: &mut Vec<Individual<T>>) {
    for idv in population {
      let fitness = (self.config.fitness_fn).apply(idv);
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
    self.evaluate_population(&mut population);

    self.config.probe.on_initial_population_created(&population);

    // 3. Store best individual.
    let mut best_individual_all_time = Self::find_best_individual(&population).clone();
    // self.config.probe.on_new_best(&self.metadata, best_individual);

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

      // 5.1 Here we should apply the mutations on children?
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
