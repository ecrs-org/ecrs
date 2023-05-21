use crate::ga::individual::IndividualTrait;

pub trait Fitness<IndividualT: IndividualTrait> {
    fn apply(&mut self, individual: &IndividualT) -> f64;
}

pub struct FnBasedFitness<IndividualT: IndividualTrait> {
    fn_ptr: fn(&IndividualT::ChromosomeT) -> f64,
}

impl<IndividualT: IndividualTrait> FnBasedFitness<IndividualT> {
    pub fn new(fn_ptr: fn(&IndividualT::ChromosomeT) -> f64) -> Self {
        FnBasedFitness { fn_ptr }
    }
}

impl<IndividualT: IndividualTrait> Fitness<IndividualT> for FnBasedFitness<IndividualT> {
    fn apply(&mut self, individual: &IndividualT) -> f64 {
        (self.fn_ptr)(individual.chromosome())
    }
}
