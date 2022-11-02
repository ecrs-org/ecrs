use crate::ga::{Population, Individual};

pub fn roulette_wheel(population: &Population, count: usize) -> Vec<&Individual> {
	let total_fitness: f64 = population.into_iter()
		.map(|individual| individual.fitness)
		.sum();

	let mut selected: Vec<&Individual> = Vec::with_capacity(count);

	for _ in 0..count {
		let border = total_fitness * rand::random::<f64>();

		let mut crt_sum = 0.0;
		for individual in population {
			crt_sum += individual.fitness;

			if crt_sum > border {
				selected.push(individual);
				break;
			}
		}
	}

	selected
}
