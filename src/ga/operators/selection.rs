use rand::Rng;

use crate::ga::individual::{ChromosomeWrapper, Chromosome};

pub trait SelectionOperator<T: Chromosome, S: ChromosomeWrapper<T>> {
	fn apply<'a>(&mut self, population: &'a Vec<S>, count: usize) -> Vec<&'a S>;
}

pub struct RouletteWheel;

impl RouletteWheel {
	pub fn new() -> Self {
		RouletteWheel { }
	}
}

impl<T: Chromosome, S: ChromosomeWrapper<T>> SelectionOperator<T, S> for RouletteWheel {
	fn apply<'a> (&mut self, population: &'a Vec<S>, count: usize) -> Vec<&'a S> {
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
}

pub struct Random;

impl Random {
	pub fn new() -> Self {
		Random { }
	}
}

impl<T: Chromosome, S: ChromosomeWrapper<T>> SelectionOperator<T, S> for Random {
	fn apply<'a>(&mut self, population: &'a Vec<S>, count: usize) -> Vec<&'a S> {
		// We must use index API, as we want to return vector of references, not vector of actual items
		let indices = rand::seq::index::sample(&mut rand::thread_rng(), population.len(), count);
		let mut selected: Vec<&S> = Vec::with_capacity(count);

		for i in indices {
			selected.push(&population[i]);
		}
		selected
	}
}

pub struct Rank;

impl Rank {
	pub fn new() -> Self {
		Rank { }
	}
}

impl<T: Chromosome, S: ChromosomeWrapper<T>> SelectionOperator<T, S> for Rank {
	fn apply<'a>(&mut self, population: &'a Vec<S>, count: usize) -> Vec<&'a S> {
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
}

pub struct Tournament;

impl Tournament {
	pub fn new() -> Self {
		Tournament { }
	}
}

impl<T: Chromosome, S: ChromosomeWrapper<T>> SelectionOperator<T, S> for Tournament {
	fn apply<'a>(&mut self, population: &'a Vec<S>, count: usize) -> Vec<&'a S> {
		// TODO: This operator must be parametrized...
		// For now I fix value of this parameter
		let tournament_size = population.len() / 5;

		assert!(tournament_size > 0);

		let mut selected: Vec<&S> = Vec::with_capacity(count);

		for _ in 0..count {
			let tournament_indices = rand::seq::index::sample(&mut rand::thread_rng(), population.len(), tournament_size);
			// FIXME: Check wheter the tournament_indices is empty or handle option below.
			let best_idv  = tournament_indices.into_iter().map(|i| &population[i]).max().unwrap();
			selected.push(best_idv);
		}

		selected
	}
}

pub struct StochasticUniversalSampling;

impl StochasticUniversalSampling {
	pub fn new() -> Self {
		StochasticUniversalSampling { }
	}
}

impl<T: Chromosome, S: ChromosomeWrapper<T>> SelectionOperator<T, S> for StochasticUniversalSampling {
	fn apply<'a>(&mut self, population: &'a Vec<S>, count: usize) -> Vec<&'a S> {
		let total_fitness: f64 = population.into_iter()
			.map(|indiv| indiv.get_fitness())
			.sum();

		let mut selected: Vec<&S> = Vec::with_capacity(count);

		let distance_between_pointers = total_fitness / (count as f64);

		assert!(distance_between_pointers > 0.0);

		let mut pointer_pos = rand::thread_rng().gen_range(0.0..=distance_between_pointers);

		let mut curr_sum = 0.0;
		for idv in population {
			curr_sum += idv.get_fitness();

			while curr_sum >= pointer_pos {
				selected.push(idv);
				pointer_pos += distance_between_pointers;
			}
		}

		assert_eq!(selected.len(), count);

		selected
	}
}
