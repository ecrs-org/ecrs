use serde::Serialize;
use std::{fmt::Debug, ops::Deref};

pub trait Chromosome: Sized + Sync + Send + Clone + Default + Debug {}

/// Blanket implementation
impl<T: Sized + Sync + Send + Clone + Default + Debug> Chromosome for T {}

#[derive(Clone, Debug, Serialize)]
pub struct Individual<T: Chromosome> {
  pub chromosome: T,
  pub fitness: f64,
}

impl<T: Chromosome> Individual<T> {
  pub fn new() -> Self {
    Individual {
      chromosome: T::default(),
      fitness: f64::default(),
    }
  }

  #[inline]
  pub fn chromosome_ref(&self) -> &T {
    &self.chromosome
  }

  #[inline]
  pub fn chromosome_ref_mut(&mut self) -> &mut T {
    &mut self.chromosome
  }
}

impl<T: Chromosome> Deref for Individual<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.chromosome
  }
}

impl<T: Chromosome> From<T> for Individual<T> {
  fn from(chromosome: T) -> Self {
    Individual {
      chromosome,
      fitness: f64::MIN,
    }
  }
}

impl<T: Chromosome> PartialEq<Self> for Individual<T> {
  fn eq(&self, other: &Self) -> bool {
    self.fitness == other.fitness
  }
}

impl<T: Chromosome> Eq for Individual<T> {}

impl<T: Chromosome> PartialOrd<Self> for Individual<T> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.fitness.partial_cmp(&other.fitness)
  }
}

impl<T: Chromosome> Ord for Individual<T> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    if let Some(ord) = self.partial_cmp(other) {
      return ord;
    }
    unimplemented!();
  }
}

pub type RealValueIndividual = Individual<Vec<f64>>;
pub type BitStringInvididual = Individual<Vec<bool>>;
