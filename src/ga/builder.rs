use super::operators::selection::SelectionOperator;
use super::{GeneticAlgorithm, GAConfig, FitnessFn, MutationOperator, CrossoverOperator, PopulationGenerator, Probe, GAParams};
use super::individual::{ChromosomeWrapper, Chromosome};

struct GAConfigOpt<T: Chromosome, S: ChromosomeWrapper<T>> {
	params: Option<GAParams>,
  fitness_fn: Option<FitnessFn<S>>,
  mutation_operator: Option<MutationOperator<S>>,
  crossover_operator: Option<CrossoverOperator<S>>,
	selection_operator: Option<Box<dyn SelectionOperator<T, S>>>,
  population_factory: Option<PopulationGenerator<S>>,
  probe: Option<Box<dyn Probe<T, S>>>,
}

impl<T: Chromosome, S: ChromosomeWrapper<T>> Default for GAConfigOpt<T, S> {
	fn default() -> Self {
		Self {
			params: Some(GAParams::default()),
			fitness_fn: None,
			mutation_operator: None,
			crossover_operator: None,
			selection_operator: None,
			population_factory: None,
			probe: None
		}
	}
}

impl<T: Chromosome, S: ChromosomeWrapper<T>> Into<GAConfig<T, S>> for GAConfigOpt<T, S> {
	fn into(self) -> GAConfig<T, S> {
		// FIXME: Unsafe implementation for now
		GAConfig {
			params: self.params.unwrap(),
			fitness_fn: self.fitness_fn.unwrap(),
			mutation_operator: self.mutation_operator.unwrap(),
			crossover_operator: self.crossover_operator.unwrap(),
			selection_operator: self.selection_operator.unwrap(),
			population_factory: self.population_factory.unwrap(),
			probe: self.probe.unwrap()
		}
	}
}

pub struct Builder<T: Chromosome, S: ChromosomeWrapper<T>> {
  config: GAConfigOpt<T, S>,
}

impl<T: Chromosome, S: ChromosomeWrapper<T>> Builder<T, S> {
  pub fn new() -> Self {
    Builder {
      config: GAConfigOpt::default()
    }
  }

	// pub fn new_with_config(config: GAConfig<T, S>) -> Self {
	// 	Builder {
	// 		config
	// 	}
	// }

  pub fn set_mutation_rate(mut self, mutation_rate: f64) -> Self {
    debug_assert!(mutation_rate >= 0f64 && mutation_rate <= 1f64);
		self.config.params = self.config.params.map(|mut params| {params.mutation_rate = mutation_rate; params});
    self
  }

  pub fn set_selection_rate(mut self, selection_rate: f64) -> Self {
    debug_assert!(selection_rate >= 0f64 && selection_rate <= 1f64);
		self.config.params = self.config.params.map(|mut params| {params.selection_rate = selection_rate; params});
    self
  }

	pub fn set_eps(mut self, eps: f64) -> Self {
		self.config.params = self.config.params.map(|mut params| {params.eps = eps; params});
		self
	}

  pub fn set_max_generation_count(mut self, max_gen_count: usize) -> Self {
    debug_assert!(max_gen_count >= 1);
		self.config.params = self.config.params.map(|mut params| {params.generation_upper_bound = max_gen_count; params});
    self
  }

  pub fn set_population_size(mut self, size: usize) -> Self {
    debug_assert!(size > 0);
		self.config.params = self.config.params.map(|mut params| {params.population_size = size; params});
    self
  }

  pub fn set_fitness_fn(mut self, fitness_fn: FitnessFn<S>) -> Self {
    self.config.fitness_fn = Some(fitness_fn);
    self
  }

  pub fn set_mutation_operator(mut self, mutation_op: MutationOperator<S>) -> Self {
    self.config.mutation_operator = Some(mutation_op);
    self
  }

  pub fn set_crossover_operator(mut self, crossover_op: CrossoverOperator<S>) -> Self {
    self.config.crossover_operator = Some(crossover_op);
    self
  }

	pub fn set_selection_operator(mut self, selection_op: Box<dyn SelectionOperator<T, S>>) -> Self {
		self.config.selection_operator = Some(selection_op);
		self
	}

  pub fn set_population_generator(mut self, generator: PopulationGenerator<S>) -> Self {
    self.config.population_factory = Some(generator);
    self
  }

  pub fn set_probe(mut self, probe: Box<dyn Probe<T, S>>) -> Self {
    self.config.probe = Some(probe);
    self
  }

  pub fn build(self) -> GeneticAlgorithm<T, S> {
    GeneticAlgorithm::new(self.config.into())
  }
}
