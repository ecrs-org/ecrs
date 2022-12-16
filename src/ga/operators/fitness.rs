use crate::ga::individual::Chromosome;

pub trait Fitness<T: Chromosome> {
  fn apply(&self, chromosome: T) -> f64;
}
