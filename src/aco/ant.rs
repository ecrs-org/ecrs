use crate::aco::FMatrix;
use push_trait::Push;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

pub trait Ant {
  /// Clears iteration specific data like visited vertices or path.
  fn clear(&mut self);
  /// Returns vector of vertices in order of visiting
  fn get_path(&self) -> &[usize];
  /// Selects an vertex to start from
  fn chose_staring_place(&mut self);
  /// Returns true when there is no valid next vertex with path not fully constructed.
  fn is_stuck(&self) -> bool;
  /// Chooses and goes to the next vertex. Returns traversed edge.
  fn go_to_next_place(&mut self, edges_goodness: &FMatrix) -> (usize, usize);
}

/// # Canonical Ant
///
/// Represent a single ant.
/// Used to build a solution.
/// Related to [AntsBehavior]
pub struct CanonicalAnt<R: Rng> {
  unvisited: HashSet<usize>,
  path: Vec<usize>,
  solution_size: usize,
  stuck: bool,
  rng: R,
}

impl<R: Rng> CanonicalAnt<R> {
  /// Create a new instance of [CanonicalAnt] with user specified RNG.
  ///
  /// ## Arguments
  /// * `solution_size` - Numer of graph vertices.
  /// * `rng` - Random numbers generator.
  pub fn with_rng(solution_size: usize, rng: R) -> Self {
    Self {
      unvisited: HashSet::with_capacity(solution_size),
      path: Vec::with_capacity(solution_size),
      stuck: false,
      solution_size,
      rng,
    }
  }
}

impl<R: Rng> Ant for CanonicalAnt<R> {
  /// Clears iteration specific data like visited vertices or path.
  fn clear(&mut self) {
    for i in 0..self.solution_size {
      self.unvisited.push(i);
    }
    self.path.clear();
    self.stuck = false;
  }
  /// Returns vector of vertices in order of visiting
  fn get_path(&self) -> &[usize] {
    &self.path
  }
  /// Selects an vertex to start from
  fn chose_staring_place(&mut self) {
    let start: usize = self.rng.gen_range(0..self.solution_size);
    self.unvisited.remove(&start);
    self.path.push(start);
  }
  /// Returns true when there is no valid next vertex with path not fully constructed.
  fn is_stuck(&self) -> bool {
    self.stuck
  }
  /// Chooses and goes to the next vertex. Returns traversed edge.
  ///
  /// Panic when starting vertex wasn't decided ([CanonicalAnt::chose_staring_place]) or when all vertices
  /// are already visited   
  fn go_to_next_place(&mut self, edges_goodness: &FMatrix) -> (usize, usize) {
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

pub type StandardAnt = CanonicalAnt<ThreadRng>;

impl StandardAnt {
  /// Create a new instance of [CanonicalAnt] with default RNG.
  ///
  /// ## Arguments
  /// * `solution_size` - Numer of graph vertices
  pub fn new(solution_size: usize) -> Self {
    Self::with_rng(solution_size, thread_rng())
  }
}
