use super::{individual::Chromosome, GAMetadata, Individual};

pub mod csv_probe;
pub mod empty;
pub mod json_probe;
pub mod probing_policy;
pub mod stdout_probe;
pub mod policy_driven_probe;

pub trait Probe<T: Chromosome> {
  fn on_start(&mut self, _metadata: &GAMetadata) { /* defaults to noop */
  }
  fn on_initial_population_created(&mut self, _population: &[Individual<T>]) {
    /* defaults to noop */
  }
  fn on_new_best(&mut self, _metadata: &GAMetadata, _individual: &Individual<T>) {
    /* defaults to noop */
  }
  fn on_new_generation(&mut self, _generation: &[Individual<T>]) { /* defaults to noop */
  }
  fn on_best_fit_in_generation(&mut self, _metadata: &GAMetadata, _individual: &Individual<T>) {
    /* defaults to noop */
  }
  fn on_iteration_start(&mut self, _iteration: usize) { /* defaults to noop */
  }
  fn on_iteration_end(&mut self, _iteration: usize) { /* defaults to noop */
  }
  fn on_end(
    &mut self,
    _metadata: &GAMetadata,
    _population: &[Individual<T>],
    _best_individual: &Individual<T>,
  ) { /* defaults to noop */
  }
}

pub trait ProbingPolicy<T: Chromosome> {
  fn on_start(&mut self, _metadata: &GAMetadata) -> bool;
  fn on_initial_population_created(&mut self, _population: &[Individual<T>]) -> bool;
  fn on_new_best(&mut self, _metadata: &GAMetadata, _individual: &Individual<T>) -> bool;
  fn on_new_generation(&mut self, _generation: &[Individual<T>]) -> bool;
  fn on_best_fit_in_generation(&mut self, _metadata: &GAMetadata, _individual: &Individual<T>) -> bool;
  fn on_iteration_start(&mut self, _iteration: usize) -> bool;
  fn on_iteration_end(&mut self, _iteration: usize) -> bool;
  fn on_end(
    &mut self,
    _metadata: &GAMetadata,
    _population: &[Individual<T>],
    _best_individual: &Individual<T>,
  ) -> bool;
}
