use len_trait::Len;
use std::ops::{Index, IndexMut};

use crate::ga::individual::{Chromosome, IndividualTrait};
use crate::ga::GAMetadata;
use push_trait::{Nothing, Push};
use rand::prelude::SliceRandom;
use rand::{rngs::ThreadRng, Rng};

use super::CrossoverOperator;

/// # Single point crossover operator
///
/// This struct implements [CrossoverOperator] trait and can be used with GA.
///
/// It works by defininig single cutpoint splitting both parent chromosomes in two parts.
/// First child gets `parent_1`'s first part and `parent_2`'s second part.
/// Second child gets `parent_2`'s first part and `parent_1`'s second part.
///
/// Degenerated case when cutpoint is selected at index 0 or last can occur.
pub struct SinglePoint<R: Rng = ThreadRng> {
    rng: R,
}

impl SinglePoint<ThreadRng> {
    /// Creates new [SinglePoint] crossover operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> SinglePoint<R> {
    /// Creates new [SinglePoint] crossover operator with custom RNG
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for SinglePoint<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy,
    R: Rng,
{
    /// Returns a tuple of children
    ///
    /// It works by randomly selecting single cutpoint splitting both parent chromosomes in two parts.
    /// First child gets `parent_1`'s first part and `parent_2`'s second part.
    /// Second child gets `parent_2`'s first part and `parent_1`'s second part.
    ///
    /// Degenerated case when cutpoint is selected at index 0 or last can occur.
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
        let chromosome_len = parent_1.chromosome().len();
        let cut_point = self.rng.gen_range(0..chromosome_len);

        let mut child_1_ch = IndividualT::ChromosomeT::default();
        let mut child_2_ch = IndividualT::ChromosomeT::default();

        for locus in 0..cut_point {
            child_1_ch.push(parent_1.chromosome()[locus]);
            child_2_ch.push(parent_2.chromosome()[locus]);
        }

        for locus in cut_point..chromosome_len {
            child_1_ch.push(parent_2.chromosome()[locus]);
            child_2_ch.push(parent_1.chromosome()[locus]);
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
