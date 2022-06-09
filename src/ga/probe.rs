use crate::ga::Individual;

pub mod stdout_probe;
pub mod csv_probe;

pub trait Probe {
  fn on_start(&mut self) { /* defaults to noop */ }
  fn on_new_best(&mut self, individual: &Individual) { /* defaults to noop */ }
  fn on_mutation(&mut self, before: &Individual, after: &Individual) { /* defaults to noop */ }
  fn on_new_generation(&mut self) { /* defaults to noop */ }
  fn on_best_fit_in_generation(&mut self, individual: &Individual) { /* defaults to noop */ }
  fn on_iteration_start(&mut self, iteration: usize) { /* defaults to noop */ }
  fn on_iteration_end(&mut self, iteration: usize) { /* defaults to noop */ }
}

