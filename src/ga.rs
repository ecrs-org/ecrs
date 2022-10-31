mod individual;
mod probe;
mod example;
mod builder;

pub use individual::Individual;
pub use probe::{Probe};
pub use probe::stdout_probe::{StdoutProbe};
pub use probe::json_probe::{JsonProbe};
pub use probe::csv_probe::{CsvProbe};
pub use example::{*};
pub use builder::*;

use std::cmp::min;
use rand::{random, Rng, thread_rng};
use rand::rngs::ThreadRng;

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

pub struct GeneticAlgorithm {
  config: GeneticAlgorithmCfg,
  rng: ThreadRng,
}

impl GeneticAlgorithm {
  pub fn new(config: GeneticAlgorithmCfg) -> Self {
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

  pub fn run(&mut self) -> Option<Individual> {
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

      self.config.probe.on_new_generation(&population);

      if let Some(individual) = population.iter().min() {
        self.config.probe.on_best_fit_in_generation(individual);
        if (self.config.fitness_fn)(&individual.chromosome) < self.config.eps {
          self.config.probe.on_new_best(individual);
          self.config.probe.on_end();
          return Option::Some((*individual).clone());
        }
      }

      self.config.probe.on_iteration_end(generation_idx as usize);
    }
    self.config.probe.on_end();
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
