//! Implementations of goodness calculation.
//!
//! Looking at equation for probability of ant choosing some edge
//! ((Scholarpedia)[http://www.scholarpedia.org/article/Ant_colony_optimization#ConstructAntSolutions])
//! we can precalculate every possible value of numerator, and use it to speed up calculations.
//! In this library we refer to this precalculated value as goodness, and this module contains
//! trait [Goodness] that must be implemented for every goodness calculating object and
//! implementations of aforementioned trait.
use crate::aco::FMatrix;

/// # Goodness
///
/// This trait must be implemented for goodness calculating struct.
pub trait Goodness {
  /// Calculates goodness based on pheromone and its own internal state
  ///
  /// ## Arguments
  /// `pheromone` - Pheromone in matrix representation.
  fn apply(&mut self, pheromone: &FMatrix) -> FMatrix;
}

/// # Canonical Goodness
///
/// Implements [Goodness]. Provides goodness calculation based on Ant System equations
pub struct CanonicalGoodness {
  alpha: f64,
  beta: f64,
  heuristic: FMatrix,
}

impl CanonicalGoodness {
  /// Creates a new instance of [CanonicalGoodness].
  ///
  /// ## Arguments
  /// * `alpha` - importance of weights in edge choosing.
  /// * `beta` - importance of heuristic in edge choosing.
  /// * `heuristic` - Weighted graph in matrix representation.
  pub fn new(alpha: f64, beta: f64, heuristic: FMatrix) -> Self {
    Self {
      alpha,
      beta,
      heuristic,
    }
  }
}

impl Goodness for CanonicalGoodness {
  fn apply(&mut self, pheromone: &FMatrix) -> FMatrix {
    let solution_size = pheromone.nrows();
    let iter = pheromone
      .iter()
      .zip(self.heuristic.iter())
      .map(|(p, h)| p.powf(self.alpha) * h.powf(self.beta));

    FMatrix::from_iterator(solution_size, solution_size, iter)
  }
}

#[cfg(test)]
mod tests {
  use crate::aco::goodness::{CanonicalGoodness, Goodness};
  use crate::aco::FMatrix;

  #[test]
  fn canonical_goodness_calculations_are_right() {
    let heuristic = FMatrix::from_vec(2, 2, vec![1.0, 2.0, 4.0, 8.0]);
    let alpha = 2.0;
    let beta = 3.0;
    let pheromone = FMatrix::from_vec(2, 2, vec![4.0, 2.0, 8.0, 0.5]);

    let goodness = vec![16.0, 32.0, 4096.0, 128.0];

    let mut g_op = CanonicalGoodness::new(alpha, beta, heuristic);
    for (a, b) in goodness.iter().zip(g_op.apply(&pheromone).iter()) {
      assert_eq!(a, b);
    }
  }
}
