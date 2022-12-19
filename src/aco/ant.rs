use crate::aco::FMatrix;
use push_trait::Push;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

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

  pub fn is_stuck(&self) -> bool {
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
