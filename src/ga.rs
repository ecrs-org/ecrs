mod individual;
mod probe;
mod example;
mod builder;
mod operators;

pub use individual::Individual;
pub use probe::{Probe};
pub use probe::stdout_probe::{StdoutProbe};
pub use probe::json_probe::{JsonProbe};
pub use probe::csv_probe::{CsvProbe};
pub use example::{*};
pub use builder::*;

use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

type Population = Vec<Individual>;
type FitnessFn = fn(&Individual) -> f64;
type MutationOperator = fn(&mut Individual) -> Individual;
type CrossoverOperator = fn(&Individual, &Individual) -> Individual;
type PopulationGenerator = fn(usize) -> Population;

pub struct GeneticAlgorithmCfg {
  pub mutation_rate: f64,
  pub selection_rate: f64,
  pub generation_upper_bound: i32,
  pub population_size: usize,
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

	fn find_best_individual(population: &Population) -> &Individual {
		debug_assert!(population.len() > 0);
		let mut best_individual = &population[0];
		for i in 1..population.len() {
			if population[i] < *best_individual {
				best_individual = &population[i];
			}
		}
		best_individual
	}

  fn run_old(&mut self) -> Option<Individual> {
    let mut population = (self.config.population_factory)(self.config.population_size);

    for generation_idx in 0..self.config.generation_upper_bound {
      self.config.probe.on_iteration_start(generation_idx as usize);
      let mut new_generation: Vec<Individual> = Vec::with_capacity(self.config.population_size as usize);

      population.iter_mut().for_each(|individual| {
        individual.fitness = (self.config.fitness_fn)(&individual);
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
        individual.fitness = (self.config.fitness_fn)(&individual);
      });

      self.config.probe.on_new_generation(&population);

      if let Some(individual) = population.iter().min() {
        self.config.probe.on_best_fit_in_generation(individual);
        if (self.config.fitness_fn)(&individual) < self.config.eps {
          self.config.probe.on_new_best(individual);
          self.config.probe.on_end();
          return Option::Some((*individual).clone());
        }
      }

      self.config.probe.on_iteration_end(generation_idx as usize);
    }
    self.config.probe.on_end();
    if let Some(individual) = population.iter().min() {
      if (self.config.fitness_fn)(&individual) < self.config.eps {
        return Option::Some((*individual).clone());
      } else {
        None
      }
    } else {
      None
    }
  }

	pub fn run(&mut self) -> Option<Individual> {
		// 1. Create initial random population.
		let mut population = (self.config.population_factory)(self.config.population_size);

		// 2. Evaluate fitness for each individual.
		let mut best_individual: &Individual = &population[0];

		for individual in population {
			individual.fitness = (self.config.fitness_fn)(&individual);

			// Note that here we use "<" operator thus minimizing the fitness function.
			// It is important to make this more generic & document this well.
			if individual < *best_individual {
				best_individual = &individual;
			}
		}

		// 3. Store best individual.
		// Already calculated at step 2.

		if best_individual.fitness < self.config.eps {
			return Some(best_individual.to_owned())
		}

		for generation_no in 0..self.config.generation_upper_bound {
			println!("Calculating generation {}", generation_no);
			// 2. Evaluate fitness for each individual.
			// let mut best_individual: &Individual = &population[0];

			for individual in population {
				individual.fitness = (self.config.fitness_fn)(&individual);
			}

			// 4. Create mating pool by applying selection operator.
			// FIXME: This should be taken from config, but as for now, I'm taking it directly
			// from operators module.
			let mating_pool: Vec<&Individual> = operators::selection::roulette_wheel(&population, population.len());

			// 5. From mating pool create new generation (apply crossover & mutation).
			let mut children: Population = Vec::with_capacity(self.config.population_size);

			// FIXME: Do not assume that population size is an even number.
			for i in (0..mating_pool.len()).step_by(2) {
			// FIXME: This should be taken from config, but as for now, I'm taking it directly
			// from operators module.
				let crt_children = operators::crossover::single_point(mating_pool[i], mating_pool[i + 1]);

				children.push(crt_children.0);
				children.push(crt_children.1);
			}

			// 6. Replacement - merge new generation with old one
			// TODO
			// As for now I'm replacing old population with the new one, but this must be
			// reimplemented. See p. 58 Introduction to Genetic Algorithms.
			population = children;

			// 6. Check for stop condition (Is good enough individual found)? If not goto 2.
			let best_individual = GeneticAlgorithm::find_best_individual(&population);
			if best_individual.fitness < self.config.eps {
				return Some(best_individual.to_owned())
			}
		}

		None
	}
}
