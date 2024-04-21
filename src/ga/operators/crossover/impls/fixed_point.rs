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

/// # Fixed point crossover operator
///
/// Works just like `SinglePoint`, however the cut point is fixed and chosen apriori instead of
/// being random.
pub struct FixedPoint {
    pub cut_point: usize,
}

impl FixedPoint {
    /// Returns new instance of the `FixedPoint` operator.
    ///
    /// # Arguments
    ///
    /// * `cut_point` - index of first gene that will be taken from second parent to first child
    pub fn new(cut_point: usize) -> Self {
        Self { cut_point }
    }
}

impl<GeneT, IndividualT> CrossoverOperator<IndividualT> for FixedPoint
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: IndexMut<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy,
{
    /// Returns a tuple of children
    ///
    /// It works by cutting parent chromosomes in single, fixed point and the acting like a single
    /// point crossover.
    ///
    /// # Arguments
    ///
    /// * `parent_1` - First parent to take part in recombination
    /// * `parent_2` - Second parent to take part in recombination
    fn apply_legacy(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT) {
        let mut child_1 = parent_1.clone();
        let mut child_2 = parent_2.clone();

        for i in self.cut_point..parent_1.chromosome().len() {
            child_1.chromosome_mut()[i] = parent_2.chromosome()[i];
            child_2.chromosome_mut()[i] = parent_1.chromosome()[i];
        }

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
