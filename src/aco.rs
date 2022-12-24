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
//!   let ant_s = ecrs::aco::Builder::new_as(10)
//!       .set_weights(cost)
//!       .set_heuristic(heuristic)
//!       .with_standard_ants(10)
//!       .with_iteration_termination(100)
//!       .with_stdout_probe()
//!       .build();
//!
//!   ant_s.run();
//! }
//! ```
pub mod ant;
pub mod ants_behaviour;
pub mod builder;
pub mod fitness;
pub mod goodness;
pub mod local_update;
pub mod pheromone;
pub mod probe;
mod solution;
pub mod termination_condition;
pub mod util;

pub use builder::Builder;
pub use solution::Solution;

use crate::aco::ant::Ant;
use crate::aco::ants_behaviour::AntsBehaviour;
use crate::aco::fitness::Fitness;
use crate::aco::goodness::Goodness;
use crate::aco::pheromone::PheromoneUpdate;
use crate::aco::probe::Probe;
use crate::aco::termination_condition::TerminationCondition;
use nalgebra::{Dynamic, OMatrix};

pub type FMatrix = OMatrix<f64, Dynamic, Dynamic>;

/// # Ant Colony Optimization
///
/// Encapsulates common ACO algorithm patterns.
///
/// To extract data use a [probe](probe)
pub struct AntColonyOptimization<P, A, G, AB, F, T, Pr>
where
  P: PheromoneUpdate,
  A: Ant,
  G: Goodness,
  AB: AntsBehaviour<A, G>,
  F: Fitness,
  T: TerminationCondition<A>,
  Pr: Probe,
{
  pheromone_update: P,
  evaporation_rate: f64,
  ants_behaviour: AB,
  pheromone: FMatrix,
  ants: Vec<A>,
  fitness: F,
  goodness: G,
  termination_cond: T,
  probe: Pr,
}

impl<P, A, G, AB, F, T, Pr> AntColonyOptimization<P, A, G, AB, F, T, Pr>
where
  P: PheromoneUpdate,
  A: Ant,
  G: Goodness,
  AB: AntsBehaviour<A, G>,
  F: Fitness,
  T: TerminationCondition<A>,
  Pr: Probe,
{
  /// Executes the algorithm
  pub fn run(mut self) {
    self.termination_cond.init(&self.pheromone);
    while !self
      .termination_cond
      .update_and_check(&self.pheromone, &self.ants)
    {
      self.probe.on_iteration_start();
      self.iterate();
      self.probe.on_iteration_end();
    }

    self.end()
  }

  fn iterate(&mut self) {
    let paths = self
      .ants_behaviour
      .simulate_ants(&mut self.ants, &mut self.pheromone, &mut self.goodness);
    let sols = self.grade(paths);

    let best = self.find_best(&sols);
    self.probe.on_current_best(best);

    let new_pheromone = self
      .pheromone_update
      .apply(&self.pheromone, &sols, self.evaporation_rate);

    self.probe.on_pheromone_update(&self.pheromone, &new_pheromone);
    self.pheromone = new_pheromone;
  }

  fn find_best<'a>(&mut self, sols: &'a [Solution]) -> &'a Solution {
    let best = sols
      .iter()
      .reduce(|a, b| if a.fitness > b.fitness { a } else { b });

    best.unwrap()
  }

  fn grade(&mut self, paths: Vec<Vec<usize>>) -> Vec<Solution> {
    let mut sols: Vec<Solution> = Vec::with_capacity(paths.len());

    for path in paths {
      let fitness = self.fitness.apply(&path);

      let mut solution = Solution::from_path(path);
      solution.fitness = fitness;
      sols.push(solution);
    }

    sols
  }

  fn end(mut self) {
    self.probe.on_end();
  }
}
