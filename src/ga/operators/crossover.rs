use rand::Rng;
use crate::ga::individual::{ChromosomeWrapper, Gene};

pub fn single_point<T: Gene, S: ChromosomeWrapper<T>>(parent1: &S, parent2: &S) -> (S, S) {
	let chromosome_len = parent1.get_chromosome().len();
	let cut_point = rand::thread_rng().gen_range(0..chromosome_len);

	println!("Cut point: {}", cut_point);

	let mut child1 = S::new();
	let mut child2 = S::new();

	for locus in 0..cut_point {
		child1.get_chromosome_mut().push(parent1.get_chromosome()[locus]);
		child2.get_chromosome_mut().push(parent2.get_chromosome()[locus]);
	}

	for locus in cut_point..chromosome_len {
		child1.get_chromosome_mut().push(parent2.get_chromosome()[locus]);
		child2.get_chromosome_mut().push(parent1.get_chromosome()[locus])
	}

	(child1, child2)
}
