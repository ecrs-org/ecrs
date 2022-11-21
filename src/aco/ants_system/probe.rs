//! Module contains probes for getting data from Ant System algorithm.
//!
//! To get data fom algorithm you must pass some kind of probe to AntSystem class.
//! you can implement your own probe using [Probe](Probe) trait or use one of provided implementations.

pub use console_probe::ConsoleProbe;
pub use csv_probe::CsvProbe;

use crate::aco::ants_system::Solution;
use crate::aco::FMatrix;

mod console_probe;
mod csv_probe;

pub trait Probe {
  /// Called when algorithm finds a new best solution overall.
  fn on_new_best(&mut self, best_sol: &Solution);
  /// Called when new pheromone has been calculated.
  fn on_pheromone_update(&mut self, old_pheromone: &FMatrix, new_pheromone: &FMatrix);
  /// Called every iteration with best solution in current iteration.
  fn on_current_best(&mut self, best: &Solution);
  /// Called on iteration begin when no process has started yet.
  fn on_iteration_start(&mut self, iteration: usize);
  /// Called on iteration end when all tasks of this iteration are completed.
  fn on_iteration_end(&mut self, iteration: usize);
  /// Called when algorithm has ended
  fn on_end(&mut self);
}
