use std::ops::Index;

use push_trait::{Push, Nothing};
use rand::Rng;
use crate::ga::individual::{ChromosomeWrapper, Chromosome};

pub trait CrossoverOperator<T: Chromosome, S: ChromosomeWrapper<T>> {
	fn apply(&mut self, parent1: &S, parent2: &S) -> (S, S);
}

pub struct SinglePoint;

impl<GeneT, ChT, ChWrapperT> CrossoverOperator<ChT, ChWrapperT> for SinglePoint
where
	ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
	ChWrapperT: ChromosomeWrapper<ChT>,
	GeneT: Copy
{
	fn apply(&mut self, parent1: &ChWrapperT, parent2: &ChWrapperT) -> (ChWrapperT, ChWrapperT) {
		let chromosome_len = parent1.get_chromosome().len();
		let cut_point = rand::thread_rng().gen_range(0..chromosome_len);

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
}
