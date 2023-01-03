use crate::ga::individual::Chromosome;

pub trait Fitness<T: Chromosome> {
  fn apply(&mut self, chromosome: &T) -> f64;
}

pub struct FnBasedFitness<T: Chromosome> {
  fn_ptr: fn(&T) -> f64,
}

impl<T: Chromosome> FnBasedFitness<T> {
  pub fn new(fn_ptr: fn(&T) -> f64) -> Self {
    FnBasedFitness { fn_ptr }
  }
}

impl<T: Chromosome> Fitness<T> for FnBasedFitness<T> {
  fn apply(&mut self, chromosome: &T) -> f64 {
    (self.fn_ptr)(chromosome)
  }
}
