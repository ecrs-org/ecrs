use crate::aco::{FMatrix, Solution};

pub trait BestPolicy {
  fn update_best(&mut self, solutions: &[Solution]);

  fn get_best_pheromone(&self) -> &FMatrix;
}

pub struct IterationBest {
  best_pheromone: FMatrix,
}

impl IterationBest {
  pub fn new() -> Self {
    Self {
      best_pheromone: FMatrix::zeros(0, 0),
    }
  }
}

impl BestPolicy for IterationBest {
  fn update_best(&mut self, solutions: &[Solution]) {
    let iter_best = find_best(solutions);
    self.best_pheromone = iter_best.matrix.scale(1.0 / iter_best.cost);
  }

  fn get_best_pheromone(&self) -> &FMatrix {
    &self.best_pheromone
  }
}

pub struct OverallBest {
  best_pheromone: FMatrix,
  cost: f64,
}

impl OverallBest {
  pub fn new() -> Self {
    Self {
      best_pheromone: FMatrix::zeros(0, 0),
      cost: f64::MAX,
    }
  }
}

impl BestPolicy for OverallBest {
  fn update_best(&mut self, solutions: &[Solution]) {
    let iter_best = find_best(solutions);

    if iter_best.cost < self.cost {
      self.best_pheromone = iter_best.matrix.scale(1.0 / iter_best.cost);
    }
  }

  fn get_best_pheromone(&self) -> &FMatrix {
    &self.best_pheromone
  }
}

#[inline]
fn find_best(solutions: &[Solution]) -> &Solution {
  solutions
    .iter()
    .reduce(|a, b| if a.cost > b.cost { b } else { a })
    .unwrap()
}
