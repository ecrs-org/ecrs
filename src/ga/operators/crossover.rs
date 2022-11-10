use std::ops::Index;

use push_trait::{Push, Nothing};
use rand::Rng;
use crate::ga::individual::{ChromosomeWrapper, Chromosome};

pub fn single_point<GeneT, ChT, ChWrapperT>(
	parent1: &ChWrapperT,
	parent2: &ChWrapperT) -> (ChWrapperT, ChWrapperT)
where
	ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
	ChWrapperT: ChromosomeWrapper<ChT>,
	GeneT: Copy
{
	let chromosome_len = parent1.get_chromosome().len();
	let cut_point = rand::thread_rng().gen_range(0..chromosome_len);

	println!("Cut point: {}", cut_point);

	let mut child1 = ChWrapperT::new();
	let mut child2 = ChWrapperT::new();

	for locus in 0..cut_point {
		child1.get_chromosome_mut().push(parent1.get_chromosome()[locus]);
		child2.get_chromosome_mut().push(parent2.get_chromosome()[locus]);
	}

	for locus in cut_point..chromosome_len {
		child1.get_chromosome_mut().push(parent2.get_chromosome()[locus]);
		child2.get_chromosome_mut().push(parent1.get_chromosome()[locus]);
	}

	(child1, child2)
}
