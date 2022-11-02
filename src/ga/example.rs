use std::cmp::min;
use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Uniform};
use std::f64::consts;

use crate::ga::Individual;

pub fn quadratic_fn(individual: &Individual) -> f64 {
	individual.chromosome.clone().into_iter().map(|val| val * val).sum()
}

pub fn quadratic_function(chromosome: &[f64]) -> f64 {
  return chromosome[0] * chromosome[0]; // + 3 as f64 * chromosome[0];
}

pub fn quadratic_mutation_operator(individual: &mut Individual) -> Individual {
  let idx = thread_rng().gen_range(0..individual.chromosome.len());
  let mut new_chromosome = individual.chromosome.clone();
  let mut rng = thread_rng();

  let mut distribution: Uniform<f64> = Uniform::from(-10_f64..10_f64);
  let random_value = distribution.sample(&mut rng) as f64;

  new_chromosome[idx] = random_value;
  // let
  Individual {
    chromosome: new_chromosome,
    fitness: f64::MAX,
  }
}

pub fn point_generator(restrictions: &Vec<(f64, f64)>) -> Vec<f64> {
	assert!(restrictions.len() > 0);

	let mut point: Vec<f64> = Vec::with_capacity(restrictions.len());

	for restriction in restrictions {
		point.push(restriction.1 * rand::random::<f64>() + restriction.0);
	}

	point
}

pub fn quadratic_population_factory(population_size: usize) -> Vec<Individual> {
  let mut population: Vec<Individual> = Vec::with_capacity(population_size);
	let mut restrictions = vec![(-2.0, 2.0), (-2.0, 2.0)];

  for _ in 0..population_size {
		let chromosome = point_generator(&restrictions);

    population.push(Individual {
      chromosome,
      fitness: f64::MAX,
    });
  }
  population
}

pub fn quadratic_crossover_operator(father: &Individual, mother: &Individual) -> Individual {
  let mean = (father.chromosome[0] + mother.chromosome[0]) / 2 as f64;

  let new_chromosome = vec![mean];
  let new_fitness = quadratic_function(&new_chromosome);

  let mean_individual = Individual {
    chromosome: new_chromosome,
    fitness: new_fitness
  };

  if mean_individual > *father || mean_individual > *mother {
    return mean_individual;
  } else if *father > *mother {
    return father.clone();
  } else {
    return mother.clone();
  }
}

pub fn rastrigin_fitness_function(individual: &Individual) -> f64 {
	let chromosome = individual.chromosome.clone();
  return 10_f64 * chromosome.len() as f64 + chromosome.iter().fold(0_f64, |sum, x| {
    sum + x.powi(2) - 10_f64 * (2_f64 * consts::PI * x).cos()
  })
}

pub fn rastrigin_mutation_operator(individual: &mut Individual) -> Individual {
  let chromosome: &Vec<f64> = &individual.chromosome;
  let idx = thread_rng().gen_range(0..chromosome.len());
  let mut new_chromosome = chromosome.clone();

  let mut rng = thread_rng();
  let mut distribution: Uniform<f64> = Uniform::from(-5.12..5.12);
  let random_value = distribution.sample(&mut rng) as f64;
  new_chromosome[idx] = random_value;
  // let fitness = rastrigin_fitness_function(&new_chromosome);

  Individual {
    chromosome: new_chromosome,
    fitness: f64::MAX,
  }
}

pub fn rastrigin_population_factory(population_size: usize) -> Vec<Individual> {
  let mut rng = thread_rng();
  let distribution: Uniform<f64> = Uniform::from(-5.12..5.12);
  let mut population: Vec<Individual> = Vec::with_capacity(population_size as usize);
  println!("Population size: {}", population_size);
  println!("Length of population vector: {}", population.len());
  for _ in 0..population_size {
    // 2d
    let x = distribution.sample(&mut rng) as f64;
    let y = distribution.sample(&mut rng) as f64;

    population.push(Individual{
      chromosome: vec![x, y],
      // fitness: rastrigin_fitness_function(&vec![x, y]),
      fitness: f64::MAX,
    });
  }
  population
}

pub fn rastrigin_crossover_operator(father: &Individual, mother: &Individual) -> Individual {
  let mut rng = thread_rng();

  let father_chromosome: &Vec<f64> = &father.chromosome;

  let (start, offset) = (rng.gen_range(0..father_chromosome.len()), rng.gen_range(0..father_chromosome.len()));

  let mut offspring: Individual = father.clone();

  for i in start..min(father_chromosome.len(), start + offset) {
    offspring.chromosome[i] = mother.chromosome[i];
  }

  return offspring;
}