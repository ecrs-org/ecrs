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
pub mod fitness;
pub mod goodness;
pub mod pheromone;
pub mod probe;
mod solution;
pub mod util;

pub(self) use aco_cfg::AntColonyOptimizationCfg;
pub use builder::Builder;
pub use solution::Solution;

use crate::aco::ants_behaviour::AntsBehaviour;
use crate::aco::fitness::Fitness;
use crate::aco::pheromone::PheromoneUpdate;
use nalgebra::{Dynamic, OMatrix};

pub type FMatrix = OMatrix<f64, Dynamic, Dynamic>;

/// # Ant Colony Optimization
///
/// Encapsulates common ACO algorithm patterns.
///
/// To extract data use a [probe](probe)
pub struct AntColonyOptimization<P: PheromoneUpdate, AB: AntsBehaviour, F: Fitness> {
  cfg: AntColonyOptimizationCfg<P>,
  ants_behaviour: AB,
  pheromone: FMatrix,
  fitness: F,
}

impl<P, AB, F> AntColonyOptimization<P, AB, F>
where
  P: PheromoneUpdate,
  AB: AntsBehaviour,
  F: Fitness,
{
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
    let paths = self.ants_behaviour.simulate_ants(&mut self.pheromone);
    let sols = self.grade(paths);

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

  fn grade(&mut self, paths: Vec<Vec<usize>>) -> Vec<Solution> {
    let mut sols: Vec<Solution> = Vec::with_capacity(paths.len());

    for path in paths {
      let fitness = self.fitness.apply(&path);
      let cost = 1.0 / fitness;

      let mut solution = Solution::from_path(path);
      solution.fitness = fitness;
      solution.cost = cost;
      sols.push(solution);
    }

    sols
  }

  fn end(mut self) {
    self.cfg.probe.on_end();
  }
}
