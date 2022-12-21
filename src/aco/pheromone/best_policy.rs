//! Module contains implementations of policies on how to choose the best ant
//!
//! There are [PheromoneUpdate] implementations that make use of best found solution.
//! This module was create to accommodate two main choosing policies
//! * [OverallBest] - chooses the best in all iterations
//! * [Iteration] - chooses the best in current iteration
//!
use crate::aco::{FMatrix, Solution};

/// # Best Choosing Policy
///
/// This trait defines common behaviors of choosing the best ant.
/// You can implement this trait to provide your own choosing policy
pub trait BestPolicy {
  /// Based on provided solutions, it updates it internal state.
  ///
  /// ## Arguments
  /// * `solutions` - current iteration solutions.
  fn update_best(&mut self, solutions: &[Solution]);

  /// Returns stored best pheromone trail
  fn get_best_pheromone(&self) -> &FMatrix;
}

/// # Iteration best choosing policy
/// Implements [ChoosingPolicy].
/// Chooses pheromone from current iteration best ant.
pub struct IterationBest {
  best_pheromone: FMatrix,
}

impl IterationBest {
  /// Crates a new instance of [IterationBest]
  pub fn new() -> Self {
    Self {
      best_pheromone: FMatrix::zeros(0, 0),
    }
  }
}

impl BestPolicy for IterationBest {
  fn update_best(&mut self, solutions: &[Solution]) {
    let iter_best = find_best(solutions);
    self.best_pheromone = iter_best.matrix.scale(1.0 / iter_best.cost);
  }

  fn get_best_pheromone(&self) -> &FMatrix {
    &self.best_pheromone
  }
}

/// # Overall best choosing policy
/// Implements [ChoosingPolicy].
/// Chooses pheromone from all past iteration best ant.
pub struct OverallBest {
  best_pheromone: FMatrix,
  cost: f64,
}

impl OverallBest {
  /// Crates a new instance of [OverallBest]
  pub fn new() -> Self {
    Self {
      best_pheromone: FMatrix::zeros(0, 0),
      cost: f64::MAX,
    }
  }
}

impl BestPolicy for OverallBest {
  fn update_best(&mut self, solutions: &[Solution]) {
    let iter_best = find_best(solutions);

    if iter_best.cost < self.cost {
      self.best_pheromone = iter_best.matrix.scale(1.0 / iter_best.cost);
      self.cost = iter_best.cost;
    }
  }

  fn get_best_pheromone(&self) -> &FMatrix {
    &self.best_pheromone
  }
}

#[inline]
fn find_best(solutions: &[Solution]) -> &Solution {
  solutions
    .iter()
    .reduce(|a, b| if a.cost > b.cost { b } else { a })
    .unwrap()
}

#[cfg(test)]
mod tests {
  use crate::aco::pheromone::best_policy::{BestPolicy, IterationBest, OverallBest};
  use crate::aco::{FMatrix, Solution};

  #[test]
  fn iteration_best_returns_correct_pheromones() {
    let gen1 = [
      Solution {
        matrix: FMatrix::repeat(1, 1, 1.0),
        path: vec![1],
        cost: 2.0,
        fitness: 0.5,
      },
      Solution {
        matrix: FMatrix::repeat(1, 1, 1.0),
        path: vec![1],
        cost: 4.0,
        fitness: 0.25,
      },
    ];

    let gen2 = [
      Solution {
        matrix: FMatrix::repeat(1, 1, 1.0),
        path: vec![1],
        cost: 8.0,
        fitness: 0.125,
      },
      Solution {
        matrix: FMatrix::repeat(1, 1, 1.0),
        path: vec![1],
        cost: 16.0,
        fitness: 0.625,
      },
    ];

    let mut best_pol = IterationBest::new();
    best_pol.update_best(&gen1);
    let pheromone = best_pol.get_best_pheromone();

    assert_eq!(pheromone[0], 0.5);

    best_pol.update_best(&gen2);
    let pheromone = best_pol.get_best_pheromone();

    assert_eq!(pheromone[0], 0.125);
  }

  #[test]
  fn overall_best_returns_correct_pheromones() {
    let gen1 = [
      Solution {
        matrix: FMatrix::repeat(1, 1, 1.0),
        path: vec![1],
        cost: 2.0,
        fitness: 0.5,
      },
      Solution {
        matrix: FMatrix::repeat(1, 1, 1.0),
        path: vec![1],
        cost: 4.0,
        fitness: 0.25,
      },
    ];

    let gen2 = [
      Solution {
        matrix: FMatrix::repeat(1, 1, 1.0),
        path: vec![1],
        cost: 8.0,
        fitness: 0.125,
      },
      Solution {
        matrix: FMatrix::repeat(1, 1, 1.0),
        path: vec![1],
        cost: 16.0,
        fitness: 0.625,
      },
    ];

    let mut best_pol = OverallBest::new();
    best_pol.update_best(&gen1);
    let pheromone = best_pol.get_best_pheromone();

    assert_eq!(pheromone[0], 0.5);

    best_pol.update_best(&gen2);
    let pheromone = best_pol.get_best_pheromone();

    assert_eq!(pheromone[0], 0.5);
  }
}
