use itertools::{enumerate, Itertools};
use len_trait::Len;

use crate::ga::individual::IndividualTrait;
use crate::ga::GAMetadata;
use push_trait::{Nothing, Push};
use rand::prelude::SliceRandom;
use rand::{rngs::ThreadRng, Rng};
use std::ops::Index;

use super::CrossoverOperator;

/// # Shuffle crossover operator
///
/// This struct implements [CrossoverOperator] trait and can be used with GA.
///
/// It works by randomly shuffling both parents chromosome the same way then
/// selecting single cutpoint splitting both parents shuffled chromosomes in two parts.
/// First child gets `parent_1`'s first part and `parent_2`'s second part.
/// Second child gets `parent_2`'s first part and `parent_1`'s second part.
/// Lastly childs chromosomes are de-shuffled.
///
/// Degenerated case when cutpoint is selected at index 0 or last can occur.
pub struct Shuffle<R: Rng = ThreadRng> {
    rng: R,
}

impl Shuffle<ThreadRng> {
    /// Creates new [Shuffle] crossover operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> Shuffle<R> {
    /// Creates new [Shuffle] crossover operator with custom RNG
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }
}

impl<R: Rng> Shuffle<R> {
    /// Returns a tuple of children
    ///
    /// It works by randomly shuffling both parents chromosome the same way then
    /// selecting single cutpoint splitting both parents shuffled chromosomes in two parts.
    /// First child gets `parent_1`'s first part and `parent_2`'s second part.
    /// Second child gets `parent_2`'s first part and `parent_1`'s second part.
    /// Lastly childs chromosomes are de-shuffled.
    ///
    /// Degenerated case when cutpoint is selected at index 0 or last can occur.
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in recombination
    /// * `parent_2` - Second parent to take part in recombination
    fn apply_single<GeneT, IndividualT>(
        &mut self,
        _metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT)
    where
        IndividualT: IndividualTrait,
        IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
        GeneT: Copy,
    {
        let chromosome_len = parent_1.chromosome().len();
        let cut_point = self.rng.gen_range(0..chromosome_len);

        let mut shuffled = Vec::from_iter(0..chromosome_len);
        shuffled.shuffle(&mut self.rng);
        let mask = shuffled.iter().map(|x| x < &cut_point).collect_vec();

        let mut child_1_ch = IndividualT::ChromosomeT::default();
        let mut child_2_ch = IndividualT::ChromosomeT::default();

        for (i, fist_parent) in enumerate(mask) {
            if fist_parent {
                child_1_ch.push(parent_1.chromosome()[i]);
                child_2_ch.push(parent_2.chromosome()[i]);
            } else {
                child_1_ch.push(parent_2.chromosome()[i]);
                child_2_ch.push(parent_1.chromosome()[i]);
            }
        }

        (IndividualT::from(child_1_ch), IndividualT::from(child_2_ch))
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for Shuffle<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy,
    R: Rng,
{
    /// Returns vector of owned individuals which were created in result of applying crossover
    /// operator.
    ///
    /// It works by randomly shuffling both parents chromosome the same way then
    /// selecting single cutpoint splitting both parents shuffled chromosomes in two parts.
    /// First child gets `parent_1`'s first part and `parent_2`'s second part.
    /// Second child gets `parent_2`'s first part and `parent_1`'s second part.
    /// Lastly childs chromosomes are de-shuffled.
    ///
    /// Degenerated case when cutpoint is selected at index 0 or last can occur.
    ///
    /// ## Arguments
    ///
    /// * `metadata` - algorithm state metadata, see the structure details for more info,
    /// * `selected` - references to individuals selected during selection step.
    fn apply(&mut self, metadata: &GAMetadata, selected: &[&IndividualT]) -> Vec<IndividualT> {
        assert!(selected.len() & 1 == 0);

        let mut output = Vec::with_capacity(selected.len());

        for parents in selected.chunks(2) {
            let (child_1, child_2) = self.apply_single(metadata, parents[0], parents[1]);
            output.push(child_1);
            output.push(child_2);
        }

        output
    }
}
