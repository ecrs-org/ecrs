use crate::ga::{individual::Chromosome, Individual};

pub trait MutationOperator<T: Chromosome> {
	fn apply(&mut self, indivudial: &mut Individual<T>);
}

pub struct Identity;

impl Identity {
	pub fn new() -> Self {
		Identity { }
	}
}

impl<T: Chromosome> MutationOperator<T> for Identity {
	fn apply(&mut self, _indivudial: &mut Individual<T>) {}
}
