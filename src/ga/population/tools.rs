use std::ops::Range;

use itertools::Itertools;
use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct PointGenerator<R: Rng = ThreadRng> {
    rng: R,
}

impl PointGenerator<ThreadRng> {
    pub fn new() -> Self {
        Self::with_rng(thread_rng())
    }
}

impl<R: Rng + Clone> PointGenerator<R> {
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }

    pub fn generate(&self, dim: usize, n: usize) -> Vec<Vec<f64>> {
        self.generate_with_single_constraint(dim, n, 0.0..1.0)
    }

    pub fn generate_with_single_constraint(
        &self,
        dim: usize,
        n: usize,
        constraint: Range<f64>,
    ) -> Vec<Vec<f64>> {
        self.generate_with_constraints(dim, n, &Vec::from_iter(std::iter::repeat(constraint)))
    }

    pub fn generate_with_constraints(
        &self,
        dim: usize,
        n: usize,
        constraints: &Vec<Range<f64>>,
    ) -> Vec<Vec<f64>> {
        assert_eq!(n, constraints.len());
        Vec::from_iter(constraints.iter().map(|constr| {
            self.rng
                .clone()
                .sample_iter(rand::distributions::Uniform::new(constr.start, constr.end))
                .take(dim)
                .collect_vec()
        }))
    }
}
