use crate::ff::probe::stdout_probe::StdoutProbe;
use crate::ff::probe::Probe;
use crate::ff::*;
use rand::distributions::Standard;
use rand::prelude::Distribution;

pub struct FireflyAlgorithmBuilder<T: Distribution<f64>> {
  config: FireflyAlgorithmCfg<T>,
  brightness_function: fn(&Vec<f64>) -> f64,
  probe: Box<dyn Probe>,
  distance_function: fn(&Vec<f64>, &[f64]) -> f64,
}

impl<T: Distribution<f64>> FireflyAlgorithmBuilder<T> {
  pub fn new() -> FireflyAlgorithmBuilder<Standard> {
    FireflyAlgorithmBuilder {
      config: FireflyAlgorithmCfg::default(),
      brightness_function: rastrigin,
      probe: Box::new(StdoutProbe {}),
      distance_function: cartesian_distance,
    }
  }

  pub fn set_dimensions(mut self, dimensions: u8) -> Self {
    self.config.dimensions = dimensions;
    self
  }

  pub fn set_lower_bound(mut self, lower_bound: f64) -> Self {
    self.config.lower_bound = lower_bound;
    self
  }
  pub fn set_upper_bound(mut self, upper_bound: f64) -> Self {
    self.config.upper_bound = upper_bound;
    self
  }
  pub fn set_max_generations(mut self, max_generations: u32) -> Self {
    self.config.max_generations = max_generations;
    self
  }
  pub fn set_population_size(mut self, population_size: u32) -> Self {
    self.config.population_size = population_size;
    self
  }
  pub fn set_alfa0(mut self, alfa0: f64) -> Self {
    self.config.alfa0 = alfa0;
    self
  }
  pub fn set_beta0(mut self, beta0: f64) -> Self {
    self.config.beta0 = beta0;
    self
  }
  pub fn set_gamma(mut self, gamma: f64) -> Self {
    self.config.gamma = gamma;
    self
  }
  pub fn set_delta(mut self, delta: f64) -> Self {
    self.config.delta = delta;
    self
  }
  pub fn set_threads(mut self, threads: u8) -> Self {
    self.config.threads = threads;
    self
  }

  pub fn set_config(mut self, config: FireflyAlgorithmCfg<T>) -> Self {
    self.config = config;
    self
  }

  pub fn set_distribution(mut self, distribution: T) -> Self {
    self.config.distribution = distribution;
    self
  }

  pub fn set_brightness_function(mut self, brightness_function: fn(&Vec<f64>) -> f64) -> Self {
    self.brightness_function = brightness_function;
    self
  }

  pub fn set_probe(mut self, probe: Box<dyn Probe>) -> Self {
    self.probe = probe;
    self
  }

  pub fn set_distance_function(mut self, distance_function: fn(&Vec<f64>, &[f64]) -> f64) -> Self {
    self.distance_function = distance_function;
    self
  }

  pub fn build(self) -> FireflyAlgorithm<T> {
    FireflyAlgorithm {
      config: self.config,
      brightness_function: self.brightness_function,
      probe: self.probe,
      distance_function: self.distance_function,
    }
  }
}
