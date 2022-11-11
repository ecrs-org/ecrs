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
	fn apply(&mut self, parent_1: &ChWrapperT, parent_2: &ChWrapperT) -> (ChWrapperT, ChWrapperT) {
		let chromosome_len = parent_1.get_chromosome().len();
		let cut_point = rand::thread_rng().gen_range(0..chromosome_len);

		let mut child_1 = ChWrapperT::new();
		let mut child_2 = ChWrapperT::new();

		for locus in 0..cut_point {
			child_1.get_chromosome_mut().push(parent_1.get_chromosome()[locus]);
			child_2.get_chromosome_mut().push(parent_2.get_chromosome()[locus]);
		}

		for locus in cut_point..chromosome_len {
			child_1.get_chromosome_mut().push(parent_2.get_chromosome()[locus]);
			child_2.get_chromosome_mut().push(parent_1.get_chromosome()[locus]);
		}

		(child_1, child_2)
	}
}

pub struct TwoPoint;

impl<GeneT, ChT, ChWrapperT> CrossoverOperator<ChT, ChWrapperT> for TwoPoint
where
	ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
	ChWrapperT: ChromosomeWrapper<ChT>,
	GeneT: Copy
{
	fn apply(&mut self, parent_1: &ChWrapperT, parent_2: &ChWrapperT) -> (ChWrapperT, ChWrapperT) {
		assert_eq!(parent_1.get_chromosome().len(), parent_2.get_chromosome().len(), "Parent chromosome length must match");

		let chromosome_len = parent_1.get_chromosome().len();

		let cut_points = (rand::thread_rng().gen_range(0..chromosome_len), rand::thread_rng().gen_range(0..chromosome_len));

		let (cut_point_1, cut_point_2) = if cut_points.0 <= cut_points.1 {
			(cut_points.0, cut_points.1)
		} else {
			(cut_points.1, cut_points.0)
		};

		let mut child_1 = ChWrapperT::new();
		let mut child_2 = ChWrapperT::new();

		for locus in 0..cut_point_1 {
			child_1.get_chromosome_mut().push(parent_1.get_chromosome()[locus]);
			child_2.get_chromosome_mut().push(parent_2.get_chromosome()[locus]);
		}

		for locus in cut_point_1..cut_point_2 {
			child_1.get_chromosome_mut().push(parent_2.get_chromosome()[locus]);
			child_2.get_chromosome_mut().push(parent_1.get_chromosome()[locus]);

		}

		for locus in cut_point_2..chromosome_len {
			child_1.get_chromosome_mut().push(parent_1.get_chromosome()[locus]);
			child_2.get_chromosome_mut().push(parent_2.get_chromosome()[locus]);
		}

		(child_1, child_2)
	}
}
