mod probe;
mod individual;

use std::cmp::min;
use rand::{random, Rng, thread_rng};
use std::fmt;
use std::fmt::Formatter;
use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};
use std::f64::consts;
use log::{warn, info};
use crate::probe::{Probe, GAStdoutProbe};
use crate::individual::Individual;


pub struct GeneticAlgorithmCfg {
  pub mutation_rate: f64,
  pub selection_rate: f64,
  pub generation_upper_bound: i32,
  pub population_size: i32,
  pub eps: f64,
  pub fitness_fn: fn(&[f64]) -> f64,
  pub mutation_operator: fn(&mut Individual) -> Individual,
  pub crossover_operator: fn(&Individual, &Individual) -> Individual,
  pub population_factory: fn(i32) -> Vec<Individual>,
  pub probe: Box<dyn Probe>
}

// impl<T> Default for GeneticAlgorithmCfg<T> {
//   fn default() -> Self {
//     GeneticAlgorithmCfg {
//       mutation_rate: 0.08,
//       selection_rate: 0.5,
//       generation_upper_bound: 200,
//       population_size: 100,
//       fitness_fn: rastrigin_fitness_function,
//       mutation_operator: custom_mutation_operator,
//       population_factory: custom_population_factory,
//       eps: 1e-4,
//       probe: Box::new(GAStdoutProbe{}),
//       crossover_operator: custom_crossover_operator
//     }
//   }
// }

fn quadratic_function(chromosome: &[f64]) -> f64 {
  return chromosome[0] * chromosome[0]; // + 3 as f64 * chromosome[0];
}

fn quadratic_mutation_operator(individual: &mut Individual) -> Individual {
  let idx = thread_rng().gen_range(0..individual.chromosome.len());
  let mut new_chromosome = individual.chromosome.clone();
  let mut rng = thread_rng();

  let mut distribution: Uniform<f64> = Uniform::from(-10_f64..10_f64);
  let random_value = distribution.sample(&mut rng) as f64;

  println!("RANDOM VALUE: {}", random_value);

  new_chromosome[idx] = random_value;
  // let
  Individual {
    chromosome: new_chromosome,
    fitness: f64::MAX,
  }
}

fn quadratic_population_factory(population_size: i32) -> Vec<Individual> {
  let mut rng = thread_rng();
  let distribution: Uniform<f64> = Uniform::from(-10_f64..10_f64);

  let mut population: Vec<Individual> = Vec::with_capacity(population_size as usize);

  for _ in 0..population_size {
    let x = distribution.sample(&mut rng) as f64;

    population.push(Individual {
      chromosome: vec![x],
      fitness: f64::MAX,
    });
  }
  population
}

fn quadratic_crossover_operator(father: &Individual, mother: &Individual) -> Individual {
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

fn rastrigin_fitness_function(chromosome: &[f64]) -> f64 {
  return 10_f64 * chromosome.len() as f64 + chromosome.iter().fold(0_f64, |sum, x| {
    sum + x.powi(2) - 10_f64 * (2_f64 * consts::PI * x).cos()
  })
}

fn rastrigin_mutation_operator(individual: &mut Individual) -> Individual {
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

fn rastrigin_population_factory(population_size: i32) -> Vec<Individual> {
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

fn rastrigin_crossover_operator(father: &Individual, mother: &Individual) -> Individual {
  let mut rng = thread_rng();

  let father_chromosome: &Vec<f64> = &father.chromosome;

  let (start, offset) = (rng.gen_range(0..father_chromosome.len()), rng.gen_range(0..father_chromosome.len()));

  let mut offspring: Individual = father.clone();

  for i in start..min(father_chromosome.len(), start + offset) {
    offspring.chromosome[i] = mother.chromosome[i];
  }

  return offspring;
}

struct GeneticAlgorithm {
  config: GeneticAlgorithmCfg,
  rng: ThreadRng,
}

impl GeneticAlgorithm {
  fn new(config: GeneticAlgorithmCfg) -> Self {
    GeneticAlgorithm {
      config,
      rng: thread_rng(),
    }
  }

  fn maybe_apply_mutation_operator(&mut self, target: &mut Individual, probability: f64) -> Individual {
    if self.rng.gen_range(0f64..1f64) < probability {
      let target_copy = target.clone();
      let result = (self.config.mutation_operator)(target);
      self.config.probe.on_mutation(&target_copy, &result);
      return result;
    }
    return target.clone();
  }

  fn select_individual_from_n_random(&mut self, population: &Vec<Individual>, n: usize) -> Individual {
    if let Some(selected) = (0..n).map(|_| population[self.rng.gen_range(0..population.len())].clone()).min() {
      selected
    } else {
      unimplemented!()
    }
  }

  // fn evaluate_population(&mut self, population: &Vec<&mut Individual>) {
  //   for individual in *population {
  //     individual.fitness = (self.config.fitness_fn)(&individual.chromosome);
  //   }
  // }

  fn run(&mut self) -> Option<Individual> {
    let mut population = (self.config.population_factory)(self.config.population_size);

    for generation_idx in 0..self.config.generation_upper_bound {
      self.config.probe.on_iteration_start(generation_idx as usize);
      let mut new_generation: Vec<Individual> = Vec::with_capacity(self.config.population_size as usize);

      population.iter_mut().for_each(|individual| {
        individual.fitness = (self.config.fitness_fn)(&individual.chromosome);
      });

      // TODO: consider to parametrize this
      for _ in 0..(self.config.population_size / 2) {
        let (mut father, mut mother) = (
            self.select_individual_from_n_random(&population, (self.config.selection_rate * self.config.population_size as f64) as usize),
            self.select_individual_from_n_random(&population, (self.config.selection_rate * self.config.population_size as f64) as usize)
        );

        // TODO: possible optimization
        father = self.maybe_apply_mutation_operator(&mut father, self.config.mutation_rate);
        mother = self.maybe_apply_mutation_operator(&mut mother, self.config.mutation_rate);

        let child = (self.config.crossover_operator)(&father, &mother);
        new_generation.push(child);
        if self.rng.gen_bool(self.config.selection_rate as f64) {
          new_generation.push(father);
        } else {
          new_generation.push(mother);
        }
      }

      population = new_generation;

      population.iter_mut().for_each(|individual| {
        individual.fitness = (self.config.fitness_fn)(&individual.chromosome);
      });

      self.config.probe.on_new_generation();

      if let Some(individual) = population.iter().min() {
        self.config.probe.on_best_fit_in_generation(individual);
        if (self.config.fitness_fn)(&individual.chromosome) < self.config.eps {
          self.config.probe.on_new_best(individual);
          return Option::Some((*individual).clone());
        }
      }

      self.config.probe.on_iteration_end(generation_idx as usize);
    }
    if let Some(individual) = population.iter().min() {
      if (self.config.fitness_fn)(&individual.chromosome) < self.config.eps {
        return Option::Some((*individual).clone());
      } else {
        None
      }
    } else {
      None
    }
  }
}

// struct GeneticAlgorithmCfg<T> {
//   mutation_rate: f64,
//   selection_rate: f64,
//   generation_upper_bound: i32,
//   population_size: i32,
//   eps: f64,
//   fitness_fn: fn(&T) -> f64,
//   mutation_operator: fn(T) -> T,
//   crossover_operator: fn(&T, &T) -> T,
//   selection_operator: fn(&[T], usize)-> T,
//   population_factory: fn(i32) -> Vec<T>,
// }

fn main() {
  let mut alg = GeneticAlgorithm::new(GeneticAlgorithmCfg {
      mutation_rate: 0.08,
      selection_rate: 0.5,
      generation_upper_bound: 200,
      population_size: 100,
      fitness_fn: rastrigin_fitness_function,
      mutation_operator: rastrigin_mutation_operator,
      population_factory: rastrigin_population_factory,
      eps: 1e-4,
      probe: Box::new(GAStdoutProbe{}),
      crossover_operator: rastrigin_crossover_operator
  });

  // alg.run();


  let mut alg_quadratic = GeneticAlgorithm::new(GeneticAlgorithmCfg {
    mutation_rate: 0.1,
    selection_rate: 0.5,
    generation_upper_bound: 200,
    population_size: 400,
    fitness_fn: quadratic_function,
    mutation_operator: quadratic_mutation_operator,
    population_factory: quadratic_population_factory,
    eps: 1e-4,
    probe: Box::new(GAStdoutProbe{}),
    crossover_operator: quadratic_crossover_operator,
  });

  alg_quadratic.run();

  // let arg = vec![1f64, 2f64, 2.4f64];
  // let result = rastrigin_fitness_function(&arg);
  // println!("Result: {}", result);

  // println!("Hello world");
}
