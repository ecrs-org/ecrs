use crate::aco::{FMatrix, Solution};
use std::ops::Add;

pub trait PheromoneUpdate {
  fn apply(&mut self, old_pheromone: &FMatrix, solutions: &[Solution], evaporation_rate: f64) -> FMatrix;
}

pub struct AntSystemPU;

impl PheromoneUpdate for AntSystemPU {
  fn apply(&mut self, old_pheromone: &FMatrix, solutions: &[Solution], evaporation_rate: f64) -> FMatrix {
    let delta_pheromone = solutions
      .iter()
      .map(|sol| sol.matrix.scale(1.0 / sol.cost))
      .reduce(|s1, s2| s1.add(s2))
      .expect("pheromone update creation error");

    old_pheromone.scale(1.0 - evaporation_rate).add(delta_pheromone)
  }
}

#[cfg(test)]
mod tests {
  use crate::aco::pheromone::{AntSystemPU, PheromoneUpdate};
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
}
