pub use console_probe::ConsoleProbe;
pub use csv_probe::CsvProbe;

use crate::aco::ants_system_v2::Solution;
use crate::FMatrix;

mod console_probe;
mod csv_probe;

pub trait Probe {
    fn on_new_best(&mut self, best_sol: &Solution);
    fn on_pheromone_update(&mut self, old_pheromone: &FMatrix, new_pheromone: &FMatrix);
    fn on_current_best(&mut self, best: &Solution);
    fn on_iteration_start(&mut self, iteration: usize);
    fn on_iteration_end(&mut self, iteration: usize);
    fn on_end(&mut self);
}