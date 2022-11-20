use crate::ga::{individual::Chromosome, Individual};

/// # Mutation Operator
///
/// This trait defines common behaviour for mutation operators.
/// You can implement this trait to provide your custom crossover operator to the GA.
pub trait MutationOperator<T: Chromosome> {
	/// Mutates provided solution in place
	///
	/// ## Arguments
	///
	/// * `individual` - mutable reference to to-be-mutated individual
	fn apply(&mut self, indivudial: &mut Individual<T>);
}

/// # Identity Mutation Operator
///
/// This struct implements [MutationOperator] trait and can be used with GA.
///
/// Identity does not perform any changes to the chromosome. Use this if you
/// do not want to mutate your solutions.
pub struct Identity;

impl Identity {
	/// Returns new [Identity] mutation operator
	pub fn new() -> Self {
		Identity { }
	}
}

impl<T: Chromosome> MutationOperator<T> for Identity {
	fn apply(&mut self, _indivudial: &mut Individual<T>) {}
}
