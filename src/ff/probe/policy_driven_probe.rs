use crate::ff::probe::{Probe, ProbingPolicy};

pub struct PolicyDrivenProbe {
  probe: Box<dyn Probe>,
  policy: Box<dyn ProbingPolicy>,
}

impl PolicyDrivenProbe {
  pub fn new(probe: Box<dyn Probe>, policy: Box<dyn ProbingPolicy>) -> PolicyDrivenProbe {
    PolicyDrivenProbe { probe, policy }
  }
}

impl Probe for PolicyDrivenProbe {
  fn on_start(&mut self) {
    if self.policy.on_start() {
      self.probe.on_start();
    }
  }

  fn on_iteration_start(&mut self, iteration: u32) {
    if self.policy.on_iteration_start(iteration) {
      self.probe.on_iteration_start(iteration);
    }
  }

  fn on_iteration_end(&mut self, iteration: u32) {
    if self.policy.on_iteration_end(iteration) {
      self.probe.on_iteration_end(iteration);
    }
  }

  fn on_current_best(&mut self, solution: f64, position: &Vec<f64>) {
    if self.policy.on_current_best() {
      self.probe.on_current_best(solution, position);
    }
  }

  fn on_end(&mut self) {
    if self.policy.on_end() {
      self.probe.on_end();
    }
  }
}
