use itertools::Itertools;
use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Uniform};

use crate::ga::Individual;

use super::individual::ChromosomeWrapper;

pub fn sum_of_squares_fitness(individual: &Individual<Vec<f64>>) -> f64 {
	10.0 * f64::exp(-sum_of_squares(individual.get_chromosome()))
}

fn sum_of_squares(chromosome: &[f64]) -> f64 {
	chromosome.iter().map(|v| v * v).sum()
}

pub fn point_generator(restrictions: &Vec<(f64, f64)>) -> Vec<f64> {
	assert!(!restrictions.is_empty());

	let mut point: Vec<f64> = Vec::with_capacity(restrictions.len());

	for restriction in restrictions {
		point.push(restriction.1 * rand::random::<f64>() + restriction.0);
	}

	point
}

pub fn quadratic_population_factory(population_size: usize) -> Vec<Individual<Vec<f64>>> {
  let mut population: Vec<Individual<Vec<f64>>> = Vec::with_capacity(population_size);
	let restrictions = vec![(-2.0, 2.0), (-2.0, 2.0)];

  for _ in 0..population_size {
		let chromosome = point_generator(&restrictions);

    population.push(Individual {
      chromosome,
      fitness: f64::MAX,
    });
  }
  population
}

pub fn rastrigin_fitness(individual: &Individual<Vec<f64>>) -> f64 {
	5.0 * f64::exp(-rastrigin(individual.get_chromosome()))
}

fn rastrigin(chromosome: &[f64]) -> f64 {
	10.0 * chromosome.len() as f64 + chromosome.iter().fold(0.0, |sum, x| {
		sum + x.powi(2) - 10.0 * (2.0 * std::f64::consts::PI * x).cos()
	})
}

pub fn rastrigin_population_factory(population_size: usize) -> Vec<Individual<Vec<f64>>> {
	let mut population: Vec<Individual<Vec<f64>>>  = Vec::with_capacity(population_size);

	let domain = rand::distributions::Uniform::from(-5.12..5.12);

	for _ in 0..population_size {
		let chromosome = rand::thread_rng().sample_iter(domain).take(3).into_iter().collect_vec();
		population.push(Individual { chromosome, fitness: f64::MAX })
	}

	population
}
