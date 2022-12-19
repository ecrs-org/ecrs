pub mod builder;
pub mod probe;
mod solution;

use itertools::Itertools;
use push_trait::Push;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::iter::zip;

pub use solution::Solution;

use crate::aco::pheromone::PheromoneUpdate;
use crate::aco::AntSystemCfg;
use crate::aco::FMatrix;

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

pub struct Ant<R: Rng> {
  unvisited: HashSet<usize>,
  path: Vec<usize>,
  solution_size: usize,
  stuck: bool,
  rng: R,
}

impl Ant<ThreadRng> {
  pub fn new(solution_size: usize) -> Self {
    Self::with_rng(solution_size, thread_rng())
  }
}

impl<R: Rng> Ant<R> {
  pub fn with_rng(solution_size: usize, rng: R) -> Self {
    Self {
      unvisited: HashSet::with_capacity(solution_size),
      path: Vec::with_capacity(solution_size),
      stuck: false,
      solution_size,
      rng,
    }
  }

  pub fn clear(&mut self) {
    for i in 0..self.solution_size {
      self.unvisited.push(i);
    }
    self.path.clear();
    self.stuck = false;
  }

  pub fn get_path(&self) -> &[usize] {
    &self.path
  }

  pub fn chose_staring_place(&mut self) -> usize {
    let start: usize = self.rng.gen_range(0..self.solution_size);
    self.unvisited.remove(&start);
    self.path.push(start);
    start
  }

  fn is_stuck(&self) -> bool {
    self.stuck
  }

  pub fn go_to_next_place(&mut self, edges_goodness: &FMatrix) -> (usize, usize) {
    let last = *self
      .path
      .last()
      .expect("Path is empty. Did you forget to call Ant::chose_staring_place");
    if self.is_stuck() {
      return (last, last);
    }
    let row = edges_goodness.row(last);

    if self.unvisited.is_empty() {
      panic!("Ant had already visited every place");
    }

    let mut goodness_sum = 0.0f64;
    for v in self.unvisited.iter() {
      goodness_sum += row[*v];
    }

    let mut random: f64 = self.rng.gen_range(0.0..=goodness_sum);
    let mut next: usize = last;
    for v in self.unvisited.iter() {
      random -= row[*v];
      if random <= 0.0 {
        next = *v;
        break;
      }
    }

    if next == last {
      self.stuck = true;
    }

    self.unvisited.remove(&next);
    self.path.push(next);

    (last, next)
  }
}
