use std::ops::Index;

use push_trait::{Push, Nothing};
use rand::{Rng, thread_rng};
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

pub struct MultiPoint {
	cut_points_no: usize,
}

impl MultiPoint {
	pub fn new(cut_points_no: usize) -> Self {
		assert!(cut_points_no >= 1, "Number of cut points must be >= 1");
		MultiPoint {
			cut_points_no
		}
	}
}

impl Default for MultiPoint {
	fn default() -> Self {
		MultiPoint { cut_points_no: 4 }
	}
}

impl<GeneT, ChT, ChWrapperT> CrossoverOperator<ChT, ChWrapperT> for MultiPoint
where
	ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
	ChWrapperT: ChromosomeWrapper<ChT>,
	GeneT: Copy
{
	fn apply(&mut self, parent_1: &ChWrapperT, parent_2: &ChWrapperT) -> (ChWrapperT, ChWrapperT) {
		assert_eq!(parent_1.get_chromosome().len(), parent_2.get_chromosome().len(), "Parent chromosome length must match");
		assert!(self.cut_points_no <= parent_1.get_chromosome().len(), "There can't be more cut points than chromosome length");
		assert!(self.cut_points_no >= 1, "Numver of cut points must be >= 1");

		let chromosome_len = parent_1.get_chromosome().len();

		let mut cut_points = rand::seq::index::sample(&mut rand::thread_rng(), chromosome_len, self.cut_points_no).into_vec();
		cut_points.sort_unstable();

		let mut child_1 = ChWrapperT::new();
		let mut child_2 = ChWrapperT::new();

		let (mut curr_parent_1, mut curr_parent_2) = (&parent_1, &parent_2);

		for locus in 0..cut_points[0] {
			child_1.get_chromosome_mut().push(parent_1.get_chromosome()[locus]);
			child_2.get_chromosome_mut().push(parent_2.get_chromosome()[locus]);
			(curr_parent_1, curr_parent_2) = (curr_parent_2, curr_parent_1);
		}

		for cut_point_idx in 0..self.cut_points_no - 1 {
			for locus in cut_points[cut_point_idx]..cut_points[cut_point_idx + 1] {
				child_1.get_chromosome_mut().push(curr_parent_1.get_chromosome()[locus]);
				child_2.get_chromosome_mut().push(curr_parent_2.get_chromosome()[locus]);
			}
			(curr_parent_1, curr_parent_2) = (curr_parent_2, curr_parent_1);
		}

		for locus in cut_points[self.cut_points_no - 1]..chromosome_len {
			child_1.get_chromosome_mut().push(curr_parent_1.get_chromosome()[locus]);
			child_2.get_chromosome_mut().push(curr_parent_2.get_chromosome()[locus]);
		}

		(child_1, child_2)
	}
}

pub struct Uniform;

impl Uniform {
	pub fn new() -> Self {
		Uniform { }
	}
}

impl<GeneT, ChT, ChWrapperT> CrossoverOperator<ChT, ChWrapperT> for Uniform
where
	ChT: Chromosome + Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
	ChWrapperT: ChromosomeWrapper<ChT>,
	GeneT: Copy
{
	fn apply(&mut self, parent_1: &ChWrapperT, parent_2: &ChWrapperT) -> (ChWrapperT, ChWrapperT) {
		assert_eq!(parent_1.get_chromosome().len(), parent_2.get_chromosome().len(), "Parent chromosome length must match");

		let chromosome_len = parent_1.get_chromosome().len();

		let mut child_1 = ChWrapperT::new();
		let mut child_2 = ChWrapperT::new();

		let mask = rand::thread_rng().sample_iter(rand::distributions::Uniform::new(0.0, 1.0)).take(chromosome_len);

		for (locus, val) in mask.enumerate() {
			if val >= 0.5 {
				child_1.get_chromosome_mut().push(parent_1.get_chromosome()[locus]);
				child_2.get_chromosome_mut().push(parent_2.get_chromosome()[locus]);
			} else {
				child_1.get_chromosome_mut().push(parent_2.get_chromosome()[locus]);
				child_2.get_chromosome_mut().push(parent_1.get_chromosome()[locus]);
			}
		}

		(child_1, child_2)
	}
}
