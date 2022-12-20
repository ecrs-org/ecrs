use crate::aco::probe::{Probe, ProbingPolicy};
use crate::aco::{FMatrix, Solution};

/// ## PolicyDrivenProbe
///
/// Checks whether policy allows for logging and if so, delegates actual logging to wrapped probe
pub struct PolicyDrivenProbe {
  probe: Box<dyn Probe>,
  policy: Box<dyn ProbingPolicy>,
}

impl PolicyDrivenProbe {
  /// Returns new instance of [PolicyDrivenProbe]
  ///
  /// ### Arguments
  ///
  /// * `policy` - logging policy to apply
  /// * `probe` - probe used to logging
  pub fn new(probe: Box<dyn Probe>, policy: Box<dyn ProbingPolicy>) -> PolicyDrivenProbe {
    PolicyDrivenProbe { probe, policy }
  }
}

impl Probe for PolicyDrivenProbe {
  /// This method is called to report on pheromone update.
  ///
  /// Delegates actual logging to wrapped `probe` only if `policy` returns `true`
  ///
  /// ### Arguments
  ///
  /// * `old_pheromone` - Matrix containing pheromone values before update
  /// * `new_pheromone` - Matrix containing pheromone values after update
  fn on_pheromone_update(&mut self, old_pheromone: &FMatrix, new_pheromone: &FMatrix) {
    if self.policy.on_pheromone_update() {
      self.probe.on_pheromone_update(old_pheromone, new_pheromone);
    }
  }

  /// This method is called to report on the best solution in current iteration.
  ///
  /// Delegates actual logging to wrapped `probe` only if `policy` returns `true`
  ///
  /// ### Arguments
  ///
  /// * `best` - current best solution
  fn on_current_best(&mut self, best: &Solution) {
    if self.policy.on_current_best() {
      self.probe.on_current_best(best);
    }
  }

  /// This method is called in the very begining of algorithm's main loop.
  ///
  /// Delegates actual logging to wrapped `probe` only if `policy` returns `true`
  ///
  /// ### Arguments
  ///
  /// * `iteration` - current iteration number
  fn on_iteration_start(&mut self, iteration: usize) {
    if self.policy.on_iteration_start(iteration) {
      self.probe.on_iteration_start(iteration);
    }
  }

  /// This method is called in the very end of algorithm's main loop, just before
  /// termination conditions are evaluated.
  ///
  /// Delegates actual logging to wrapped `probe` only if `policy` returns `true`
  ///
  /// ### Arguments
  ///
  /// * `iteration` - current iteration number
  fn on_iteration_end(&mut self, iteration: usize) {
    if self.policy.on_iteration_end(iteration) {
      self.probe.on_iteration_end(iteration);
    }
  }

  /// This method is called after algorithm 's main loop is exited, just before the `run`
  /// method returns
  ///
  /// Delegates actual logging to wrapped `probe` only if `policy` returns `true`
  fn on_end(&mut self) {
    if self.policy.on_end() {
      self.probe.on_end();
    }
  }
}
