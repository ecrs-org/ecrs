use std::cmp::Ordering;
use serde::Serialize;

pub trait Gene: Sized + Default + Copy {}

// Blanket implementaion.
impl<T: Sized + Default + Copy> Gene for T {}

pub type Chromosome<T> = Vec<T>;

pub trait ChromosomeWrapper<T: Gene>: Ord {
	fn new() -> Self;
	fn get_chromosome(&self) -> &Chromosome<T>;
	fn get_chromosome_mut(&mut self) -> &mut Chromosome<T>;
	fn get_fitness(&self) -> f64;
}

#[derive(Clone, Debug, Serialize)]
pub struct Individual<T: Gene> {
  pub chromosome: Chromosome<T>,
  pub fitness: f64,
}

impl<T: Gene> PartialEq<Self> for Individual<T> {
	fn eq(&self, other: &Self) -> bool {
		self.fitness == other.fitness
	}
}

impl<T: Gene> Eq for Individual<T> {}

impl<T: Gene> PartialOrd<Self> for Individual<T> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.fitness.partial_cmp(&other.fitness)
	}
}

impl<T: Gene> Ord for Individual<T> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if let Some(ord) = self.partial_cmp(other) {
			return ord;
		}
		unimplemented!();
	}
}

impl<T: Gene> ChromosomeWrapper<T> for Individual<T> {
	fn new() -> Self {
		Individual { chromosome: Chromosome::default(), fitness: f64::default() }
	}

	#[inline]
	fn get_chromosome(&self) -> &Chromosome<T> {
		&self.chromosome
	}

	#[inline]
	fn get_chromosome_mut(&mut self) -> &mut Chromosome<T> {
		&mut &mut self.chromosome
	}

	#[inline]
	fn get_fitness(&self) -> f64 {
		self.fitness
	}
}

pub type RealValueIndividual = Individual<f64>;
pub type BitStringInvididual = Individual<bool>;
