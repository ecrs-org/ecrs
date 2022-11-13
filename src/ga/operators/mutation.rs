use crate::ga::individual::{ChromosomeWrapper, Chromosome};

pub trait MutationOperator<T: Chromosome, S: ChromosomeWrapper<T>> {
	fn apply(&mut self, indivudial: &mut S);
}

pub struct Identity;

impl Identity {
	pub fn new() -> Self {
		Identity { }
	}
}

impl<T: Chromosome, S: ChromosomeWrapper<T>> MutationOperator<T, S> for Identity {
	fn apply(&mut self, _indivudial: &mut S) {}
}
