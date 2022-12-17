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
