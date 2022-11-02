use rand::Rng;

use crate::ga::Individual;

pub fn single_point(parent1: &Individual, parent2: &Individual) -> (Individual, Individual) {
	// FIXME: Handle cases when 0 or parent1.chromosome.len() - 1 are selected (there is only 1 child then)
	// thus we must handle such case manually to produce second children.

	debug_assert!(parent1.chromosome.len() == parent2.chromosome.len());

	let chromosome_len = parent1.chromosome.len();

	let cut_point: usize = rand::thread_rng().gen_range(0..chromosome_len);

	let mut child1_chromosome = Vec::with_capacity(chromosome_len);
	let mut child2_chromosome = Vec::with_capacity(chromosome_len);

	for locus in 0..cut_point {
		child1_chromosome.push(parent1.chromosome[locus]);
		child2_chromosome.push(parent2.chromosome[locus]);
	}

	for locus in cut_point..chromosome_len {
		child1_chromosome.push(parent2.chromosome[locus]);
		child2_chromosome.push(parent1.chromosome[locus]);
	}


	(
		Individual {
			chromosome: child1_chromosome,
			fitness: 0.0,
		},
		Individual {
			chromosome: child2_chromosome,
			fitness: 0.0,
		},
	)
}
