use len_trait::Len;

use std::ops::Index;

use crate::ga::individual::IndividualTrait;
use crate::ga::GAMetadata;
use push_trait::{Nothing, Push};

use rand::{rngs::ThreadRng, Rng};

use super::CrossoverOperator;

/// # Mutli-point crossover operator
///
/// This struct implements [CrossoverOperator] and can be used with GA.
///
/// It works analogously to [SinglePoint] or [TwoPoint]. One important difference is that
/// all cutpoints are distinct, thus single or two point crossover with guarantee of distinct cutpoints
/// can be achieved.
pub struct MultiPoint<R: Rng = ThreadRng> {
    cut_points_no: usize,
    rng: R,
}

impl MultiPoint<ThreadRng> {
    /// Creates new [MultiPoint] crossover operator with default RNG
    ///
    /// ## Arguments
    ///
    /// * `cut_points_no` - Number of cutpoints (crossover points)
    pub fn new(cut_points_no: usize) -> Self {
        Self::with_rng(cut_points_no, rand::thread_rng())
    }
}

impl Default for MultiPoint<ThreadRng> {
    /// Creates new [MultiPoint] crossover operator with 4 cutpoints and default RNG
    fn default() -> Self {
        Self::with_rng(4, rand::thread_rng())
    }
}

impl<R: Rng> MultiPoint<R> {
    /// Creates new [MultiPoint] crossover operator with custom RNG
    ///
    /// ## Arguments
    ///
    /// * `cut_points_no` - Number of cutpoints (crossover points)
    /// * `rng` - Custom random number generator
    pub fn with_rng(cut_points_no: usize, rng: R) -> Self {
        assert!(cut_points_no >= 1, "Number of cut points must be >= 1");
        Self { cut_points_no, rng }
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for MultiPoint<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy,
    R: Rng,
{
    /// Returns a tuple of children
    ///
    /// It works analogously to [SinglePoint] or [TwoPoint]. One important difference is that
    /// all cutpoints are distinct, thus single or two point crossover with guarantee of distinct cutpoints
    /// can be achieved.
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in recombination
    /// * `parent_2` - Second parent to take part in recombination
    fn apply_legacy(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT) {
        assert_eq!(
            parent_1.chromosome().len(),
            parent_2.chromosome().len(),
            "Parent chromosome length must match"
        );
        assert!(
            self.cut_points_no <= parent_1.chromosome().len(),
            "There can't be more cut points than chromosome length"
        );
        assert!(self.cut_points_no >= 1, "Numver of cut points must be >= 1");

        let chromosome_len = parent_1.chromosome().len();

        let mut cut_points =
            rand::seq::index::sample(&mut self.rng, chromosome_len, self.cut_points_no).into_vec();
        cut_points.sort_unstable();

        let mut child_1_ch = IndividualT::ChromosomeT::default();
        let mut child_2_ch = IndividualT::ChromosomeT::default();

        let (mut curr_parent_1, mut curr_parent_2) = (&parent_1, &parent_2);

        for locus in 0..cut_points[0] {
            child_1_ch.push(parent_1.chromosome()[locus]);
            child_2_ch.push(parent_2.chromosome()[locus]);
            (curr_parent_1, curr_parent_2) = (curr_parent_2, curr_parent_1);
        }

        for cut_point_idx in 0..self.cut_points_no - 1 {
            for locus in cut_points[cut_point_idx]..cut_points[cut_point_idx + 1] {
                child_1_ch.push(curr_parent_1.chromosome()[locus]);
                child_2_ch.push(curr_parent_2.chromosome()[locus]);
            }
            (curr_parent_1, curr_parent_2) = (curr_parent_2, curr_parent_1);
        }

        for locus in cut_points[self.cut_points_no - 1]..chromosome_len {
            child_1_ch.push(curr_parent_1.chromosome()[locus]);
            child_2_ch.push(curr_parent_2.chromosome()[locus]);
        }

        (IndividualT::from(child_1_ch), IndividualT::from(child_2_ch))
    }

    fn apply(&mut self, metadata: &GAMetadata, selected: &[&IndividualT], output: &mut Vec<IndividualT>) {
        assert!(selected.len() & 1 == 0);
        for parents in selected.chunks(2) {
            let (child_1, child_2) = self.apply_legacy(metadata, parents[0], parents[1]);
            output.push(child_1);
            output.push(child_2);
        }
    }
}
