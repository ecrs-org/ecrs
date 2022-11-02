mod aco;
mod ga;
mod pso;
mod ff;
mod examples;

fn main() {
  let res = ga::Builder::new()
    .set_max_generation_count(100)
    .set_mutation_rate(0.5f64)
		.set_population_size(100)
		.set_fitness_fn(ga::example::quadratic_fn)
		.set_crossover_operator(ga::operators::crossover::single_point)
		.set_mutation_operator(ga::operators::mutation::range_compliment)
		.set_population_generator(ga::example::quadratic_population_factory)
		.set_eps(0.01)
    .build()
    .run();

	println!("{:?}", res);
}
