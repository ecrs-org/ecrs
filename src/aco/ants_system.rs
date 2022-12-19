pub mod builder;
pub mod probe;
mod solution;

use push_trait::Push;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::iter::zip;
use rand::rngs::ThreadRng;

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
  ants: Vec<Ant<ThreadRng>>
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

  fn run_ants(&self) -> Vec<FMatrix> {
    let prob_iter = self
      .pheromone
      .iter()
      .zip(self.cfg.heuristic.iter())
      .map(|(p, h)| self.calc_prob(p, h));

    let prob = FMatrix::from_iterator(self.pheromone.nrows(), self.pheromone.ncols(), prob_iter);

    let sols: Vec<FMatrix> = Vec::from_iter((0..self.cfg.ants_num).map(|_| run_ant(&prob)));

    sols
  }

  fn calc_prob(&self, p: &f64, h: &f64) -> f64 {
    p.powf(self.cfg.alpha) * h.powf(self.cfg.beta)
  }

  fn end(mut self) {
    self.cfg.probe.on_end();
  }
}

pub struct Ant<R: Rng> {
  unvisited: HashSet<usize>,
  path: Vec<usize>,
  solution_size: usize,
  stuck: bool,
  rng: R,
}

impl Ant<rand::prelude::ThreadRng> {
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

    (last, next)
  }
}

pub fn run_ant(prob: &FMatrix) -> FMatrix {
  let n = prob.nrows();
  let mut sol = FMatrix::zeros(n, n);
  let mut random = thread_rng();
  let mut unvisited: HashSet<usize> = HashSet::from_iter(0..n);

  let first: usize = random.gen_range(0..n);
  unvisited.remove(&first);
  let mut last: usize = first;

  while !unvisited.is_empty() {
    let mut sum = 0.0_f64;
    let row = prob.row(last);
    for v in unvisited.iter() {
      sum += row[*v];
    }

    let r_range = 0.0..sum;
    if r_range.is_empty() {
      println!("Could not find a solution");
      return FMatrix::zeros(n, n);
    }

    let mut r = random.gen_range(r_range);
    let mut next = last; // maybe 0
    for v in unvisited.iter() {
      r -= row[*v];
      if r <= 0.0 {
        next = *v;
        break;
      }
    }

    sol[(last, next)] = 1.0;
    sol[(next, last)] = 1.0;
    unvisited.remove(&next);
    last = next;
  }

  sol[(last, first)] = 1.0;
  sol[(first, last)] = 1.0;

  sol
}
