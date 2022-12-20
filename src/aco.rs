//! Implementation of Ant based algorithms.
//!
//! # Ant Colony Optimization
//! Implementation is based on those sources:
//! * <https://ieeexplore.ieee.org/document/4129846> DOI: 10.1109/MCI.2006.329691
//! * <http://www.scholarpedia.org/article/Ant_colony_optimization>
//!
//! Look at [Builder](Builder) for parameters overview and
//! at [AntColonyOptimization] for interface details
//!
//! Logging system details can be found [here](probe)
//!
//! ## Example
//! Solving TSP using Ant System algorithm variant
//! ```rust
//! pub fn ants_example_run() {
//!   let (cities, cost) = ecrs::aco::util::generate_tsp_cost(10);
//!
//!   let heuristic = ecrs::aco::util::create_heuristic_from_weights(&cost);
//!
//!   let ant_s = ecrs::aco::Builder::new()
//!       .set_weights(cost)
//!       .set_heuristic(heuristic)
//!       .set_pheromone_update(ecrs::aco::pheromone::AntSystemPU)
//!       .build();
//!
//!   ant_s.run();
//! }
//! ```
mod aco_cfg;
mod ant;
mod ants_behaviour;
pub mod builder;
pub mod goodness;
pub mod pheromone;
pub mod probe;
mod solution;
pub mod util;

pub(self) use aco_cfg::AntColonyOptimizationCfg;
pub use builder::Builder;
pub use solution::Solution;

use crate::aco::ants_behaviour::AntsBehaviour;
use crate::aco::pheromone::PheromoneUpdate;
use itertools::Itertools;
use nalgebra::{Dynamic, OMatrix};
use std::iter::zip;

pub type FMatrix = OMatrix<f64, Dynamic, Dynamic>;

/// # Ant Colony Optimization
///
/// Encapsulates common ACO algorithm patterns.
///
/// To extract data use a [probe](probe)
pub struct AntColonyOptimization<P: PheromoneUpdate, AB: AntsBehaviour> {
  cfg: AntColonyOptimizationCfg<P>,
  ants_behaviour: AB,
  pheromone: FMatrix,
}

impl<P: PheromoneUpdate, AB: AntsBehaviour> AntColonyOptimization<P, AB> {
  /// Executes the algorithm
  pub fn run(mut self) {
    for i in 0..self.cfg.iteration {
      self.cfg.probe.on_iteration_start(i);
      self.iterate();
      self.cfg.probe.on_iteration_end(i);
    }

    self.end()
  }

  fn iterate(&mut self) {
    let sols_m = self.ants_behaviour.simulate_ants(&mut self.pheromone);
    let sols = self.grade(sols_m);

    let best = self.find_best(&sols);
    self.cfg.probe.on_current_best(best);

    let new_pheromone = self
      .cfg
      .pheromone_update
      .apply(&self.pheromone, &sols, self.cfg.evaporation_rate);

    self
      .cfg
      .probe
      .on_pheromone_update(&self.pheromone, &new_pheromone);
    self.pheromone = new_pheromone;
  }

  fn find_best<'a>(&mut self, sols: &'a [Solution]) -> &'a Solution {
    let best = sols.iter().min_by(|a, b| (*a).partial_cmp(*b).unwrap());

    best.unwrap()
  }

  fn grade(&self, sols_m: Vec<FMatrix>) -> Vec<Solution> {
    let costs: Vec<f64> = Vec::from_iter(sols_m.iter().map(|s| self.grade_one(s)));
    let mut sols: Vec<Solution> = Vec::new();
    for (m, c) in zip(sols_m, costs) {
      sols.push(Solution { matrix: m, cost: c })
    }

    sols
  }

  fn grade_one(&self, s: &FMatrix) -> f64 {
    s.component_mul(&self.cfg.weights).sum() / 2.0
  }

  fn end(mut self) {
    self.cfg.probe.on_end();
  }
}

#[inline]
fn path_to_matrix(path: &[usize]) -> FMatrix {
  let sol_size = path.len();
  let mut sol = FMatrix::zeros(sol_size, sol_size);
  for (i, j) in path.iter().tuples::<(&usize, &usize)>() {
    sol[(*i, *j)] = 1.0;
    sol[(*j, *i)] = 1.0;
  }

  sol
}
