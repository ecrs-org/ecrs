use len_trait::Len;

use std::ops::Index;

use crate::ga::individual::IndividualTrait;
use crate::ga::GAMetadata;
use push_trait::{Nothing, Push};

use rand::{rngs::ThreadRng, Rng};

use super::CrossoverOperator;

/// # Parameterized Uniform  crossover operator
///
/// This struct implements [CrossoverOperator] and can be used with GA.
///
/// It works by creating a bit-mask of chromosome length. 1 means that gene should be taken from first
/// parent, 0 means that gene should be take from second parent. This is inverted when creating second child.
///
/// Bias is a probability of drawing a 1 in the bit-mask.
pub struct UniformParameterized<R: Rng = ThreadRng> {
    rng: R,
    distr: rand::distributions::Uniform<f64>,
    bias: f64,
}

impl UniformParameterized<ThreadRng> {
    pub fn new(bias: f64) -> Self {
        Self::with_rng(rand::thread_rng(), bias)
    }
}

impl<R: Rng> UniformParameterized<R> {
    pub fn with_rng(rng: R, bias: f64) -> Self {
        Self {
            rng,
            distr: rand::distributions::Uniform::new(0.0, 1.0),
            bias,
        }
    }
}

impl<R> UniformParameterized<R>
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
        _metadata: &GAMetadata,
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

        let mask = self.rng.clone().sample_iter(self.distr).take(chromosome_len);

        for (locus, val) in mask.enumerate() {
            if val <= self.bias {
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

impl<GeneT, IndividualT, R> CrossoverOperator<IndividualT> for UniformParameterized<R>
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

