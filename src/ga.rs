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

use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

type FitnessFn = fn(&[f64]) -> f64;
type MutationOperator = fn(&mut Individual) -> Individual;
type CrossoverOperator = fn(&Individual, &Individual) -> Individual;
type PopulationGenerator = fn(i32) -> Vec<Individual>;

pub struct GeneticAlgorithmCfg {
  pub mutation_rate: f64,
  pub selection_rate: f64,
  pub generation_upper_bound: i32,
  pub population_size: i32,
  pub eps: f64,
  pub fitness_fn: FitnessFn,
  pub mutation_operator: MutationOperator,
  pub crossover_operator: CrossoverOperator,
  pub population_factory: PopulationGenerator,
  pub probe: Box<dyn Probe>
}

impl Default for GeneticAlgorithmCfg {
  fn default() -> Self {
      GeneticAlgorithmCfg {
        mutation_rate: 0.08f64,
        selection_rate: 0.5f64,
        generation_upper_bound: 200,
        population_size: 100,
        eps: 1e-4,
        fitness_fn: rastrigin_fitness_function,
        mutation_operator: rastrigin_mutation_operator,
        crossover_operator: rastrigin_crossover_operator,
        population_factory: rastrigin_population_factory,
        probe: Box::new(StdoutProbe{}),
      }
  }
}

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

	pub fn run_new(&mut self) -> Option<Individual> {
		// 1. Create initial random population.
		// 2. Evaluate fitness for each individual.
		// 3. Store best individual.
		// 4. Create mating pool by applying selection operator.
		// 5. From mating pool create new generation (apply crossover & mutation).
		// 6. Check for stop condition (Is good enough individual found)? If not goto 2.
		//

		None
	}
}
