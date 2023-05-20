use ecrs::prelude::fitness::Fitness;

struct JsspFitness {}

impl JsspFitness {
    fn new() -> Self {
        Self {}
    }
}

impl Fitness<Vec<f64>> for JsspFitness {
    fn apply(&mut self, chromosome: &Vec<f64>) -> f64 {
        // chromosome.
        todo!()
    }
}

