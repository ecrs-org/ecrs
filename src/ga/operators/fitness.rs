use crate::ga::individual::IndividualTrait;

pub trait Fitness<IndividualT: IndividualTrait> {
    fn apply(&mut self, individual: &IndividualT) -> IndividualT::FitnessValueT;
}

pub struct FnBasedFitness<IndividualT: IndividualTrait> {
    fn_ptr: fn(&IndividualT::ChromosomeT) -> IndividualT::FitnessValueT,
}

impl<IndividualT: IndividualTrait> FnBasedFitness<IndividualT> {
    pub fn new(fn_ptr: fn(&IndividualT::ChromosomeT) -> IndividualT::FitnessValueT) -> Self {
        FnBasedFitness { fn_ptr }
    }
}

impl<IndividualT: IndividualTrait> Fitness<IndividualT> for FnBasedFitness<IndividualT> {
    fn apply(&mut self, individual: &IndividualT) -> IndividualT::FitnessValueT {
        (self.fn_ptr)(individual.chromosome())
    }
}
