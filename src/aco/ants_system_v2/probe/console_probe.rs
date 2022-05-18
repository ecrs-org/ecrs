use crate::aco::ants_system_v2::probe::Probe;
use crate::aco::ants_system_v2::Solution;
use crate::FMatrix;

pub struct ConsoleProbe{
}

impl Probe for ConsoleProbe{
    fn on_new_best(&mut self, best_sol: &Solution) {
        println!("New best!!!");
    }

    fn on_pheromone_update(&mut self, old_pheromone: &FMatrix, new_pheromone: &FMatrix) {

    }

    fn on_current_best(&mut self, best: &Solution) {
        println!("Iteration best: {}", best.cost);
    }

    fn on_iteration_start(&mut self, iteration: usize) {
        println!("--- ITERATION {} ---", iteration);
    }

    fn on_iteration_end(&mut self, iteration: usize) {
        println!("################################");
    }

    fn on_end(&mut self) {
        println!("END")
    }
}

impl ConsoleProbe {

    pub fn new() -> ConsoleProbe {
        ConsoleProbe{
        }
    }

}