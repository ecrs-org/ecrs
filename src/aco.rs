//! Implementation of Ant based algorithms.
//!
//! As for now only only original Ant System algorithm is implemented.
//!
//! # Ant System
//! Implementation is based on those sources:
//! * <https://ieeexplore.ieee.org/document/4129846> DOI: 10.1109/MCI.2006.329691
//! * <http://www.scholarpedia.org/article/Ant_colony_optimization>
//!
//! Look at [Builder](Builder) for parameters overview and
//! at [AntSystem](AntSystem) for interface details
//!
//! Logging system details can be found [here](probe)
//!
//! ## Example
//! Solving TSP using AntSystem
//! ```rust
//! pub fn ants_example_run() {
//!   let (cities, cost) = ecrs::aco::generate_tsp_cost(10);
//!
//!   let heuristic = ecrs::aco::create_heuristic_from_weights(&cost);
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
//!
use crate::aco::ant::Ant;
use crate::aco::pheromone::PheromoneUpdate;
pub(self) use ant_system_cfg::AntSystemCfg;
pub use builder::Builder;
use itertools::Itertools;
use nalgebra::{Dynamic, OMatrix};
use rand::rngs::ThreadRng;
use rand::Rng;
pub use solution::Solution;
use std::error::Error;
use std::iter::zip;
mod ant;
mod ant_system_cfg;
mod ants_system;
pub mod builder;
pub mod pheromone;
pub mod probe;
mod solution;

pub type FMatrix = OMatrix<f64, Dynamic, Dynamic>;

/// Wrapper class for AntSystem algorithm.
///
/// To extract data use a [probe](probe)
pub struct AntSystem<P: PheromoneUpdate> {
  cfg: AntSystemCfg<P>,
  pheromone: FMatrix,
  best_sol: Solution,
  ants: Vec<Ant<ThreadRng>>,
}

impl<P: PheromoneUpdate> AntSystem<P> {
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
    let sols_m = self.run_ants();
    let sols = self.grade(sols_m);

    let best = self.find_best(&sols);
    self.cfg.probe.on_current_best(best);
    self.update_best(best);

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

  fn update_best(&mut self, current_best: &Solution) {
    if self.best_sol > *current_best {
      self.cfg.probe.on_new_best(current_best);
      self.best_sol = (*current_best).clone();
    }
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

  fn run_ants(&mut self) -> Vec<FMatrix> {
    let prob_iter = self
      .pheromone
      .iter()
      .zip(self.cfg.heuristic.iter())
      .map(|(p, h)| self.calc_prob(p, h));

    let solution_size: usize = self.pheromone.nrows();
    let prob = FMatrix::from_iterator(solution_size, solution_size, prob_iter);

    let mut sols: Vec<FMatrix> = Vec::with_capacity(self.cfg.ants_num);
    for ant in self.ants.iter_mut() {
      ant.clear();
      ant.chose_staring_place();
      for _ in 1..solution_size {
        ant.go_to_next_place(&prob);
      }

      if ant.is_stuck() {
        break;
      }
      let path = ant.get_path();
      sols.push(path_to_matrix(path));
    }

    sols
  }

  fn calc_prob(&self, p: &f64, h: &f64) -> f64 {
    p.powf(self.cfg.alpha) * h.powf(self.cfg.beta)
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

pub fn into_vec(m: &FMatrix) -> Vec<Vec<f64>> {
  let mut m_vec: Vec<Vec<f64>> = Vec::new();

  for row in m.row_iter() {
    m_vec.push(row.iter().copied().collect_vec());
  }
  m_vec
}

/// Utility function for generating heuristic from cost(weights)
pub fn create_heuristic_from_weights(weights: &FMatrix) -> FMatrix {
  let mut heu = FMatrix::zeros(weights.nrows(), weights.ncols());
  heu.iter_mut().zip(weights.iter()).for_each(|(h, w)| {
    if *w == 0.0 {
      *h = 0.0;
    } else {
      *h = 1.0 / *w
    }
  });

  heu
}

/// Utility function for generating TSP input data.
///
/// ## Arguments
/// * `sol_size` - number of cites.
pub fn generate_tsp_cost(sol_size: usize) -> (Vec<(f64, f64)>, FMatrix) {
  let mut cities: Vec<(f64, f64)> = Vec::new();
  let mut r = rand::thread_rng();
  for _ in 0..sol_size {
    let x: f64 = r.gen_range(0.0..100.0);
    let y: f64 = r.gen_range(0.0..100.0);

    cities.push((x, y))
  }

  let mut cost: FMatrix = FMatrix::zeros(sol_size, sol_size);
  for i in 0..sol_size {
    for j in i..sol_size {
      let (x1, y1) = cities[i];
      let (x2, y2) = cities[j];
      let x = x1 - x2;
      let y = y1 - y2;

      let dist = f64::sqrt(x * x + y * y);

      cost[(i, j)] = dist;
      cost[(j, i)] = dist;
    }
  }

  (cities, cost)
}

/// Utility function for writing TSP input data to a CSV file.
///
/// ## Arguments
/// * `cities` - Vector of tuples representing cities x and y positions
/// * `path` - Where to save file.
pub fn write_cities_csv(cities: &[(f64, f64)], path: &str) -> Result<(), Box<dyn Error>> {
  let mut wtr = csv::Writer::from_path(path)?;
  wtr.write_record(["x", "y"])?;
  for (x, y) in cities.iter() {
    wtr.write_record(&[x.to_string(), y.to_string()])?;
  }
  wtr.flush()?;

  Ok(())
}
