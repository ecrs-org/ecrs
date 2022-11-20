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
	fn apply(&self, _indivudial: &mut Individual<T>) {}
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::{Rng, distributions::Uniform};
    use crate::ga::Individual;

    use super::{Identity, MutationOperator};

	#[test]
	fn identity_does_not_change_chromosome() {
		let chromosome = rand::thread_rng()
			.sample_iter(Uniform::from(-1.0..1.0))
			.take(30).collect_vec();

		let mut individual = Individual { chromosome: chromosome.clone(), fitness: f64::default( )};

		let identity_mutation = Identity;

		identity_mutation.apply(&mut individual);

		assert_eq!(chromosome, individual.chromosome);
	}
}
