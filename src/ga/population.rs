use std::ops::Range;
use itertools::Itertools;
use rand::{Rng, rngs::ThreadRng};

use super::individual::{Chromosome, ChromosomeWrapper};

/// # Population generator
///
/// Implement this trait in order to provide custom population generator
/// and feed it to an solver.
pub trait PopulationGenerator<T: Chromosome, S: ChromosomeWrapper<T>> {
	fn generate(&self, count: usize) -> Vec<S>;
}

/// # Random points population generator
///
/// Implements [PopulationGenerator] trait. Can be used with genetic algorithm.
///
/// Generates vector of random points from R^(dim) space within passed domain restrictions.
pub struct RandomPoints {
	dim: usize,
	restrictions: Vec<(f64, f64)>
}

impl RandomPoints {
	pub fn new(dim: usize, restrictions: Vec<Range<f64>>) -> Self {
		assert!(dim > 0, "Space dimension must be > 0");
		assert_eq!(dim, restrictions.len(), "Number of restrictions must match dimension of sampled space");

		RandomPoints {
			dim,
			restrictions: restrictions.into_iter()
																.map(|range| (range.end - range.start, range.start))
																.collect_vec()
		}
	}
}

impl<S> PopulationGenerator<Vec<f64>, S> for RandomPoints
where
	S: ChromosomeWrapper<Vec<f64>>
{
	fn generate(&self, count: usize) -> Vec<S> {
		// FIXME: Sampling from such short interval may cause some f64 values to be more unlikely...
		let distribution = rand::distributions::Uniform::from(0.0..1.0);

		let mut population: Vec<S> = Vec::with_capacity(count);
		let rng = &mut rand::thread_rng();

		for _ in 0..count {
			let mut point: Vec<f64> = Vec::with_capacity(self.dim);
			for restriction in &self.restrictions {
				point.push(restriction.1 * rng.sample(distribution) + restriction.0);
			}
			population.push(S::from(point));
		}

		population
	}
}

pub struct BitStrings {
	dim: usize,
}

impl BitStrings {
	pub fn new(dim: usize) -> Self {
		assert!(dim > 0, "Space dimension must be > 0");
		BitStrings { dim }
	}
}

impl<S> PopulationGenerator<Vec<bool>, S> for BitStrings
where
	S: ChromosomeWrapper<Vec<bool>>
{
	fn generate(&self, count: usize) -> Vec<S> {
		let mut population: Vec<S> = Vec::with_capacity(count);

		let distr = rand::distributions::Uniform::from(0.0..1.0);
		let rng = &mut rand::thread_rng();

		for _ in 0..count {
			population.push(S::from(rng.sample_iter(distr).take(self.dim).map(|v| v < 0.5).collect_vec()));
		}

		population
	}
}
