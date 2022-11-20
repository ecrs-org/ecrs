use crate::ga::{individual::Chromosome, Individual};

pub trait MutationOperator<T: Chromosome> {
	fn apply(&self, indivudial: &mut Individual<T>);
}

pub struct Identity;

impl Identity {
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
