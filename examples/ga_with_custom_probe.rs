use std::fmt::Display;

use ecrs::ga::{Individual, Probe};
use itertools::Itertools;

// struct IndividualWrapper<'a>(&'a Individual<Vec<f64>>);

// impl<'a> Display for IndividualWrapper<'a> {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(f, "{{chromosome: {:?}, fitness: {}}}", self.0.chromosome_ref(), self.0.fitness)
//   }
// }

struct PyProbe;

impl Probe<Vec<f64>> for PyProbe {
  fn on_new_generation(
    &mut self,
    metadata: &ecrs::ga::GAMetadata,
    generation: &[ecrs::ga::Individual<Vec<f64>>],
  ) {
    println!(
      "new_gen|{:?}|{}|{:?}",
      metadata.duration.unwrap().as_millis(),
      metadata.generation,
      generation.into_iter().map(|idv| idv.to_json()).collect_vec()
    )
  }

  fn on_new_best(&mut self, metadata: &ecrs::ga::GAMetadata, individual: &ecrs::ga::Individual<Vec<f64>>) {
    println!(
      "new_best|{:?}|{}|{}",
      metadata.duration.unwrap().as_millis(),
      metadata.generation,
      individual.to_json()
    )
  }

  fn on_initial_population_created(&mut self, population: &[ecrs::ga::Individual<Vec<f64>>]) {
    println!(
      "init_gen|{:?}",
      population.into_iter().map(|idv| idv.to_json()).collect_vec()
    )
  }

  fn on_best_fit_in_generation(
    &mut self,
    _metadata: &ecrs::ga::GAMetadata,
    _individual: &ecrs::ga::Individual<Vec<f64>>,
  ) {
    /* noop */
  }

  fn on_end(
    &mut self,
    metadata: &ecrs::ga::GAMetadata,
    population: &[ecrs::ga::Individual<Vec<f64>>],
    best_individual: &ecrs::ga::Individual<Vec<f64>>,
  ) {
    println!(
      "end|{:?}|{}|{}|{:?}",
      metadata.duration.unwrap().as_millis(),
      metadata.generation,
      best_individual.to_json(),
      population.into_iter().map(|idv| idv.to_json()).collect_vec()
    )
  }
}

fn main() {
  let mut solver = ecrs::ga::Builder::new()
    .set_max_duration(std::time::Duration::from_millis(500))
    .set_population_size(2)
    .set_probe(ecrs::ga::probe::PolicyDrivenProbe::new(
      ecrs::ga::probe::GenerationInterval::new(1, 0),
      PyProbe,
    ))
    .set_crossover_operator(ecrs::ga::operators::crossover::TwoPoint::new())
    .set_mutation_operator(ecrs::ga::operators::mutation::Interchange::new())
    .set_selection_operator(ecrs::ga::operators::selection::StochasticUniversalSampling::new())
    .set_replacement_operator(ecrs::ga::operators::replacement::WeakParent::new())
    .set_population_generator(ecrs::ga::population::RandomPoints::with_constraints(
      3,
      vec![-5.12..5.12, -5.12..5.12, -5.12..5.12],
    ))
    .set_fitness_fn(ecrs::test_functions::rastrigin)
    .build();

  solver.run();
}
