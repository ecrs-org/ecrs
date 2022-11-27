use crate::ga::individual::Chromosome;

use super::ProbingPolicy;

pub struct GenerationInterval {
  interval: usize,
  threshold: usize,
  should_log: bool,
}

impl GenerationInterval {
  /// Returns new instance of [GenerationInverval] policy
  ///
  /// ### Arguments
  ///
  /// * `interval` - how many iteration should be skipped between logs
  /// * `first_threshold` - number of first iteration to log
  pub fn new(interval: usize, first_threshold: usize) -> Self {
    Self {
      interval,
      threshold: first_threshold,
      should_log: false,
    }
  }
}

impl<T: Chromosome> ProbingPolicy<T> for GenerationInterval {
  #[inline(always)]
  fn on_start(&mut self, _metadata: &crate::ga::GAMetadata) -> bool {
    true
  }

  #[inline(always)]
  fn on_initial_population_created(&mut self, _population: &[crate::ga::Individual<T>]) -> bool {
    true
  }

  #[inline(always)]
  fn on_new_best(
    &mut self,
    _metadata: &crate::ga::GAMetadata,
    _individual: &crate::ga::Individual<T>,
  ) -> bool {
    true
  }

  #[inline(always)]
  fn on_new_generation(&mut self, _generation: &[crate::ga::Individual<T>]) -> bool {
    self.should_log
  }

  #[inline(always)]
  fn on_best_fit_in_generation(
    &mut self,
    _metadata: &crate::ga::GAMetadata,
    _individual: &crate::ga::Individual<T>,
  ) -> bool {
    self.should_log
  }

  #[inline]
  fn on_iteration_start(&mut self, iteration: usize) -> bool {
    if iteration >= self.threshold {
      self.threshold += self.interval;
      self.should_log = true;
      true
    } else {
      false
    }
  }

  #[inline(always)]
  fn on_iteration_end(&mut self, _iteration: usize) -> bool {
    let prev = self.should_log;
    self.should_log = false;
    prev
  }

  #[inline(always)]
  fn on_end(
    &mut self,
    _metadata: &crate::ga::GAMetadata,
    _population: &[crate::ga::Individual<T>],
    _best_individual: &crate::ga::Individual<T>,
  ) -> bool {
    true
  }
}
