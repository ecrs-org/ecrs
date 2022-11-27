use std::marker::PhantomData;

use crate::ga::{individual::Chromosome, GAMetadata, Individual};

use super::{Probe, ProbingPolicy};

/// ## PolicyDrivenProbe
///
/// Checks whether policy allows for logging and if so, delegates actual logging to wrapped probe
pub struct PolicyDrivenProbe<T: Chromosome, Pc: ProbingPolicy<T>, Pr: Probe<T>> {
	policy: Pc,
	probe: Pr,
	_phantom: PhantomData<T>, // FIXME: Is there a way to avoid it?
}

impl<T: Chromosome, Pc: ProbingPolicy<T>, Pr: Probe<T>> PolicyDrivenProbe<T, Pc, Pr> {
	/// Returns new instance of [PolicyDrivenProbe]
	///
	/// ### Arguments
	///
	/// * `policy` - logging policy to apply
	/// * `probe` - probe used to logging
	pub fn new(policy: Pc, probe: Pr) -> Self {
		Self {
			policy,
			probe,
			_phantom: PhantomData,
		}
	}
}

impl<T: Chromosome, Pc: ProbingPolicy<T>, Pr: Probe<T>> Probe<T> for PolicyDrivenProbe<T, Pc, Pr> {
  fn on_start(&mut self, metadata: &GAMetadata) { /* defaults to noop */
		if self.policy.on_start(metadata) {
			self.probe.on_start(metadata);
		}
  }

  fn on_initial_population_created(&mut self, population: &[Individual<T>]) {
		if self.policy.on_initial_population_created(population) {
			self.probe.on_initial_population_created(population);
		}
  }

  fn on_new_best(&mut self, metadata: &GAMetadata, individual: &Individual<T>) {
		if self.policy.on_new_best(metadata, individual) {
			self.probe.on_new_best(metadata, individual);
		}
  }

  fn on_new_generation(&mut self, generation: &[Individual<T>]) { /* defaults to noop */
	 if self.policy.on_new_generation(generation) {
			self.probe.on_new_generation(generation);
	 }
  }

  fn on_best_fit_in_generation(&mut self, metadata: &GAMetadata, individual: &Individual<T>) {
		if self.policy.on_best_fit_in_generation(metadata, individual) {
			self.probe.on_best_fit_in_generation(metadata, individual);
		}
  }

  fn on_iteration_start(&mut self, iteration: usize) { /* defaults to noop */
		if self.policy.on_iteration_start(iteration) {
			self.probe.on_iteration_start(iteration);
		}
  }

  fn on_iteration_end(&mut self, iteration: usize) { /* defaults to noop */
		if self.policy.on_iteration_end(iteration) {
			self.probe.on_iteration_end(iteration);
		}
  }

  fn on_end(
    &mut self,
    metadata: &GAMetadata,
    population: &[Individual<T>],
    best_individual: &Individual<T>)
	{
		if self.policy.on_end(metadata, population, best_individual) {
			self.probe.on_end(metadata, population, best_individual);
		}
  }
}
