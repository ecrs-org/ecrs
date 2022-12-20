use std::ops::Add;
use std::time::{Duration, Instant};

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
  pub fn new(interval: usize, first_threshold: usize) -> GenerationInterval {
    GenerationInterval {
      interval,
      threshold: first_threshold,
      should_log: false,
    }
  }
}

impl ProbingPolicy for GenerationInterval {
  fn on_pheromone_update(&mut self) -> bool {
    self.should_log
  }

  fn on_current_best(&mut self) -> bool {
    self.should_log
  }

  fn on_iteration_start(&mut self, iteration: usize) -> bool {
    if iteration >= self.threshold {
      self.threshold += self.interval;
      self.should_log = true;
    }
    self.should_log
  }

  fn on_iteration_end(&mut self, _iteration: usize) -> bool {
    let should_log_current = self.should_log;
    self.should_log = false;
    should_log_current
  }

  fn on_end(&mut self) -> bool {
    true
  }
}

pub struct ElapsedTime {
  interval: usize,
  threshold: Instant,
  should_log: bool,
}

impl ElapsedTime {
  /// Returns a new instance of [ElapsedTime] policy
  ///
  /// ### Arguments
  ///
  /// * `interval` - time between logging iterations
  /// * `first_threshold` - time of first logging iteration
  pub fn new(interval: usize, first_threshold: usize) -> ElapsedTime {
    ElapsedTime {
      interval,
      threshold: Instant::now().add(Duration::new(first_threshold as u64, 0)),
      should_log: false,
    }
  }
}

impl ProbingPolicy for ElapsedTime {
  fn on_pheromone_update(&mut self) -> bool {
    self.should_log
  }

  fn on_current_best(&mut self) -> bool {
    self.should_log
  }

  fn on_iteration_start(&mut self, _iteration: usize) -> bool {
    if self.threshold.elapsed().as_secs() >= self.interval as u64 {
      self.threshold = Instant::now();
      self.should_log = true;
    }
    self.should_log
  }

  fn on_iteration_end(&mut self, _iteration: usize) -> bool {
    let should_log_current = self.should_log;
    self.should_log = false;
    should_log_current
  }

  fn on_end(&mut self) -> bool {
    true
  }
}
