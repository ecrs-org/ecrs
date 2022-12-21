pub mod aggregated_probe;
pub mod csv_probe;
pub mod json_probe;
pub mod policy_driven_probe;
pub mod probing_policy;
pub mod stdout_probe;

pub trait Probe {
  fn on_start(&mut self);
  fn on_iteration_start(&mut self, iteration: u32);
  fn on_iteration_end(&mut self, iteration: u32);
  fn on_current_best(&mut self, solution: f64, position: &Vec<f64>);
  fn on_end(&mut self);
}

pub trait ProbingPolicy {
  fn on_start(&mut self) -> bool;
  fn on_iteration_start(&mut self, iteration: u32) -> bool;
  fn on_iteration_end(&mut self, iteration: u32) -> bool;
  fn on_current_best(&mut self) -> bool;
  fn on_end(&mut self) -> bool;
}
