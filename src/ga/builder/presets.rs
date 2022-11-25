use crate::ga::{
  operators::{
    crossover::SinglePoint,
    mutation::Identity,
    selection::Tournament,
  },
  population::{RandomPoints, BitStrings},
  StdoutProbe, FitnessFn, Individual, GeneticAlgorithm,
};

use super::GAConfigOpt;

type Rvc = Vec<f64>;
type Bsc = Vec<bool>;

pub struct RealValuedBuilder
{
  config: GAConfigOpt<Rvc, Identity, SinglePoint, Tournament, RandomPoints, StdoutProbe>,
	dim: Option<usize>,
}


impl RealValuedBuilder
{
	pub(super) fn new() -> Self {
		RealValuedBuilder { config: GAConfigOpt::new(), dim: None }
	}

  pub fn selection_rate(mut self, selection_rate: f64) -> Self {
    assert!((0f64..=1f64).contains(&selection_rate));
    self.config.params = self.config.params.map(|mut params| {
      params.selection_rate = selection_rate;
      params
    });
    self
  }

  pub fn max_duration(mut self, max_duration: std::time::Duration) -> Self {
    self.config.params = self.config.params.map(|mut params| {
      params.max_duration = Some(max_duration);
      params
    });
    self
  }

  pub fn max_generations(mut self, max_gen_count: usize) -> Self {
    assert!(max_gen_count >= 1);
    self.config.params = self.config.params.map(|mut params| {
      params.generation_upper_bound = max_gen_count;
      params
    });
    self
  }

  pub fn population_size(mut self, size: usize) -> Self {
    assert!(size > 0);
    self.config.params = self.config.params.map(|mut params| {
      params.population_size = size;
      params
    });
    self
  }

	pub fn dim(mut self, dim: usize) -> Self {
		assert!(dim > 0, "Dimension of a problem must be > 0");
		self.dim = Some(dim);
		self
	}

	pub fn fitness_fn(mut self, fitness_fn: FitnessFn<Individual<Rvc>>) -> Self {
		self.config.fitness_fn = Some(fitness_fn);
		self
	}

	pub fn build(self) -> GeneticAlgorithm<Rvc, Identity, SinglePoint, Tournament, RandomPoints, StdoutProbe> {
		 if self.config.fitness_fn.is_none() {
				panic!("Fitness function must be set");
		 }

		if self.dim.is_none() {
			panic!("Problem dimension must be set");
		}

		GeneticAlgorithm::new(self.config.into())
	}
}


pub struct BitStringBuilder {
  config: GAConfigOpt<Bsc, Identity, SinglePoint, Tournament, BitStrings, StdoutProbe>,
	dim: Option<usize>,
}

impl BitStringBuilder {
	pub(super) fn new() -> Self {
		BitStringBuilder { config: GAConfigOpt::new(), dim: None }
	}

  pub fn selection_rate(mut self, selection_rate: f64) -> Self {
    assert!((0f64..=1f64).contains(&selection_rate));
    self.config.params = self.config.params.map(|mut params| {
      params.selection_rate = selection_rate;
      params
    });
    self
  }

  pub fn max_duration(mut self, max_duration: std::time::Duration) -> Self {
    self.config.params = self.config.params.map(|mut params| {
      params.max_duration = Some(max_duration);
      params
    });
    self
  }

  pub fn max_generations(mut self, max_gen_count: usize) -> Self {
    assert!(max_gen_count >= 1);
    self.config.params = self.config.params.map(|mut params| {
      params.generation_upper_bound = max_gen_count;
      params
    });
    self
  }

  pub fn population_size(mut self, size: usize) -> Self {
    assert!(size > 0);
    self.config.params = self.config.params.map(|mut params| {
      params.population_size = size;
      params
    });
    self
  }

	pub fn dim(mut self, dim: usize) -> Self {
		assert!(dim > 0, "Dimension of a problem must be > 0");
		self.dim = Some(dim);
		self
	}

	pub fn fitness_fn(mut self, fitness_fn: FitnessFn<Individual<Bsc>>) -> Self {
		self.config.fitness_fn = Some(fitness_fn);
		self
	}

	pub fn build(self) -> GeneticAlgorithm<Bsc, Identity, SinglePoint, Tournament, BitStrings, StdoutProbe> {
		 if self.config.fitness_fn.is_none() {
				panic!("Fitness function must be set");
		 }

		if self.dim.is_none() {
			panic!("Problem dimension must be set");
		}

		GeneticAlgorithm::new(self.config.into())
	}
}
