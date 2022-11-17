use super::operators::selection::SelectionOperator;
use super::population::PopulationGenerator;
use super::{GeneticAlgorithm, GAConfig, FitnessFn, MutationOperator, CrossoverOperator, Probe, GAParams};
use super::individual::{ChromosomeWrapper, Chromosome};

struct GAConfigOpt<T: Chromosome, S: ChromosomeWrapper<T>> {
	params: Option<GAParams>,
  fitness_fn: Option<FitnessFn<S>>,
  mutation_operator: Option<Box<dyn MutationOperator<T, S>>>,
  crossover_operator: Option<Box<dyn CrossoverOperator<T, S>>>,
	selection_operator: Option<Box<dyn SelectionOperator<T, S>>>,
  population_factory: Option<Box<dyn PopulationGenerator<T, S>>>,
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

impl<T: Chromosome, S: ChromosomeWrapper<T>> From<GAConfigOpt<T, S>> for GAConfig<T, S> {
	fn from(config_opt: GAConfigOpt<T, S>) -> Self {
		GAConfig {
			params: config_opt.params.unwrap(),
			fitness_fn: config_opt.fitness_fn.unwrap(),
			mutation_operator: config_opt.mutation_operator.unwrap(),
			crossover_operator: config_opt.crossover_operator.unwrap(),
			selection_operator: config_opt.selection_operator.unwrap(),
			population_factory: config_opt.population_factory.unwrap(),
			probe: config_opt.probe.unwrap()
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

  pub fn set_selection_rate(mut self, selection_rate: f64) -> Self {
    debug_assert!((0f64..=1f64).contains(&selection_rate));
		self.config.params = self.config.params.map(|mut params| {params.selection_rate = selection_rate; params});
    self
  }

	pub fn set_max_duration(mut self, max_duration: std::time::Duration) -> Self {
		self.config.params = self.config.params.map(|mut params| {params.max_duration = Some(max_duration); params});
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

  pub fn set_mutation_operator(mut self, mutation_op: Box<dyn MutationOperator<T, S>>) -> Self {
    self.config.mutation_operator = Some(mutation_op);
    self
  }

  pub fn set_crossover_operator(mut self, crossover_op: Box<dyn CrossoverOperator<T, S>>) -> Self {
    self.config.crossover_operator = Some(crossover_op);
    self
  }

	pub fn set_selection_operator(mut self, selection_op: Box<dyn SelectionOperator<T, S>>) -> Self {
		self.config.selection_operator = Some(selection_op);
		self
	}

  pub fn set_population_generator(mut self, generator: Box<dyn PopulationGenerator<T, S>>) -> Self {
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
