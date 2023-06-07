//! Tools for primitive chromosomes generation. These are meant mainly for internal usage
//! by [population generators][PopulationGenerators] but they also might be of some help for you.
//!
//! [PopulationGenerators]: super::PopulationGenerator
use std::ops::Range;

use rand::{rngs::ThreadRng, thread_rng, Rng};

/// Random point generator. Just that.
pub struct PointGenerator<R: Rng = ThreadRng> {
    rng: R,
}

impl PointGenerator<ThreadRng> {
    /// Returns instance of [PointGenerator] with ThreadRng as default RNG.
    pub fn new() -> Self {
        Self::with_rng(thread_rng())
    }
}

impl<R: Rng + Clone> PointGenerator<R> {
    /// Returns new instance of [PointGenerator with given RNG.
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }

    /// Generates `n` random points, each with `dim` coordinates from range [0.0, 1.0).
    pub fn generate(&mut self, dim: usize, n: usize) -> Vec<Vec<f64>> {
        self.generate_with_single_constraint(dim, n, 0.0..1.0)
    }

    /// Generates `n` random points, each with `dim` coordinates from given range.
    pub fn generate_with_single_constraint(
        &mut self,
        dim: usize,
        n: usize,
        constraint: Range<f64>,
    ) -> Vec<Vec<f64>> {
        self.generate_with_constraints(dim, n, &Vec::from_iter(std::iter::repeat(constraint).take(dim)))
    }

    /// Generates `n` random points, each with `dim` coordinates. Each coordinate respects given
    /// constraint.
    pub fn generate_with_constraints(
        &mut self,
        dim: usize,
        n: usize,
        constraints: &Vec<Range<f64>>,
    ) -> Vec<Vec<f64>> {
        assert_eq!(dim, constraints.len());
        let distr = rand::distributions::Uniform::new(0.0, 1.0);
        Vec::from_iter((0..n).map(|_| {
            constraints
                .iter()
                .map(|constr| (constr.end - constr.start) * self.rng.sample(distr) + constr.start)
                .collect()
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::PointGenerator;

    #[test]
    fn points_have_correct_length() {
        let dim: usize = 40;
        let n: usize = 100;

        PointGenerator::new()
            .generate(dim, n)
            .into_iter()
            .for_each(|point| assert_eq!(point.len(), dim));
    }

    #[test]
    fn points_follow_implicit_constraints() {
        let dim: usize = 20;
        let n: usize = 100;

        PointGenerator::new()
            .generate(dim, n)
            .into_iter()
            .for_each(|point| point.iter().for_each(|x| assert!((0.0..1.0).contains(x))));
    }

    #[test]
    fn points_follow_single_constraint() {
        let dim: usize = 20;
        let n: usize = 100;
        let constraint = -5.0..15.0;

        PointGenerator::new()
            .generate_with_single_constraint(dim, n, constraint.clone())
            .into_iter()
            .for_each(|point| point.iter().for_each(|x| assert!(constraint.contains(x))));
    }

    #[test]
    fn points_follow_multiple_constraints() {
        let dim: usize = 20;
        let n: usize = 100;

        PointGenerator::new().generate_with_constraints(
            dim,
            n,
            &std::iter::repeat(-2.0..3.0)
                .take(dim / 2)
                .chain(std::iter::repeat(3.0..40.0).take(dim / 2))
                .collect(),
        );
    }

    #[test]
    fn right_number_of_points_is_generated() {
        let dim: usize = 20;
        let n: usize = 100;

        let count = PointGenerator::new().generate(dim, n).len();
        assert_eq!(count, n);

        let count = PointGenerator::new()
            .generate_with_constraints(dim, n, &std::iter::repeat(-1.0..1.0).take(dim).collect())
            .len();
        assert_eq!(count, n);

        let count = PointGenerator::new()
            .generate_with_single_constraint(dim, n, -2.0..2.0)
            .len();
        assert_eq!(count, n);
    }
}
