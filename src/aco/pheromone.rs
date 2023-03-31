//! Implementation of pheromone calculations strategies.
//!
use std::ops::AddAssign;
use crate::aco::pheromone::best_policy::{BestPolicy, OverallBest};
use crate::aco::{FMatrix, FMatrixArray, Solution};
use itertools::Itertools;

pub mod best_policy;

pub trait Pheromone {}

impl Pheromone for FMatrix {}

impl Pheromone for FMatrixArray {}



/// # Pheromone Update
///
/// This trait defines common behaviour for pheromone update calculations.
/// You can implement this trait to provide your custom way of calculating new pheromone to the ACO.
pub trait PheromoneUpdate<P: Pheromone> {
  /// Modifies the pheromone
  ///
  /// ## Arguments
  ///
  /// * `old_pheromone` - Pheromone used to generate current solutions
  /// * `solutions` - Current generated solution.
  /// * `evaporation_rate` - rate of old pheromone evaporation
  fn apply(&mut self, pheromone: &mut P, solutions: &[Solution], evaporation_rate: f64);
}

/// # Ant System Pheromone Update
///
/// Implements [PheromoneUpdate]. The pheromone is updated as first proposed by Marco Dorigo,
/// every ant leaves pheromone trail on its way, the pheromone trail strength is proportional
/// to the way fitness. New pheromone is a sum of old pheromone scaled by (1 - evaporation rate) and sum
/// of pheromone trails left by ants.
pub struct AntSystemPU;

impl AntSystemPU {
  /// Creates a new instance of [AntSystemPU]
  pub fn new() -> Self {
    AntSystemPU
  }
}

impl PheromoneUpdate<FMatrix> for AntSystemPU {
  fn apply(&mut self, pheromone: &mut FMatrix, solutions: &[Solution], evaporation_rate: f64) {
    let delta_pheromone = sum_iter_pheromone(solutions, pheromone.nrows());
    pheromone.scale_mut(1.0 -evaporation_rate);
    pheromone.add_assign(delta_pheromone);
  }
}

/// # Elitist Ant System Pheromone Update
///
/// Implements [PheromoneUpdate]. Similarity to [AntSystemPU], every ant leaves pheromone trail on its way,
/// the pheromone trail strength is proportional
/// to the way fitness. New pheromone is a sum of old pheromone scaled by (1 - evaporation rate) and sum
/// of pheromone trails left by ants, additionally we are adding pheromone left by the best ant overall.
pub struct ElitistAntSystemPU {
  overall_best: OverallBest,
}

impl ElitistAntSystemPU {
  /// Creates a new instance of [ElitistAntSystemPU]
  pub fn new() -> Self {
    ElitistAntSystemPU {
      overall_best: OverallBest::new(),
    }
  }
}

impl PheromoneUpdate<FMatrix> for ElitistAntSystemPU {
  fn apply(&mut self, pheromone: &mut FMatrix, solutions: &[Solution], evaporation_rate: f64) {
    self.overall_best.update_best(solutions);
    let delta_pheromone = sum_iter_pheromone(solutions, pheromone.nrows());

    pheromone.scale_mut(1.0 - evaporation_rate);
    pheromone.add_assign(delta_pheromone);
    pheromone.add_assign(self.overall_best.get_best_pheromone());
  }
}

/// # MAX-MIN Ant System Pheromone Update
///
/// Implements [PheromoneUpdate].
/// the pheromone trail strength is proportional
/// to the way fitness. New pheromone is a sum of old pheromone scaled by (1 - evaporation rate) and
/// pheromone trail left by ant chosen by [BestPolicy], additionally the pheromone value is clamped.
pub struct MMAntSystemPU<B: BestPolicy> {
  pub(in crate::aco) best_policy: B,
  pub(in crate::aco) lower_bound: f64,
  pub(in crate::aco) upper_bound: f64,
}

impl<B: BestPolicy> MMAntSystemPU<B> {
  /// Creates an [MMAntSystemPU] with user provided implementation of [BestPolicy].
  ///
  /// ## Arguments
  /// * `lower_bound` - Minimal possible pheromone value.
  /// * `upper_bound` - Maximal possible pheromone value.
  /// * `best_policy` - Implementation of [BestPolicy]
  pub fn with_best_policy(lower_bound: f64, upper_bound: f64, best_policy: B) -> Self {
    assert!(lower_bound >= 0.0, "Lower bound must be grater or equal 0");
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

impl MMAntSystemPU<OverallBest> {
  /// Creates an [MMAntSystemPU] with [OverallBest] best ant choosing policy
  ///
  /// ## Arguments
  /// * `lower_bound` - Minimal possible pheromone value.
  /// * `upper_bound` - Maximal possible pheromone value.
  pub fn new(lower_bound: f64, upper_bound: f64) -> Self {
    Self::with_best_policy(lower_bound, upper_bound, OverallBest::new())
  }
}

impl<B: BestPolicy> PheromoneUpdate<FMatrix> for MMAntSystemPU<B> {
  fn apply(&mut self, pheromone: &mut FMatrix, solutions: &[Solution], evaporation_rate: f64) {
    self.best_policy.update_best(solutions);
    let best_pheromone = self.best_policy.get_best_pheromone();

    pheromone.scale_mut(1.0 - evaporation_rate);
    pheromone.add_assign(best_pheromone);
    pheromone.iter_mut().for_each(|a| *a = a.clamp(self.lower_bound, self.upper_bound));
  }
}

/// # Ant Colony System Pheromone Update
///
/// Implements [PheromoneUpdate].
/// the pheromone trail strength is  proportional
/// to the way fitness. New pheromone is a sum of old pheromone scaled by (1 - evaporation rate) and
/// best ant pheromone trail scaled by evaporation rate. Best ant pheromone is selected based
/// on [BestPolicy] implementation.
pub struct AntColonySystemPU<B: BestPolicy> {
  pub(in crate::aco) best_policy: B,
}

impl AntColonySystemPU<OverallBest> {
  /// Creates an [AntColonySystemPU] with [OverallBest] best ant choosing policy
  pub fn new() -> Self {
    Self {
      best_policy: OverallBest::new(),
    }
  }
}

impl<B: BestPolicy> AntColonySystemPU<B> {
  /// Creates an [AntColonySystemPU] with user provided implementation of [BestPolicy].
  ///
  /// ## Arguments
  /// * `best_policy` - Implementation of [BestPolicy]
  pub fn with_policy(best_policy: B) -> Self {
    Self { best_policy }
  }
}


impl<B: BestPolicy> PheromoneUpdate<FMatrix> for AntColonySystemPU<B> {
  fn apply(&mut self, pheromone: &mut FMatrix, solutions: &[Solution], evaporation_rate: f64) {
    self.best_policy.update_best(solutions);
    let best_pheromone = self.best_policy.get_best_pheromone();

    pheromone.scale_mut(1.0 - evaporation_rate);
    pheromone.add_assign(best_pheromone.scale(evaporation_rate))
  }
}

/// # Part From Evaluation Pheromone Update
///
/// Implements [PheromoneUpdate].
/// The solution are split into the number of pheromone traits by value range
pub struct PartFromEvalPU;

impl PartFromEvalPU {
  pub fn new() -> Self {
    Self {}
  }
}

impl PheromoneUpdate<FMatrixArray> for PartFromEvalPU{

  fn apply(&mut self, pheromone: &mut FMatrixArray, solutions: &[Solution], evaporation_rate: f64) {
    let parts_num = pheromone.len() as f64;
    let mut min = solutions[0].fitness;
    let mut max = min;
    for sol in solutions {
      if sol.fitness < min {
        min = sol.fitness
      }
      if sol.fitness > max {
        max = sol.fitness
      }
    }
    let increment = (max - min) / parts_num;

    // evaporate
    pheromone.iter_mut()
      .for_each(|x| x.scale_mut(1.0 - evaporation_rate));

    // lay trail
    for s in solutions.iter() {
      let part = ((s.fitness - min) / increment) as usize;
      let i = part.clamp(0, pheromone.len()-1);
      let pheromone = &mut pheromone[i];
      for (i, j) in s.path.iter().circular_tuple_windows::<(&usize, &usize)>() {
        pheromone[(*i, *j)] += s.fitness;
        pheromone[(*j, *i)] += s.fitness;
      }
    }
  }
}

#[inline]
fn sum_iter_pheromone(solutions: &[Solution], solution_size: usize) -> FMatrix {
  let mut sum = FMatrix::zeros(solution_size, solution_size);
  for s in solutions.iter() {
    for (i, j) in s.path.iter().circular_tuple_windows::<(&usize, &usize)>() {
      sum[(*i, *j)] += s.fitness;
      sum[(*j, *i)] += s.fitness;
    }
  }
  sum
}

#[cfg(test)]
mod tests {
  use crate::aco::pheromone::{
    AntColonySystemPU, AntSystemPU, ElitistAntSystemPU, MMAntSystemPU, PheromoneUpdate,
  };
  use crate::aco::{FMatrix, Solution};

  #[test]
  fn check_ant_system_pu_with_example() {
    let mut pheromone = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 2.0, 1.0, 0.0, 4.0, 2.0, 4.0, 0.0]);

    let sols = [
      Solution {
        path: vec![0, 1, 2],
        fitness: 0.125,
      },
      Solution {
        path: vec![0, 1, 2],
        fitness: 0.25,
      },
    ];

    let mut pu = AntSystemPU;
    pu.apply(&mut pheromone, &sols, 0.25);
    let new_pheromone = pheromone;
    let pheromone = vec![0.0, 1.125, 1.875, 1.125, 0.0, 3.375, 1.875, 3.375, 0.0];

    for (p, p_exp) in new_pheromone.iter().zip(pheromone.iter()) {
      assert_eq!(p, p_exp);
    }
  }

  #[test]
  fn check_elitist_ant_system_pu_with_example() {
    let mut pheromone = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 2.0, 1.0, 0.0, 4.0, 2.0, 4.0, 0.0]);

    let sols = [
      Solution {
        path: vec![0, 1, 2],
        fitness: 0.125,
      },
      Solution {
        path: vec![0, 1, 2],
        fitness: 0.25,
      },
    ];

    let mut pu = ElitistAntSystemPU::new();
    pu.apply(&mut pheromone, &sols, 0.25);
    let new_pheromone = pheromone;
    let pheromone = vec![0.0, 1.375, 2.125, 1.375, 0.0, 3.625, 2.125, 3.625, 0.0];

    for (p, p_exp) in new_pheromone.iter().zip(pheromone.iter()) {
      assert_eq!(p, p_exp);
    }
  }

  #[test]
  fn check_max_min_ant_system_pu_with_example() {
    let mut pheromone = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 2.0, 1.0, 0.0, 4.0, 2.0, 4.0, 0.0]);

    let sols = [
      Solution {
        path: vec![0, 1, 2],
        fitness: 0.125,
      },
      Solution {
        path: vec![0, 1, 2],
        fitness: 0.25,
      },
    ];

    let mut pu = MMAntSystemPU::new(1.5, 3.0);
    pu.apply(&mut pheromone, &sols, 0.25);
    let new_pheromone = pheromone;
    let pheromone = vec![1.5, 1.5, 1.75, 1.5, 1.5, 3.0, 1.75, 3.0, 1.5];

    for (p, p_exp) in new_pheromone.iter().zip(pheromone.iter()) {
      assert_eq!(p, p_exp);
    }
  }

  #[test]
  fn check_ant_colony_system_pu_with_example() {
    let mut pheromone = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 2.0, 1.0, 0.0, 4.0, 2.0, 4.0, 0.0]);

    let sols = [
      Solution {
        path: vec![0, 1, 2],
        fitness: 0.125,
      },
      Solution {
        path: vec![0, 1, 2],
        fitness: 0.25,
      },
    ];

    let mut pu = AntColonySystemPU::new();
    pu.apply(&mut pheromone, &sols, 0.25);
    let new_pheromone = pheromone;
    let pheromone = vec![0.0, 0.8125, 1.5625, 0.8125, 0.0, 3.0625, 1.5625, 3.0625, 0.0];

    for (p, p_exp) in new_pheromone.iter().zip(pheromone.iter()) {
      assert_eq!(p, p_exp);
    }
  }
}
