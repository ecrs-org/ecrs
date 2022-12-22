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

macro_rules! standard_ant_impl {
  () => {
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
  };
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
  standard_ant_impl!();
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

impl CanonicalAnt<ThreadRng> {
  /// Create a new instance of [CanonicalAnt] with default RNG.
  ///
  /// ## Arguments
  /// * `solution_size` - Numer of graph vertices
  pub fn new(solution_size: usize) -> Self {
    Self::with_rng(solution_size, thread_rng())
  }
}

/// # Exploitation Ant
///
/// Represent a single exploiting ant.
///
/// With given probability it will chose the path with most pheromone.
///
/// Used to build a solution.
/// Related to [AntsBehavior]
pub struct ExploitingAnt<R: Rng> {
  unvisited: HashSet<usize>,
  path: Vec<usize>,
  solution_size: usize,
  stuck: bool,
  exploitation_rate: f64,
  rng: R,
}

impl<R: Rng> ExploitingAnt<R> {
  /// Create a new instance of [ExploitingAnt] with user specified RNG.
  ///
  /// ## Arguments
  /// * `solution_size` - Numer of graph vertices.
  /// * `exploitation_rate` - Number between 0.0 and 1.0. Probability of exploiting pheromone information.
  /// * `rng` - Random numbers generator.
  pub fn with_rng(solution_size: usize, exploitation_rate: f64, rng: R) -> Self {
    assert!(
      (0.0..1.0).contains(&exploitation_rate),
      "Exploitation rate must be in range (0.0..1.0)"
    );
    Self {
      unvisited: HashSet::with_capacity(solution_size),
      path: Vec::with_capacity(solution_size),
      stuck: false,
      solution_size,
      exploitation_rate,
      rng,
    }
  }
}

impl<R: Rng> Ant for ExploitingAnt<R> {
  standard_ant_impl!();

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

    let should_exploit = self.rng.gen::<f64>() < self.exploitation_rate;

    if should_exploit {
      let mut next = last;
      let mut value = f64::MIN;

      for v in self.unvisited.iter() {
        if value < row[*v] {
          next = *v;
          value = row[*v];
        }
      }

      return (last, next);
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
