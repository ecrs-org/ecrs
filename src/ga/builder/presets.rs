use crate::ga::{
  operators::{
    crossover::{CrossoverOperator, SinglePoint},
    mutation::{Identity, MutationOperator},
    selection::{SelectionOperator, Tournament},
  },
  population::{PopulationGenerator, RandomPoints},
  GAParams, Probe, StdoutProbe, FitnessFn, Individual, GeneticAlgorithm,
};

use super::GAConfigOpt;

type RVC = Vec<f64>;
type BC = Vec<bool>;

pub struct RealValuedBuilder
{
  config: GAConfigOpt<RVC, Identity, SinglePoint, Tournament, RandomPoints, StdoutProbe>,
}


impl RealValuedBuilder
{
  pub fn set_selection_rate(mut self, selection_rate: f64) -> Self {
    debug_assert!((0f64..=1f64).contains(&selection_rate));
    self.config.params = self.config.params.map(|mut params| {
      params.selection_rate = selection_rate;
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

	pub fn fitness_fn(mut self, fitness_fn: FitnessFn<Individual<RVC>>) -> Self {
		self.config.fitness_fn = Some(fitness_fn);
		self
	}

	pub fn build(self) -> GeneticAlgorithm<RVC, Identity, SinglePoint, Tournament, RandomPoints, StdoutProbe> {
		let Some(fitness_fn) = self.config.fitness_fn else {
			panic!("Fitness function must be set");
		};

		GeneticAlgorithm::new(self.config.into())
	}
}


pub struct BitStringBuilder {}
