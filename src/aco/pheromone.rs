//! Implementation of pheromone calculations strategies.
//!
use crate::aco::pheromone::best_policy::BestPolicy;
use crate::aco::{FMatrix, Solution};
use std::ops::Add;

mod best_policy;

/// # Pheromone Update
///
/// This trait defines common behaviour for pheromone update calculations.
/// You can implement this trait to provide your custom way of calculating new pheromone to the ACO.
pub trait PheromoneUpdate {
  /// Returns the new pheromone
  ///
  /// ## Arguments
  ///
  /// * `old_pheromone` - Pheromone used to generate current solutions
  /// * `solutions` - Current generated solution.
  /// * `evaporation_rate` - rate of old pheromone evaporation
  fn apply(&mut self, old_pheromone: &FMatrix, solutions: &[Solution], evaporation_rate: f64) -> FMatrix;
}

/// # Ant System Pheromone Update
///
/// Implements [PheromoneUpdate]. The pheromone is updated as first proposed by Marco Dorigo,
/// every ant leaves pheromone trail on its way, the pheromone trail strength is inversely proportional
/// to the way cost. New pheromone is a sum of old pheromone scaled by (1 - evaporation rate) and sum
/// of pheromone trails left by ants.
pub struct AntSystemPU;

impl AntSystemPU {
  /// Creates a new instance of [AntSystemPU]
  pub fn new() -> Self {
    AntSystemPU
  }
}

impl PheromoneUpdate for AntSystemPU {
  fn apply(&mut self, old_pheromone: &FMatrix, solutions: &[Solution], evaporation_rate: f64) -> FMatrix {
    let delta_pheromone = scale_and_sum(solutions);

    old_pheromone.scale(1.0 - evaporation_rate).add(delta_pheromone)
  }
}

/// # Elitist Ant System Pheromone Update
///
/// Implements [PheromoneUpdate]. Similarity to [AntSystemPU], every ant leaves pheromone trail on its way,
/// the pheromone trail strength is inversely proportional
/// to the way cost. New pheromone is a sum of old pheromone scaled by (1 - evaporation rate) and sum
/// of pheromone trails left by ants, additionally we are adding pheromone left by the best ant overall.
pub struct ElitistAntSystemPU {
  best_solution_pheromone: FMatrix,
  best_solution_cost: f64,
}

impl ElitistAntSystemPU {
  /// Creates a new instance of [ElitistAntSystemPU]
  pub fn new() -> Self {
    ElitistAntSystemPU {
      best_solution_pheromone: FMatrix::zeros(0, 0),
      best_solution_cost: f64::MAX,
    }
  }

  fn update_best(&mut self, solutions: &[Solution]) {
    let iter_best = solutions
      .iter()
      .reduce(|a, b| if a.cost > b.cost { b } else { a })
      .unwrap();

    if self.best_solution_cost > iter_best.cost {
      self.best_solution_cost = iter_best.cost;
      self.best_solution_pheromone = iter_best.matrix.scale(1.0 / iter_best.cost);
    }
  }
}

impl PheromoneUpdate for ElitistAntSystemPU {
  fn apply(&mut self, old_pheromone: &FMatrix, solutions: &[Solution], evaporation_rate: f64) -> FMatrix {
    self.update_best(solutions);
    let delta_pheromone = scale_and_sum(solutions);

    old_pheromone
      .scale(1.0 - evaporation_rate)
      .add(delta_pheromone)
      .add(&self.best_solution_pheromone)
  }
}

/// # MAX-MIN Ant System Pheromone Update
///
/// Implements [PheromoneUpdate].
/// the pheromone trail strength is inversely proportional
/// to the way cost. New pheromone is a sum of old pheromone scaled by (1 - evaporation rate) and
/// pheromone trail left by ant chosen by [BestPolicy], additionally the pheromone value is clamped.
struct MMAntSystemPU<B: BestPolicy> {
  best_policy: B,
  lower_bound: f64,
  upper_bound: f64,
}

impl<B: BestPolicy> MMAntSystemPU<B> {
  /// Creates an [MMAntSystemPU] with user provided implementation of [BestPolicy].
  ///
  /// ## Arguments
  /// * `lower_bound` - Minimal possible pheromone value.
  /// * `upper_bound` - Maximal possible pheromone value.
  /// * `best_policy` - Implementation of [BestPolicy]
  pub fn with_best_policy(lower_bound: f64, upper_bound: f64, best_policy: B) -> Self {
    assert!(lower_bound > 0.0, "Lower bound must be grater than 0");
    assert!(
      upper_bound > lower_bound,
      "Lower bound must be smaller than upper bound"
    );

    Self {
      lower_bound,
      upper_bound,
      best_policy,
    }
  }
}

impl MMAntSystemPU<best_policy::OverallBest> {
  /// Creates an [MMAntSystemPU] with [best_policy::OverallBest] best ant choosing policy
  ///
  /// ## Arguments
  /// * `lower_bound` - Minimal possible pheromone value.
  /// * `upper_bound` - Maximal possible pheromone value.
  pub fn new(lower_bound: f64, upper_bound: f64) -> Self {
    Self::with_best_policy(lower_bound, upper_bound, best_policy::OverallBest::new())
  }
}

impl<B: BestPolicy> PheromoneUpdate for MMAntSystemPU<B> {
  fn apply(&mut self, old_pheromone: &FMatrix, solutions: &[Solution], evaporation_rate: f64) -> FMatrix {
    self.best_policy.update_best(solutions);
    let best_pheromone = self.best_policy.get_best_pheromone();

    old_pheromone
      .scale(1.0 - evaporation_rate)
      .add(best_pheromone)
      .map(|a| a.clamp(self.lower_bound, self.upper_bound))
  }
}

struct AntColonySystemPU<B: BestPolicy> {
  best_policy: B,
}

impl AntColonySystemPU<best_policy::OverallBest> {
  pub fn new() -> Self {
    Self {
      best_policy: best_policy::OverallBest::new(),
    }
  }
}

impl<B: BestPolicy> AntColonySystemPU<B> {
  pub fn with_policy(best_policy: B) -> Self {
    Self { best_policy }
  }
}

impl<B: BestPolicy> PheromoneUpdate for AntColonySystemPU<B> {
  fn apply(&mut self, old_pheromone: &FMatrix, solutions: &[Solution], evaporation_rate: f64) -> FMatrix {
    self.best_policy.update_best(solutions);
    let best_pheromone = self.best_policy.get_best_pheromone();

    old_pheromone
      .scale(1.0 - evaporation_rate)
      .add(best_pheromone.scale(evaporation_rate))
  }
}

#[inline]
fn scale_and_sum(solutions: &[Solution]) -> FMatrix {
  solutions
    .iter()
    .map(|sol| sol.matrix.scale(1.0 / sol.cost))
    .reduce(|s1, s2| s1.add(s2))
    .expect("pheromone update creation error")
}

#[cfg(test)]
mod tests {
  use crate::aco::pheromone::{
    AntColonySystemPU, AntSystemPU, ElitistAntSystemPU, MMAntSystemPU, PheromoneUpdate,
  };
  use crate::aco::{FMatrix, Solution};

  #[test]
  fn check_ant_system_pu_with_example() {
    let pheromone = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 2.0, 1.0, 0.0, 4.0, 2.0, 4.0, 0.0]);

    let s1 = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
    let s2 = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
    let sols = [
      Solution {
        matrix: s1,
        cost: 8.0,
      },
      Solution {
        matrix: s2,
        cost: 4.0,
      },
    ];

    let mut pu = AntSystemPU;
    let new_pheromone = pu.apply(&pheromone, &sols, 0.25);
    let pheromone = vec![0.0, 1.125, 1.875, 1.125, 0.0, 3.375, 1.875, 3.375, 0.0];

    for (p, p_exp) in new_pheromone.iter().zip(pheromone.iter()) {
      assert_eq!(p, p_exp);
    }
  }

  #[test]
  fn check_elitist_ant_system_pu_with_example() {
    let pheromone = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 2.0, 1.0, 0.0, 4.0, 2.0, 4.0, 0.0]);

    let s1 = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
    let s2 = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
    let sols = [
      Solution {
        matrix: s1,
        cost: 8.0,
      },
      Solution {
        matrix: s2,
        cost: 4.0,
      },
    ];

    let mut pu = ElitistAntSystemPU::new();
    let new_pheromone = pu.apply(&pheromone, &sols, 0.25);
    let pheromone = vec![0.0, 1.375, 2.125, 1.375, 0.0, 3.625, 2.125, 3.625, 0.0];

    for (p, p_exp) in new_pheromone.iter().zip(pheromone.iter()) {
      assert_eq!(p, p_exp);
    }
  }

  #[test]
  fn check_max_min_ant_system_pu_with_example() {
    let pheromone = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 2.0, 1.0, 0.0, 4.0, 2.0, 4.0, 0.0]);

    let s1 = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
    let s2 = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
    let sols = [
      Solution {
        matrix: s1,
        cost: 8.0,
      },
      Solution {
        matrix: s2,
        cost: 4.0,
      },
    ];

    let mut pu = MMAntSystemPU::new(1.5, 3.0);
    let new_pheromone = pu.apply(&pheromone, &sols, 0.25);
    let pheromone = vec![1.5, 1.5, 1.75, 1.5, 1.5, 3.0, 1.75, 3.0, 1.5];

    for (p, p_exp) in new_pheromone.iter().zip(pheromone.iter()) {
      assert_eq!(p, p_exp);
    }
  }

  #[test]
  fn check_ant_colony_system_pu_with_example() {
    let pheromone = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 2.0, 1.0, 0.0, 4.0, 2.0, 4.0, 0.0]);

    let s1 = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
    let s2 = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
    let sols = [
      Solution {
        matrix: s1,
        cost: 8.0,
      },
      Solution {
        matrix: s2,
        cost: 4.0,
      },
    ];

    let mut pu = AntColonySystemPU::new();
    let new_pheromone = pu.apply(&pheromone, &sols, 0.25);
    let pheromone = vec![0.0, 0.8125, 1.5625, 0.8125, 0.0, 3.0625, 1.5625, 3.0625, 0.0];

    for (p, p_exp) in new_pheromone.iter().zip(pheromone.iter()) {
      assert_eq!(p, p_exp);
    }
  }
}
