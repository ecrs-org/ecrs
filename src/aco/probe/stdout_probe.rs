use crate::aco::probe::Probe;
use crate::aco::{AdditionalArgs, FMatrix, Solution};

/// Simple Probe implementation for writing algorithm output onto standard output.
pub struct StdoutProbe {
    iteration: usize,
}

impl StdoutProbe {
    /// Constructor
    pub fn new() -> Self {
        Self { iteration: 0 }
    }
}

impl<Args: AdditionalArgs> Probe<FMatrix, Args> for StdoutProbe {
    fn on_current_best(&mut self, best: &Solution, _: &Args) {
        println!("Iteration best fitness: {}", best.fitness);
    }

    fn on_iteration_start(&mut self, _: &Args) {
        self.iteration += 1;
        println!("--- ITERATION {} ---", self.iteration);
    }

    fn on_iteration_end(&mut self, _: &Args) {
        println!("################################");
    }

    fn on_end(&mut self, _: &Args) {
        println!("END")
    }
}
