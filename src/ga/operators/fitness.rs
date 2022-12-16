use crate::ga::individual::Chromosome;

pub trait Fitness<T: Chromosome> {
  fn apply(&self, chromosome: T) -> f64;
}

impl<T: Chromosome> Fitness<T> for fn(chromosome: T) -> f64 {
  fn apply(&self, chromosome: T) -> f64 {
    self(chromosome)
  }
}
