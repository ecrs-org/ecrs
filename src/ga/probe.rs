use super::{individual::Chromosome, GAMetadata, Individual};

pub mod stdout_probe;
pub mod csv_probe;
pub mod json_probe;

pub trait Probe<T: Chromosome> {
  fn on_start(&mut self, _metadata: &GAMetadata) { /* defaults to noop */ }
	fn on_initial_population_created(&mut self, _population: &[Individual<T>]) { /* defaults to noop */}
  fn on_new_best(&mut self, _metadata: &GAMetadata,_individual: &Individual<T>) { /* defaults to noop */ }
  fn on_mutation(&mut self, _before: &Individual<T>, _after: &Individual<T>) { /* defaults to noop */ }
  fn on_new_generation(&mut self, _generation: &[Individual<T>]) { /* defaults to noop */ }
  fn on_best_fit_in_generation(&mut self, _individual: &Individual<T>) { /* defaults to noop */ }
  fn on_iteration_start(&mut self, _iteration: usize) { /* defaults to noop */ }
  fn on_iteration_end(&mut self, _iteration: usize) { /* defaults to noop */ }
  fn on_end(&mut self) { /* defaults to noop */ }
}
