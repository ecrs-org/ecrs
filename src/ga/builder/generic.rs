use crate::ga::builder::FitnessFn;
use crate::ga::operators::fitness::{Fitness, FnBasedFitness};
use crate::ga::operators::replacement::ReplacementOperator;
use crate::ga::{
  individual::Chromosome,
  operators::{crossover::CrossoverOperator, mutation::MutationOperator, selection::SelectionOperator},
  population::PopulationGenerator,
  GeneticAlgorithm, Probe,
};

use super::{DefaultParams, GAConfigOpt};

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
  pub fn new() -> Self {
    GenericBuilder {
      config: GAConfigOpt::new(),
    }
  }

  pub fn set_selection_rate(mut self, selection_rate: f64) -> Self {
    debug_assert!((0f64..=1f64).contains(&selection_rate));
    self.config.params.selection_rate = Some(selection_rate);
    self
  }

  pub fn set_mutation_rate(mut self, mutation_rate: f64) -> Self {
    assert!((0.0..=1.0).contains(&mutation_rate));
    self.config.params.mutation_rate = Some(mutation_rate);
    self
  }

  pub fn set_max_duration(mut self, max_duration: std::time::Duration) -> Self {
    self.config.params.max_duration = Some(max_duration);
    self
  }

  pub fn set_max_generation_count(mut self, max_gen_count: usize) -> Self {
    debug_assert!(max_gen_count >= 1);
    self.config.params.generation_limit = Some(max_gen_count);
    self
  }

  pub fn set_population_size(mut self, size: usize) -> Self {
    debug_assert!(size > 0);
    self.config.params.population_size = Some(size);
    self
  }

  pub fn set_fitness(mut self, fitness: F) -> Self {
    self.config.fitness_fn = Some(fitness);
    self
  }

  pub fn set_mutation_operator(mut self, mutation_op: M) -> Self {
    self.config.mutation_operator = Some(mutation_op);
    self
  }

  pub fn set_crossover_operator(mut self, crossover_op: C) -> Self {
    self.config.crossover_operator = Some(crossover_op);
    self
  }

  pub fn set_selection_operator(mut self, selection_op: S) -> Self {
    self.config.selection_operator = Some(selection_op);
    self
  }

  pub fn set_replacement_operator(mut self, replacement_op: R) -> Self {
    self.config.replacement_operator = Some(replacement_op);
    self
  }

  pub fn set_population_generator(mut self, generator: P) -> Self {
    self.config.population_factory = Some(generator);
    self
  }

  pub fn set_probe(mut self, probe: Pr) -> Self {
    self.config.probe = Some(probe);
    self
  }

  pub fn build(mut self) -> GeneticAlgorithm<T, M, C, S, R, P, F, Pr> {
    self.config.params.fill_from(&Self::DEFAULT_PARAMS);

    let config = match self.config.try_into() {
      Ok(config) => config,
      Err(err) => panic!("Builder panicked with error: {}", err),
    };

    GeneticAlgorithm::new(config)
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
