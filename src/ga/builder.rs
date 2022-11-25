pub mod presets;

use super::individual::Chromosome;
use super::operators::selection::SelectionOperator;
use super::population::PopulationGenerator;
use super::{
  CrossoverOperator, FitnessFn, GAConfig, GAParams, GeneticAlgorithm, Individual, MutationOperator, Probe,
};

pub use presets::{BitStringBuilder, RealValuedBuilder};

struct GAConfigOpt<T, M, C, S, P, Pr>
where
  T: Chromosome,
  M: MutationOperator<T>,
  C: CrossoverOperator<T>,
  S: SelectionOperator<T>,
  P: PopulationGenerator<T>,
  Pr: Probe<T>,
{
  params: Option<GAParams>,
  fitness_fn: Option<FitnessFn<Individual<T>>>,
  mutation_operator: Option<M>,
  crossover_operator: Option<C>,
  selection_operator: Option<S>,
  population_factory: Option<P>,
  probe: Option<Pr>,
}

impl<T, M, C, S, P, Pr> GAConfigOpt<T, M, C, S, P, Pr>
where
  T: Chromosome,
  M: MutationOperator<T>,
  C: CrossoverOperator<T>,
  S: SelectionOperator<T>,
  P: PopulationGenerator<T>,
  Pr: Probe<T>,
{
  pub fn new() -> Self {
    Self {
      params: Some(GAParams::default()),
      fitness_fn: None,
      mutation_operator: None,
      crossover_operator: None,
      selection_operator: None,
      population_factory: None,
      probe: None,
    }
  }
}

impl<T, M, C, S, P, Pr> From<GAConfigOpt<T, M, C, S, P, Pr>> for GAConfig<T, M, C, S, P, Pr>
where
  T: Chromosome,
  M: MutationOperator<T>,
  C: CrossoverOperator<T>,
  S: SelectionOperator<T>,
  P: PopulationGenerator<T>,
  Pr: Probe<T>,
{
  fn from(config_opt: GAConfigOpt<T, M, C, S, P, Pr>) -> Self {
    GAConfig {
      params: config_opt.params.unwrap(),
      fitness_fn: config_opt.fitness_fn.unwrap(),
      mutation_operator: config_opt.mutation_operator.unwrap(),
      crossover_operator: config_opt.crossover_operator.unwrap(),
      selection_operator: config_opt.selection_operator.unwrap(),
      population_factory: config_opt.population_factory.unwrap(),
      probe: config_opt.probe.unwrap(),
    }
  }
}

pub struct Builder;

impl Builder {
	#[allow(clippy::new_ret_no_self)]
	pub fn new<T, M, C, S, P, Pr>() -> GenericBuilder<T, M, C, S, P, Pr>
	where
		T: Chromosome,
		M: MutationOperator<T>,
		C: CrossoverOperator<T>,
		S: SelectionOperator<T>,
		P: PopulationGenerator<T>,
		Pr: Probe<T>
	{
		GenericBuilder::new()
	}

	pub fn with_rvc() -> RealValuedBuilder {
		RealValuedBuilder::new()
	}

	pub fn with_bsc() -> BitStringBuilder {
		BitStringBuilder::new()
	}
}

pub struct GenericBuilder<T, M, C, S, P, Pr>
where
  T: Chromosome,
  M: MutationOperator<T>,
  C: CrossoverOperator<T>,
  S: SelectionOperator<T>,
  P: PopulationGenerator<T>,
  Pr: Probe<T>,
{
  config: GAConfigOpt<T, M, C, S, P, Pr>,
}

impl<T, M, C, S, P, Pr> GenericBuilder<T, M, C, S, P, Pr>
where
  T: Chromosome,
  M: MutationOperator<T>,
  C: CrossoverOperator<T>,
  S: SelectionOperator<T>,
  P: PopulationGenerator<T>,
  Pr: Probe<T>,
{
  pub fn new() -> Self {
    GenericBuilder {
      config: GAConfigOpt::new(),
    }
  }

  pub fn set_selection_rate(mut self, selection_rate: f64) -> Self {
    debug_assert!((0f64..=1f64).contains(&selection_rate));
    self.config.params = self.config.params.map(|mut params| {
      params.selection_rate = selection_rate;
      params
    });
    self
  }

	pub fn set_mutation_rate(mut self, mutation_rate: f64) -> Self {
		assert!((0.0..=1.0).contains(&mutation_rate));
		self.config.params = self.config.params.map(|mut params| {
			params.mutation_rate = mutation_rate;
			params
		});
		self
	}

  pub fn set_max_duration(mut self, max_duration: std::time::Duration) -> Self {
    self.config.params = self.config.params.map(|mut params| {
      params.max_duration = Some(max_duration);
      params
    });
    self
  }

  pub fn set_max_generation_count(mut self, max_gen_count: usize) -> Self {
    debug_assert!(max_gen_count >= 1);
    self.config.params = self.config.params.map(|mut params| {
      params.generation_upper_bound = max_gen_count;
      params
    });
    self
  }

  pub fn set_population_size(mut self, size: usize) -> Self {
    debug_assert!(size > 0);
    self.config.params = self.config.params.map(|mut params| {
      params.population_size = size;
      params
    });
    self
  }

  pub fn set_fitness_fn(mut self, fitness_fn: FitnessFn<Individual<T>>) -> Self {
    self.config.fitness_fn = Some(fitness_fn);
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

  pub fn set_population_generator(mut self, generator: P) -> Self {
    self.config.population_factory = Some(generator);
    self
  }

  pub fn set_probe(mut self, probe: Pr) -> Self {
    self.config.probe = Some(probe);
    self
  }

  pub fn build(self) -> GeneticAlgorithm<T, M, C, S, P, Pr> {
    GeneticAlgorithm::new(self.config.into())
  }
}

#[cfg(test)]
mod test {

  #[test]
  fn api_test() {}
}
