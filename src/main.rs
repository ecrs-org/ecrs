use std::cmp::min;
use rand::{Rng, thread_rng};
use std::fmt;
use std::fmt::Formatter;
use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};
use std::f64::consts;
use log::{warn, info};


struct GeneticAlgorithmCfg<T> {
  mutation_rate: f64,
  selection_rate: f64,
  generation_upper_bound: i32,
  population_size: i32,
  eps: f64,
  fitness_fn: fn(&T) -> f64,
  mutation_operator: fn(T) -> T,
  crossover_operator: fn(&T, &T) -> T,
  selection_operator: fn(&[T], usize)-> T,
  population_factory: Option<fn(i32) -> Vec<T>>
}

impl<T> Default for GeneticAlgorithmCfg<T> {
  fn default() -> Self {
    GeneticAlgorithmCfg {
      mutation_rate: 0.08,
      selection_rate: 0.5,
      generation_upper_bound: 200,
      population_size: 100,

    }
  }
}

fn rastrigin_fitness_function(chromosome: &[f64]) -> f64 {
  return 10_f64 * chromosome.len() as f64 + chromosome.iter().fold(0_f64, |sum, x| {
    sum + x.powi(2) - 10_f64 * (2_f64 * consts::PI * x).cos()
  })
}

fn custom_selection_operator<'a>(population: &[&'a[f64]], population_size: usize) -> &'a[f64] {
  // if let Some(choosen_one) = population.iter().map(|x| rastrigin_fitness_function(x) ).min() {
  //   return choosen_one
  // }
  if let Some(choosen_one) = population.get(thread_rng().gen_range(0..population.len())) {
    return choosen_one
  }
  unimplemented!()
}

fn custom_mutation_operator(chromosome: Vec<f64>) -> Vec<f64> {
  let idx = thread_rng().gen_range(0..chromosome.len());
  let mut new_chromosome = chromosome.clone();

  let mut rng = thread_rng();
  let mut distribution: Uniform<f64> = Uniform::from(-5.12..5.12);
  let random_value = distribution.sample(&mut rng) as f64;
  new_chromosome[idx] = random_value;
  new_chromosome
}

fn custom_population_factory(population_size: i32) -> Vec<Vec<f64>> {
  let mut rng = thread_rng();
  let mut distribution: Uniform<f64> = Uniform::from(-5.12..5.12);
  let mut population: Vec<Vec<f64>> = Vec::with_capacity(population_size as usize);
  for i in 0..population_size {
    for _ in 0..2 {
      population[i as usize].push(distribution.sample(&mut rng) as f64);
    }
  }
  population
}

// fn custom_crossover_operator(father: &Vec<f64>, mother: &Vec<f64>) -> Vec<f64> {
//   let mut rng = thread_rng();
//   let (start, offset) = (rng.gen_range(0..father.len()), rng.gen_range(0..father.len()));
//
//   for i in start..min(father.len(), start + offset) {
//
//   }
// }

struct GeneticAlgorithm<T: Ord + Clone> {
  config: GeneticAlgorithmCfg<T>,
  rng: ThreadRng,
  current_best: &T
}

impl<T: Ord + Clone> GeneticAlgorithm<T> {
  fn new(config: GeneticAlgorithmCfg<T>) -> Self {
    GeneticAlgorithm {
      config,
      rng: thread_rng()
    }
  }

  fn maybe_apply_mutation_operator(&self, target: T) -> T {
    // TODO
    (self.config.mutation_operator)(target)
  }

  fn run(&mut self) -> Option<T> {
    let mut population = (self.config.population_factory)(self.config.population_size);

    for generation_idx in 0..self.config.generation_upper_bound {
      let mut new_generation: Vec<T> = Vec::with_capacity(self.config.population_size as usize);

      // TODO: consider to parametrize this
      for _ in 0..(self.config.population_size / 2) {
        let (mut father, mut mother) = (
          (self.config.selection_operator)(&population, self.config.population_size as usize),
          (self.config.selection_operator)(&population, self.config.population_size as usize)
        );

        // TODO: possible optimization
        father = self.maybe_apply_mutation_operator(father);
        mother = self.maybe_apply_mutation_operator(mother);

        let mut child = (self.config.crossover_operator)(&father, &mother);
        new_generation.push(child);
        if self.rng.gen_bool(self.config.selection_rate as f64) {
          new_generation.push(father);
        } else {
          new_generation.push(mother);
        }
      }

      population = new_generation;

      if let Some(chromosome) = population.iter().min() {
        if (self.config.fitness_fn)(chromosome) < self.config.eps {
          return Option::Some((*chromosome).clone());
        }
        println!("Generation: {}, best fit: {}", generation_idx + 1, (self.config.fitness_fn)(chromosome));
      }
    }
    if let Some(chromosome) = population.iter().min() {
      if (self.config.fitness_fn)(chromosome) < self.config.eps {
        return Option::Some((*chromosome).clone());
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
  let alg = GeneticAlgorithm::new(GeneticAlgorithmCfg {
    mutation_rate: 0.1,
    selection_rate: 0.3,
    generation_upper_bound: 200,
    population_size: 100,
    eps: 0.001,
    ..GeneticAlgorithmCfg::default(),
  })

  let data_provider = DataProviderFromJSON::new();
  let output_handler = OutpuHandlerCSV::new();

  let pipeline = Pipeline::new(data_provider, alg, output_handler);

  pipeline.execute();



  println!("Hello world");
}
