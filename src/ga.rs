mod individual;
mod probe;
pub mod example;
pub mod builder;
pub mod operators;

pub use individual::Individual;
pub use probe::Probe;
pub use probe::stdout_probe::StdoutProbe;
pub use probe::json_probe::JsonProbe;
pub use probe::csv_probe::CsvProbe;
pub use example::*;
pub use builder::*;

use self::individual::{Chromosome, ChromosomeWrapper, Gene};

type FitnessFn<T: Gene, S: ChromosomeWrapper<T>> = fn(&S) -> f64;
type MutationOperator<T: Gene, S: ChromosomeWrapper<T>> = fn(&mut S) -> Individual<T>;
type CrossoverOperator<T: Gene, S: ChromosomeWrapper<T>> = fn(&S, &S) -> (S, S);
type PopulationGenerator<T: Gene, S: ChromosomeWrapper<T>> = fn(usize) -> Vec<S>;

pub struct GeneticAlgorithmCfg<T: Gene, S: ChromosomeWrapper<T>> {
  pub mutation_rate: f64,
  pub selection_rate: f64,
  pub generation_upper_bound: i32,
  pub population_size: usize,
  pub eps: f64,
  pub fitness_fn: FitnessFn<T, S>,
  pub mutation_operator: MutationOperator<T, S>,
  pub crossover_operator: CrossoverOperator<T, S>,
  pub population_factory: PopulationGenerator<T, S>,
  pub probe: Box<dyn Probe<T, S>>
}

impl<T: Gene, S: ChromosomeWrapper<T>> Default for GeneticAlgorithmCfg<T, S> {
  fn default() -> Self {
      GeneticAlgorithmCfg {
        mutation_rate: 0.08f64,
        selection_rate: 0.5f64,
        generation_upper_bound: 200,
        population_size: 100,
        eps: 1e-4,
        fitness_fn: quadratic_fn,
        mutation_operator: operators::mutation::range_compliment,
        crossover_operator: operators::crossover::single_point,
        population_factory: quadratic_population_factory,
        probe: Box::new(StdoutProbe{}),
      }
  }
}

pub struct GeneticAlgorithm<T: Gene, S: ChromosomeWrapper<T>> {
  config: GeneticAlgorithmCfg<T, S>,
}

impl<T: Gene, S: ChromosomeWrapper<T>> GeneticAlgorithm<T, S> {
  pub fn new(config: GeneticAlgorithmCfg<T, S>) -> Self {
    GeneticAlgorithm {
      config,
    }
  }

	fn find_best_individual(population: &Vec<S>) -> &S {
		debug_assert!(population.len() > 0);
		let mut best_individual = &population[0];
		for i in 1..population.len() {
			if population[i] < *best_individual {
				best_individual = &population[i];
			}
		}
		best_individual
	}

	fn evaluate_fitness_in_population(&self, population: &mut Vec<S>) -> () {
		for i in 0..population.len() {
			population[i].fitness = (self.config.fitness_fn)(&population[i]);
		}
	}

	pub fn run(&mut self) -> Option<S> {
		// 1. Create initial random population.
		let mut population = (self.config.population_factory)(self.config.population_size);

		// 2. Evaluate fitness for each individual.
		self.evaluate_fitness_in_population(&mut population);

		// 3. Store best individual.
		let best_individual = GeneticAlgorithm::find_best_individual(&population);

		if best_individual.get_fitness() < self.config.eps {
			return Some(best_individual.to_owned())
		}

		for generation_no in 0..self.config.generation_upper_bound {
			println!("Calculating generation {}", generation_no);

			// 2. Evaluate fitness for each individual.
			self.evaluate_fitness_in_population(&mut population);

			// 4. Create mating pool by applying selection operator.
			// FIXME: This should be taken from config, but as for now, I'm taking it directly
			// from operators module.
			let mating_pool: Vec<&S> = operators::selection::roulette_wheel(&population, population.len());

			// 5. From mating pool create new generation (apply crossover & mutation).
			let mut children: Vec<S> = Vec::with_capacity(self.config.population_size);

			// FIXME: Do not assume that population size is an even number.
			for i in (0..mating_pool.len()).step_by(2) {
				// FIXME: This should be taken from config, but as for now, I'm taking it directly
				// from operators module.
				let crt_children = operators::crossover::single_point(mating_pool[i], mating_pool[i + 1]);

				children.push(crt_children.0);
				children.push(crt_children.1);
			}

			// 5.1 Here we should apply the mutations on children?

			// 6. Replacement - merge new generation with old one
			// TODO
			// As for now I'm replacing old population with the new one, but this must be
			// reimplemented. See p. 58 Introduction to Genetic Algorithms.
			population = children;

			// 6. Check for stop condition (Is good enough individual found)? If not goto 2.
			self.evaluate_fitness_in_population(&mut population);
			let best_individual = GeneticAlgorithm::find_best_individual(&population);
			if best_individual.fitness < self.config.eps {
				return Some(best_individual.to_owned())
			}
		}

		None
	}
}
