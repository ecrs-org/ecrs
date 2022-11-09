use crate::ga::individual::{ChromosomeWrapper, Chromosome};

pub fn roulette_wheel<T: Chromosome, S: ChromosomeWrapper<T>>(population: &Vec<S>, count: usize) -> Vec<&S> {
	let total_fitness: f64 = population.into_iter()
		.map(|indiv| indiv.get_fitness())
		.sum();

	let mut selected: Vec<&S> = Vec::with_capacity(count);

	for _ in 0..count {
		let threshold = total_fitness * rand::random::<f64>();

		let mut crt_sum = 0.0;
		for indiv in population {
			crt_sum += indiv.get_fitness();

			if crt_sum > threshold {
				selected.push(indiv);
				break;
			}
		}
	}

	selected
}
