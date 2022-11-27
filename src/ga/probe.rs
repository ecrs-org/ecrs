use super::{individual::Chromosome, GAMetadata, Individual};

mod csv_probe;
mod empty;
mod json_probe;
mod policy_driven_probe;
mod probing_policy;
mod stdout_probe;

pub use csv_probe::CsvProbe;
pub use empty::EmptyProbe;
pub use json_probe::JsonProbe;
pub use policy_driven_probe::PolicyDrivenProbe;
pub use probing_policy::{ElapsedTime, GenerationInterval};
pub use stdout_probe::StdoutProbe;

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
  fn on_iteration_start(&mut self, _metadata: &GAMetadata) { /* defaults to noop */
  }
  fn on_iteration_end(&mut self, _metadata: &GAMetadata) { /* defaults to noop */
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
  fn on_iteration_start(&mut self, _metadata: &GAMetadata) -> bool;
  fn on_iteration_end(&mut self, _metadata: &GAMetadata) -> bool;
  fn on_end(
    &mut self,
    _metadata: &GAMetadata,
    _population: &[Individual<T>],
    _best_individual: &Individual<T>,
  ) -> bool;
}
