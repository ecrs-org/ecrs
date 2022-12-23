//! Contains stuff related to algorithm termination

use crate::aco::ant::Ant;
use crate::aco::FMatrix;
use std::time::{Duration, Instant};

/// # Termination Condition
///
/// Represents common interface between termination conditions
pub trait TerminationCondition<A: Ant> {
  /// Initialises condition internal state.
  ///
  /// ## Arguments
  /// * `pheromone` - start_pheromone.
  fn init(&mut self, pheromone: &FMatrix);

  /// Updates internal state and then checks if condition is met.
  ///
  /// ## Arguments
  /// * `pheromone` - current pheromone.
  /// * `ants` - Ants containing solutions.
  fn update_and_check(&mut self, pheromone: &FMatrix, ants: &[A]) -> bool;
}

/// # Iteration Condition
///
/// Implements [TerminationCondition].
///
/// Adds an limit on maximal number of iterations.
pub struct IterationCond {
  curr_iteration: usize,
  iterations_limit: usize,
}

impl IterationCond {
  /// Creates a new instance of [IterationCond]. Implements [TerminationCondition]
  ///
  /// ## Arguments
  /// * `iterations_limit` - how many iterations should take place;
  pub fn new(iterations_limit: usize) -> Self {
    Self {
      curr_iteration: 0,
      iterations_limit,
    }
  }
}

impl<A: Ant> TerminationCondition<A> for IterationCond {
  fn init(&mut self, _pheromone: &FMatrix) {
    self.curr_iteration = 0;
  }

  fn update_and_check(&mut self, _pheromone: &FMatrix, _ants: &[A]) -> bool {
    self.curr_iteration += 1;

    self.curr_iteration > self.iterations_limit
  }
}

/// # Time Condition
/// Implements [TerminationCondition].
///
/// If by the end of iteration the algorithm is running longer than specified duration,
/// this condition will be met.
pub struct TimeCond {
  start_time: Instant,
  duration: Duration,
}

impl TimeCond {
  /// Creates a new instance of [TimeCond]. Implements [TerminationCondition]
  ///
  /// ## Arguments
  /// * `duration` - minimal amount of time to terminate algorithm.
  pub fn new(duration: Duration) -> Self {
    Self {
      start_time: Instant::now(),
      duration,
    }
  }
}

impl<A: Ant> TerminationCondition<A> for TimeCond {
  fn init(&mut self, _pheromone: &FMatrix) {
    self.start_time = Instant::now()
  }

  fn update_and_check(&mut self, _pheromone: &FMatrix, _ants: &[A]) -> bool {
    let curr_duration = Instant::now() - self.start_time;

    curr_duration > self.duration
  }
}
