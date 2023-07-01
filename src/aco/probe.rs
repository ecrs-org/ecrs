//! Module contains probes for getting data from Ant System algorithm.
//!
//! To get data fom algorithm you must pass some kind of probe to AntSystem class.
//! you can implement your own probe using [Probe](Probe) trait or use one of provided implementations.

use crate::aco::Solution;

mod aggregated_probe;
mod csv_probe;
mod json_probe;
mod policy_driven_probe;
mod probing_policy;
mod stdout_probe;

use crate::aco::pheromone::Pheromone;
pub use aggregated_probe::AggregatedProbe;
pub use csv_probe::CsvProbe;
pub use json_probe::JsonProbe;
pub use policy_driven_probe::PolicyDrivenProbe;
pub use probing_policy::{ElapsedTime, GenerationInterval};
pub use stdout_probe::StdoutProbe;

#[allow(unused_variables)]
pub trait Probe<P: Pheromone> {
    /// Called when new pheromone has been calculated.
    fn on_pheromone_update(&mut self, new_pheromone: &P) {}
    /// Called every iteration with best solution in current iteration.
    fn on_current_best(&mut self, best: &Solution) {}
    /// Called on iteration begin when no process has started yet.
    fn on_iteration_start(&mut self) {}
    /// Called on iteration end when all tasks of this iteration are completed.
    fn on_iteration_end(&mut self) {}
    /// Called when algorithm has ended
    fn on_end(&mut self) {}
}

pub trait ProbingPolicy {
    fn on_pheromone_update(&mut self) -> bool;
    fn on_current_best(&mut self) -> bool;
    fn on_iteration_start(&mut self) -> bool;
    fn on_iteration_end(&mut self) -> bool;
    fn on_end(&mut self) -> bool;
}
