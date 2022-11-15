pub mod rastrigin;
pub mod sum_of_squares;

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
    .set_mutation_rate(0.5f64)
		.set_population_size(100)
		.set_fitness_fn(rastrigin::rastrigin_fitness)
		.set_crossover_operator(Box::new(ga::operators::crossover::SinglePoint::new()))
		.set_mutation_operator(Box::new(ga::operators::mutation::Identity::new()))
		.set_population_generator(rastrigin::rastrigin_population_factory)
		.set_selection_operator(Box::new(ga::operators::selection::Boltzmann::new(0.05, 80.0, 500, false)))
		.set_probe(Box::new(ga::probe::stdout_probe::StdoutProbe{}))
    .build()
    .run();

	println!("{:?}", res);
}