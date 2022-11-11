use rand::Rng;

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

pub fn random<T: Chromosome, S: ChromosomeWrapper<T>>(population: &Vec<S>, count: usize) -> Vec<&S> {
	// We must use index API, as we want to return vector of references, not vector of actual items
	let indices = rand::seq::index::sample(&mut rand::thread_rng(), population.len(), count);
	let mut selected: Vec<&S> = Vec::with_capacity(count);

	for i in indices {
		selected.push(&population[i]);
	}

	selected
}

pub fn rank<T: Chromosome, S: ChromosomeWrapper<T>>(population: &Vec<S>, count: usize) -> Vec<&S> {
	// TODO: Second implementation with r parameter

	let mut selected: Vec<&S> = Vec::with_capacity(count);

	let population_len = population.len();
	for _ in 0..count {
		// TODO: Consider creating two random index permutations and then iterating over them
		// instead of N times using random.
		let p1 = & population[rand::thread_rng().gen_range(0..population_len)];
		let p2 = &population[rand::thread_rng().gen_range(0..population_len)];

		selected.push(
			if p1.get_fitness() >= p2.get_fitness() {
				p1
			} else {
				p2
			}
		)
	}

	selected
}
