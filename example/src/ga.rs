pub mod rastrigin;
pub mod sum_of_squares;
pub mod wordmax;

use ecrs::ga;

pub fn point_generator(restrictions: &Vec<(f64, f64)>) -> Vec<f64> {
  assert!(!restrictions.is_empty());

  let mut point: Vec<f64> = Vec::with_capacity(restrictions.len());

  for restriction in restrictions {
    point.push(restriction.1 * rand::random::<f64>() + restriction.0);
  }

  point
}

pub fn ga_example() {
  let res = ecrs::ga::Builder::new()
    .set_max_generation_count(500)
    .set_population_size(100)
    .set_fitness_fn(rastrigin::rastrigin_fitness)
    .set_crossover_operator(ga::operators::crossover::SinglePoint::new())
    .set_mutation_operator(ga::operators::mutation::Identity::new())
    .set_population_generator(ga::population::RandomPoints::with_constraints(
      3,
      vec![-5.12..5.12, -5.12..5.12, -5.12..5.12],
    ))
    .set_selection_operator(ga::operators::selection::Boltzmann::new(0.05, 80.0, 500, false))
    .set_probe(ga::probe::stdout_probe::StdoutProbe)
    .build()
    .run();

  println!("{:?}", res);
}

pub fn ga_rvc_example() -> Option<ga::Individual<Vec<f64>>> {
  ecrs::ga::Builder::with_rvc()
    .fitness_fn(rastrigin::rastrigin_fitness)
    .dim(5)
    .build()
    .run()
}

pub fn ga_bsc_example() -> Option<ga::Individual<Vec<bool>>> {
  ecrs::ga::Builder::with_bsc()
    .fitness_fn(wordmax::wordmax_fitness)
    .dim(10)
    .population_size(6)
    .build()
    .run()
}

pub fn ga_exmaple_test_functions() -> Option<ga::Individual<Vec<f64>>> {
  ecrs::ga::Builder::with_rvc()
    .fitness_fn(ecrs::test_functions::ackley)
    .dim(4)
    .build()
    .run()
}
