use itertools::Itertools;
use rand::Rng;
use std::ops::Range;

use super::{individual::Chromosome, Individual};

/// Implement this trait in order to provide custom population generator
/// and feed it to an solver.
pub trait PopulationGenerator<T: Chromosome> {
  fn generate(&self, count: usize) -> Vec<Individual<T>>;
}

/// Implements [PopulationGenerator] trait. Can be used with genetic algorithm.
///
/// Generates vector of random points from R^(dim) space within passed domain constraints.
pub struct RandomPoints {
  dim: usize,
  constraints: Vec<(f64, f64)>,
}

impl RandomPoints {
  /// Returns [RandomPoints] population generator with given constraints
  ///
  /// ### Arguments
  ///
  /// * `dim` -- Dimension of the sampling space
  /// * `constraints` -- Ranges for coordinates
  pub fn with_constraints(dim: usize, constraints: Vec<Range<f64>>) -> Self {
    assert!(dim > 0, "Space dimension must be > 0");
    assert_eq!(
      dim,
      constraints.len(),
      "Number of constraints must match dimension of sampled space"
    );

    RandomPoints {
      dim,
      constraints: constraints
        .into_iter()
        .map(|range| (range.end - range.start, range.start))
        .collect_vec(),
    }
  }

	/// Returns [RandomPoints] population generator with no explicit constraints.
	/// Points coords will be from range 0.0..1.0.
	///
	/// ### Arguments
	///
	/// * `dim` - Dimension of the sampling space
	pub fn new(dim: usize) -> Self {
		assert!(dim > 0, "Space dimension must be > 0");
		RandomPoints { dim, constraints: Vec::<(f64, f64)>::with_capacity(0) }
	}
}

impl PopulationGenerator<Vec<f64>> for RandomPoints {
  /// Generates vector of `count` random points from R^(dim) space within passed domain constraints.
	/// If there were no constraints passed then the points coords are from range 0.0..1.0.
  ///
  /// ### Arguments
  ///
  /// * `count` -- Number of points to generate
  fn generate(&self, count: usize) -> Vec<Individual<Vec<f64>>> {
    // FIXME: Sampling from such short interval may cause some f64 values to be more unlikely...
    let distribution = rand::distributions::Uniform::from(0.0..1.0);

    let mut population: Vec<Individual<Vec<f64>>> = Vec::with_capacity(count);
    let rng = &mut rand::thread_rng();

		// We do not use Option to designate whether there are constraints or not
		// because using unwrap moves!
		if self.constraints.is_empty() {
			for _ in 0..count {
				let mut point = Vec::<f64>::with_capacity(self.dim);
				for _ in 0..self.dim {
					point.push(rng.sample(distribution));
				}
				population.push(Individual::from(point));
			}
		} else {
			for _ in 0..count {
				let mut point: Vec<f64> = Vec::with_capacity(self.dim);
				for restriction in &self.constraints {
					point.push(restriction.0 * rng.sample(distribution) + restriction.1);
				}
				population.push(Individual::from(point));
			}
		}
    population
  }
}

/// Implements [PopulationGenerator] trait. Can be used with genetic algorithm.
///
/// Generates vector of random bit-strings.
pub struct BitStrings {
  dim: usize,
}

impl BitStrings {
  /// Returns [BitString] population generator
  ///
  /// ### Arguments
  ///
  /// * `dim` -- Dimension of the sampling space
  pub fn new(dim: usize) -> Self {
    assert!(dim > 0, "Space dimension must be > 0");
    BitStrings { dim }
  }
}

impl PopulationGenerator<Vec<bool>> for BitStrings {
  /// Generates vector of `count` random bitstrings
  ///
  /// ### Arguments
  ///
  /// * `count` -- Number of bitstrings to generate
  fn generate(&self, count: usize) -> Vec<Individual<Vec<bool>>> {
    let mut population: Vec<Individual<Vec<bool>>> = Vec::with_capacity(count);

    let distr = rand::distributions::Uniform::from(0.0..1.0);
    let rng = &mut rand::thread_rng();

    for _ in 0..count {
      population.push(Individual::from(
        rng
          .sample_iter(distr)
          .take(self.dim)
          .map(|v| v < 0.5)
          .collect_vec(),
      ));
    }

    population
  }
}

#[cfg(test)]
mod tests {
  use super::{BitStrings, PopulationGenerator, RandomPoints};

  #[test]
  fn points_have_appropriate_len() {
    let dim = 4;
    let gen = RandomPoints::with_constraints(dim, vec![(0.0..2.0), (-1.0..1.0), (3.0..10.0), (-5.0..-4.0)]);
    let points: Vec<crate::ga::Individual<Vec<f64>>> = gen.generate(30);

    for p in points {
      assert_eq!(p.chromosome.len(), dim)
    }
  }

  #[test]
  fn points_follow_explicit_constraints() {
    let dim = 4;
    let constraints = vec![(0.0..2.0), (-1.0..1.0), (3.0..10.0), (-5.0..-4.0)];
    let gen = RandomPoints::with_constraints(dim, constraints.clone());
    let points: Vec<crate::ga::Individual<Vec<f64>>> = gen.generate(30);

    for p in points {
      for (v, res) in std::iter::zip(p.chromosome_ref(), &constraints) {
        assert!(res.contains(v));
      }
    }
  }

	#[test]
	fn points_follow_implicit_constraints() {
		let dim = 30;
		let count = 100;

		let gen = RandomPoints::new(dim);
		let points: Vec<crate::ga::Individual<Vec<f64>>> = gen.generate(count);

		for p in points {
			for v in p.chromosome_ref() {
				assert!((0.0..1.0).contains(v));
			}
		}
	}

  #[test]
  fn bistrings_have_appropriate_len() {
    let dim = 30;
    let gen = BitStrings::new(dim);
    let points: Vec<crate::ga::Individual<Vec<bool>>> = gen.generate(30);

    for p in points {
      assert_eq!(p.chromosome_ref().len(), dim)
    }
  }
}
