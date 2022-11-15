use num::iter::Range;

use super::individual::{Chromosome, ChromosomeWrapper};

/// # Population generator
///
/// Implement this trait in order to provide custom population generator
/// and feed it to an solver.
pub trait PopulationGenerator<T: Chromosome, S: ChromosomeWrapper<T>> {
	fn generate(&self, count: usize) -> Vec<S>;
}

pub struct RandomPoints<T> {
	dim: usize,
	restrictions: Vec<Range<T>>
}

impl<T> RandomPoints<T> {
	pub fn new(dim: usize, restrictions: Vec<Range<T>>) -> Self {
		assert_eq!(dim, restrictions.len(), "Number of restrictions must match dimension of sampled space");
		RandomPoints { dim, restrictions }
	}
}

impl<S, R> PopulationGenerator<Vec<f64>, S> for RandomPoints<R>
where
	S: ChromosomeWrapper<Vec<f64>>
{
	fn generate(&self, count: usize) -> Vec<S> {

	}
}

