use itertools::{enumerate, Itertools};
use len_trait::Len;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::{Index, IndexMut};

use crate::ga::individual::{Chromosome, IndividualTrait};
use crate::ga::GAMetadata;
use push_trait::{Nothing, Push};
use rand::prelude::SliceRandom;
use rand::{rngs::ThreadRng, Rng};

use super::CrossoverOperator;

/// # Two point crossover operator
///
/// This struct implements [CrossoverOperator] and can be used with GA.
///
/// It works by randomly selecting two cutpoints splitting parents chromosomes in three parts.
/// Then it creates children by taking parents chromosome parts interchangeably.
/// Its mechanism is analoguous to [SinglePoint].
///
/// Degenerate case when both cutpoints are in the same place or at position 0 or last can occur.
pub struct TwoPoint<R: Rng = ThreadRng> {
    rng: R,
}

impl TwoPoint<ThreadRng> {
    /// Creates new [TwoPoint] crossover operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> TwoPoint<R> {
    /// Creates new [TwoPoint] crossover operator with custom RNG
    pub fn with_rng(rng: R) -> Self {
        TwoPoint { rng }
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for TwoPoint<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy,
    R: Rng,
{
    /// Returns a tuple of children
    ///
    /// It works by randomly selecting two cutpoints splitting parents chromosomes in three parts.
    /// Then it creates children by taking parents chromosome parts interchangeably.
    /// Its mechanism is analoguous to [SinglePoint].
    ///
    /// Degenerate case when both cutpoints are in the same place or at position 0 or last can occur.
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

        let chromosome_len = parent_1.chromosome().len();

        let cut_points = (
            self.rng.gen_range(0..chromosome_len),
            self.rng.gen_range(0..chromosome_len),
        );

        let (cut_point_1, cut_point_2) = if cut_points.0 <= cut_points.1 {
            (cut_points.0, cut_points.1)
        } else {
            (cut_points.1, cut_points.0)
        };

        let mut child_1_ch = IndividualT::ChromosomeT::default();
        let mut child_2_ch = IndividualT::ChromosomeT::default();

        for locus in 0..cut_point_1 {
            child_1_ch.push(parent_1.chromosome()[locus]);
            child_2_ch.push(parent_2.chromosome()[locus]);
        }

        for locus in cut_point_1..cut_point_2 {
            child_1_ch.push(parent_2.chromosome()[locus]);
            child_2_ch.push(parent_1.chromosome()[locus]);
        }

        for locus in cut_point_2..chromosome_len {
            child_1_ch.push(parent_1.chromosome()[locus]);
            child_2_ch.push(parent_2.chromosome()[locus]);
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
