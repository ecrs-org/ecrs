use std::fmt::Debug;
use serde::Serialize;


pub trait Chromosome: Sized + Sync + Send + Clone + Default + Debug {}

/// Blanket implementation
impl<T: Sized + Sync + Send + Clone + Default + Debug> Chromosome for T {}

pub trait ChromosomeWrapper<T: Chromosome>: Sized + Sync + Clone + Ord + Debug {
	fn new() -> Self;
	fn get_chromosome(&self) -> &T;
	fn get_chromosome_mut(&mut self) -> &mut T;
	fn get_fitness(&self) -> f64;
	fn set_fitness(&mut self, fitness: f64);
}

#[derive(Clone, Debug, Serialize)]
pub struct Individual<T: Chromosome> {
  pub chromosome: T,
  pub fitness: f64,
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

impl<T: Chromosome> ChromosomeWrapper<T> for Individual<T> {
	fn new() -> Self {
		Individual { chromosome: T::default(), fitness: f64::default() }
	}

	#[inline]
	fn get_chromosome(&self) -> &T {
		&self.chromosome
	}

	#[inline]
	fn get_chromosome_mut(&mut self) -> &mut T {
		&mut self.chromosome
	}

	#[inline]
	fn get_fitness(&self) -> f64 {
		self.fitness
	}

	#[inline]
	fn set_fitness(&mut self, fitness: f64) {
		self.fitness = fitness;
	}
}

pub type RealValueIndividual = Individual<Vec<f64>>;
pub type BitStringInvididual = Individual<Vec<bool>>;
