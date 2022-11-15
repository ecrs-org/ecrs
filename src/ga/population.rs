use std::ops::Range;
use itertools::Itertools;
use rand::Rng;

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
				point.push(restriction.0 * rng.sample(distribution) + restriction.1);
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


#[cfg(test)]
mod tests {
    use crate::ga::individual::ChromosomeWrapper;

    use super::{RandomPoints, PopulationGenerator};

	#[test]
	fn points_have_appropriate_len() {
		let dim = 4;
		let gen = RandomPoints::new(dim, vec![(0.0..2.0), (-1.0..1.0), (3.0..10.0), (-5.0..-4.0)]);
		let points: Vec<crate::ga::Individual<Vec<f64>>> = gen.generate(30);

		for p in points {
			assert_eq!(p.get_chromosome().len(), dim)
		}
	}

	#[test]
	fn points_follow_restrictions() {
		let dim = 4;
		let restrictions = vec![(0.0..2.0), (-1.0..1.0), (3.0..10.0), (-5.0..-4.0)];
		let gen = RandomPoints::new(dim, restrictions.clone());
		let points: Vec<crate::ga::Individual<Vec<f64>>> = gen.generate(30);

		for p in points {
			for (v, res) in std::iter::zip(p.get_chromosome(), &restrictions) {
				assert!(res.contains(v))
			}
		}
	}
}
