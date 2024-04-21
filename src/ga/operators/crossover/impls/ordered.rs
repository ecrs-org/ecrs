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

/// # Ordered crossover operator
///
/// This struct implements [CrossoverOperator] trait and can be used with GA.
///
/// It works by taking a substring from one parent, and filling the missing places with alleles from
/// second parent in the order they appear in.
///
/// P1 : 2 <b>4 1 3</b> 5 <br>
/// P2 : 5 2 1 4 3 <br>
/// Ch : 5 <b>4 1 3</b> 2
///
/// Degenerated case when substring has length equal to genome length can occur.
pub struct OrderedCrossover<R: Rng = ThreadRng> {
    rng: R,
}

impl OrderedCrossover<ThreadRng> {
    /// Creates new [OrderedCrossover] crossover operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> OrderedCrossover<R> {
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }

    /// Helper function for [OrderedCrossover::apply]
    /// ## Arguments
    ///
    /// * `p1` - First parent to take part in crossover
    /// * `p2` - Second parent to take part in crossover
    /// * `begin` - Start (inclusive) of substring to transplant
    /// * `end` - End (exclusive) of substring to transplant
    fn create_child<GeneT, IndividualT>(
        &self,
        p1: &IndividualT,
        p2: &IndividualT,
        begin: usize,
        end: usize,
    ) -> IndividualT
    where
        IndividualT: IndividualTrait,
        IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
        GeneT: Copy + Eq + Hash,
    {
        let chromosome_len = p1.chromosome().len();

        let mut substring_set: HashSet<GeneT> = HashSet::new();

        for i in begin..end {
            substring_set.push(p1.chromosome()[i]);
        }

        let mut child_ch = IndividualT::ChromosomeT::default();
        let mut index: usize = 0;

        while child_ch.len() < begin {
            let gene = p2.chromosome()[index];
            if !substring_set.contains(&gene) {
                child_ch.push(gene);
            }
            index += 1;
        }

        for i in begin..end {
            child_ch.push(p1.chromosome()[i]);
        }

        while index < chromosome_len {
            let gene = p2.chromosome()[index];
            if !substring_set.contains(&gene) {
                child_ch.push(gene);
            }
            index += 1;
        }
        IndividualT::from(child_ch)
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for OrderedCrossover<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy + Eq + Hash,
    R: Rng,
{
    /// Returns a tuple of children, first child is created by taking a substring from parent_1,
    /// second child is created by using a substring from parent_2
    ///
    /// It works by taking a substring from one parent, and filling the missing places with alleles from
    /// second parent in the order they appear in.
    ///
    /// P1 : 2 <b>4 1 3</b> 5 <br>
    /// P2 : 5 2 1 4 3 <br>
    /// Ch : 5 <b>4 1 3</b> 2
    ///
    /// Degenerated case when substring has length equal to genome length can occur.
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in crossover
    /// * `parent_2` - Second parent to take part in crossover
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

        let begin: usize = self.rng.gen_range(0..chromosome_len);
        let end: usize = self.rng.gen_range(begin..=chromosome_len);

        let child_1 = self.create_child(parent_1, parent_2, begin, end);
        let child_2 = self.create_child(parent_2, parent_1, begin, end);

        (child_1, child_2)
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
