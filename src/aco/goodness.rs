use crate::aco::FMatrix;

pub trait Goodness {
  fn apply(&mut self, pheromone: &FMatrix) -> FMatrix;
}

pub struct CanonicalGoodness {
  alpha: f64,
  beta: f64,
  heuristic: FMatrix,
}

impl CanonicalGoodness {
  pub fn new(alpha: f64, beta: f64, heuristic: FMatrix) -> Self {
    Self {
      alpha,
      beta,
      heuristic,
    }
  }
}

impl Goodness for CanonicalGoodness {
  fn apply(&mut self, pheromone: &FMatrix) -> FMatrix {
    let solution_size = pheromone.nrows();
    let iter = pheromone
      .iter()
      .zip(self.heuristic.iter())
      .map(|(p, h)| p.powf(self.alpha) * h.powf(self.beta));

    FMatrix::from_iterator(solution_size, solution_size, iter)
  }
}
