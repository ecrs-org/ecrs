use len_trait::Len;

use std::ops::Index;

use crate::ga::individual::IndividualTrait;
use crate::ga::Metrics;
use push_trait::{Nothing, Push};

use rand::{rngs::ThreadRng, Rng};

use super::CrossoverOperator;

/// # Uniform crossover operator
///
/// This struct implements [CrossoverOperator] and can be used with GA.
///
/// It works by creating a bit-mask of chromosome length. 1 means that gene should be taken from first
/// parent, 0 means that gene should be take from second parent. This is inverted when creating second child.
pub struct Uniform<R: Rng + Clone = ThreadRng> {
    rng: R,
}

impl Uniform<ThreadRng> {
    /// Creates new [Uniform] crossover operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng + Clone> Uniform<R> {
    /// Creates new [Uniform] crossover operator with custom RNG
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }
}

impl<R> Uniform<R>
where
    R: Rng + Clone,
{
    /// Returns a tuple of children
    ///
    /// It works by creating a bit-mask of chromosome length. 1 means that gene should be taken from first
    /// parent, 0 means that gene should be take from second parent. This is inverted when creating second child.
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in recombination
    /// * `parent_2` - Second parent to take part in recombination
    fn apply_single<GeneT, IndividualT>(
        &mut self,
        _metrics: &Metrics,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT)
    where
        IndividualT: IndividualTrait,
        IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
        GeneT: Copy,
    {
        assert_eq!(
            parent_1.chromosome().len(),
            parent_2.chromosome().len(),
            "Parent chromosome length must match"
        );

        let chromosome_len = parent_1.chromosome().len();

        let mut child_1_ch = IndividualT::ChromosomeT::default();
        let mut child_2_ch = IndividualT::ChromosomeT::default();

        let mask = self
            .rng
            .clone()
            .sample_iter(rand::distributions::Uniform::new(0.0, 1.0))
            .take(chromosome_len);

        for (locus, val) in mask.enumerate() {
            if val >= 0.5 {
                child_1_ch.push(parent_1.chromosome()[locus]);
                child_2_ch.push(parent_2.chromosome()[locus]);
            } else {
                child_1_ch.push(parent_2.chromosome()[locus]);
                child_2_ch.push(parent_1.chromosome()[locus]);
            }
        }

        (IndividualT::from(child_1_ch), IndividualT::from(child_2_ch))
    }
}

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for Uniform<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: Index<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    GeneT: Copy,
    R: Rng + Clone,
{
    /// Returns vector of owned individuals which were created in result of applying crossover
    /// operator.
    ///
    /// It works by creating a bit-mask of chromosome length. 1 means that gene should be taken from first
    /// parent, 0 means that gene should be take from second parent. This is inverted when creating second child.
    ///
    /// ## Arguments
    ///
    /// * `metrics` - algorithm state metrics, see the structure details for more info,
    /// * `selected` - references to individuals selected during selection step.
    fn apply(&mut self, metrics: &Metrics, selected: &[&IndividualT]) -> Vec<IndividualT> {
        assert!(selected.len() & 1 == 0);

        let mut output = Vec::with_capacity(selected.len());

        for parents in selected.chunks(2) {
            let (child_1, child_2) = self.apply_single(metrics, parents[0], parents[1]);
            output.push(child_1);
            output.push(child_2);
        }

        output
    }
}
