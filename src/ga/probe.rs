use crate::ga::Individual;

use super::individual::{ChromosomeWrapper, Chromosome};

pub mod stdout_probe;
pub mod csv_probe;
pub mod json_probe;

pub trait Probe<T: Chromosome, S: ChromosomeWrapper<T>> {
  fn on_start(&mut self) { /* defaults to noop */ }
  fn on_new_best(&mut self, individual: &S) { /* defaults to noop */ }
  fn on_mutation(&mut self, before: &S, after: &S) { /* defaults to noop */ }
  fn on_new_generation(&mut self, generation: &Vec<S>) { /* defaults to noop */ }
  fn on_best_fit_in_generation(&mut self, individual: &S) { /* defaults to noop */ }
  fn on_iteration_start(&mut self, iteration: usize) { /* defaults to noop */ }
  fn on_iteration_end(&mut self, iteration: usize) { /* defaults to noop */ }
  fn on_end(&mut self) { /* defaults to noop */ }
}

