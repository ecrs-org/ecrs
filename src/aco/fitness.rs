use crate::aco::FMatrix;
use itertools::Itertools;

pub trait Fitness {
  fn apply(&mut self, path: &[usize]) -> f64;
}

pub struct CanonicalFitness {
  weights: FMatrix,
}

impl CanonicalFitness {
  pub fn new(weights: FMatrix) -> Self {
    Self { weights }
  }
}

impl Fitness for CanonicalFitness {
  fn apply(&mut self, path: &[usize]) -> f64 {
    let mut cost = 0.0f64;
    for (i, j) in path.iter().circular_tuple_windows::<(&usize, &usize)>() {
      cost += self.weights[(*i, *j)];
    }

    1.0 / cost
  }
}
