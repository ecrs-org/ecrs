use super::{GeneticAlgorithm, GeneticAlgorithmCfg, FitnessFn, MutationOperator, CrossoverOperator, PopulationGenerator, Probe};


pub struct Builder {
  config: GeneticAlgorithmCfg,
}

impl Builder {
  pub fn new() -> Self {
    Builder {
      config: GeneticAlgorithmCfg::default()
    }
  }

  pub fn set_mutation_rate(mut self, mutation_rate: f64) -> Self {
    debug_assert!(mutation_rate >= 0f64 && mutation_rate <= 1f64);
    self.config.mutation_rate = mutation_rate;
    self
  }

  pub fn set_selection_rate(mut self, selection_rate: f64) -> Self {
    debug_assert!(selection_rate >= 0f64 && selection_rate <= 1f64);
    self.config.selection_rate = selection_rate;
    self
  }

	pub fn set_eps(mut self, eps: f64) -> Self {
		self.config.eps = eps;
		self
	}

  pub fn set_max_generation_count(mut self, max_gen_count: i32) -> Self {
    debug_assert!(max_gen_count >= 1);
    self.config.generation_upper_bound = max_gen_count;
    self
  }

  pub fn set_population_size(mut self, size: usize) -> Self {
    debug_assert!(size > 0);
    self.config.population_size = size;
    self
  }

  pub fn set_fitness_fn(mut self, fitness_fn: FitnessFn) -> Self {
    self.config.fitness_fn = fitness_fn;
    self
  }

  pub fn set_mutation_operator(mut self, mutation_op: MutationOperator) -> Self {
    self.config.mutation_operator = mutation_op;
    self
  }

  pub fn set_crossover_operator(mut self, crossover_op: CrossoverOperator) -> Self {
    self.config.crossover_operator = crossover_op;
    self
  }

  pub fn set_population_generator(mut self, generator: PopulationGenerator) -> Self {
    self.config.population_factory = generator;
    self
  }

  pub fn set_probe(mut self, probe: Box<dyn Probe>) -> Self {
    self.config.probe = probe;
    self
  }

  pub fn build(self) -> GeneticAlgorithm {
    GeneticAlgorithm::new(self.config)
  }
}
