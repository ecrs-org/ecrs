use ecrs::prelude::fitness::Fitness;

use super::individual::JsspIndividual;

pub struct JsspFitness {}

impl JsspFitness {
    fn new() -> Self {
        Self {}
    }
}

impl Fitness<JsspIndividual> for JsspFitness {
    fn apply(&mut self, individual: &mut JsspIndividual) -> f64 {
        individual.eval() as f64
    }
}
