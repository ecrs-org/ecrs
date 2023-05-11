use crate::ff::probe::Probe;

pub struct StdoutProbe {}

impl StdoutProbe {
    pub fn new() -> StdoutProbe {
        StdoutProbe {}
    }
}

impl Probe for StdoutProbe {
    fn on_start(&mut self) {
        println!("START");
    }
    fn on_iteration_start(&mut self, iteration: u32) {
        println!("--- ITERATION {iteration} ---");
    }
    fn on_iteration_end(&mut self, _iteration: u32) {
        println!("################################");
    }
    fn on_current_best(&mut self, solution: f64, _position: &[f64]) {
        println!("Iteration best: {solution}");
        for (dim, pos) in _position.iter().enumerate() {
            println!("X{dim}: {pos}",)
        }
    }
    fn on_end(&mut self) {
        println!("END");
    }
}
