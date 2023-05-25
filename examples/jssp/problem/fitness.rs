#![allow(dead_code)]
use ecrs::prelude::fitness::Fitness;

use super::individual::JsspIndividual;

pub struct JsspFitness {}

impl JsspFitness {
    fn new() -> Self {
        Self {}
    }
}

impl Fitness<JsspIndividual> for JsspFitness {
    fn apply(&mut self, individual: &mut JsspIndividual) -> usize {
        individual.eval()
    }
}
