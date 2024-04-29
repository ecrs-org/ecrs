use std::{marker::PhantomData, ops::IndexMut};

use len_trait::Len;
use push_trait::{Nothing, Push};
use rand::{rngs::ThreadRng, Rng};

use crate::ga::{individual::IndividualTrait, GAMetadata};

use super::MutationOperator;

/// # Identity Mutation Operator
///
/// This struct implements [MutationOperator] trait and can be used with GA.
///
/// Identity does not perform any changes to the chromosome. Use this if you
/// do not want to mutate your solutions.
pub struct Identity;

impl Identity {
    /// Returns new [Identity] mutation operator
    pub fn new() -> Self {
        Identity {}
    }
}

impl<IndividualT: IndividualTrait> MutationOperator<IndividualT> for Identity {
    fn apply(&mut self, _metadata: &GAMetadata, _individual: &mut IndividualT) {}
}

/// ### Flilp bit mutation operator
///
/// This struct implements [MutationOperator] trait and can be used with GA
///
/// Genes are muatated by flipping the value - `1` becomes `0` and vice versa
pub struct FlipBit<R: Rng = ThreadRng> {
    mutation_rate: f64,
    rng: R,
}

impl FlipBit<ThreadRng> {
    /// Returns new instance of [FlipBit] mutation operator with default RNG
    ///
    /// * `mutation_rate` - probability of gene mutation
    pub fn new(mutation_rate: f64) -> Self {
        Self::with_rng(mutation_rate, rand::thread_rng())
    }
}

impl<R: Rng> FlipBit<R> {
    /// Returns new instance of [FlipBit] mutation operator with custom RNG
    pub fn with_rng(mutation_rate: f64, rng: R) -> Self {
        Self { mutation_rate, rng }
    }
}

impl<IndividualT, R> MutationOperator<IndividualT> for FlipBit<R>
where
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: IndexMut<usize, Output = bool> + Push<bool, PushedOut = Nothing>,
    R: Rng,
{
    /// Mutates provided solution in place
    ///
    /// Genes are muatated by flipping the value - `1` becomes `0` and vice versa
    ///
    /// ## Arguments
    ///
    /// * `individual` - mutable reference to to-be-mutated individual
    /// * `mutation_rate` - probability of gene mutation
    fn apply(&mut self, _metadata: &GAMetadata, individual: &mut IndividualT) {
        let distribution = rand::distributions::Uniform::from(0.0..1.0);
        let chromosome_ref = individual.chromosome_mut();
        let chromosome_len = chromosome_ref.len();

        for i in 0..chromosome_len {
            if self.rng.sample(distribution) < self.mutation_rate {
                chromosome_ref[i] = !chromosome_ref[i];
            }
        }
    }
}

/// ### Interchange mustation operator
///
/// This struct implements [MutationOperator] trait and can be used with GA
///
/// If a gene is to be muatated, a new locus is randomly choosen and gene values are interchanged
pub struct Interchange<R: Rng = ThreadRng> {
    mutation_rate: f64,
    rng: R,
}

impl Interchange<ThreadRng> {
    /// Returns new instance of [Interchange] mutation operator with default RNG
    ///
    /// * `mutation_rate` - probability of gene mutation
    pub fn new(mutation_rate: f64) -> Self {
        Self::with_rng(mutation_rate, rand::thread_rng())
    }
}

impl<R: Rng> Interchange<R> {
    /// Returns new instance of [Interchange] mutation operator with custom RNG
    pub fn with_rng(mutation_rate: f64, rng: R) -> Self {
        Self { mutation_rate, rng }
    }
}

impl<IndividualT, G, R> MutationOperator<IndividualT> for Interchange<R>
where
    G: Copy,
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: IndexMut<usize, Output = G> + Push<G, PushedOut = Nothing>,
    R: Rng,
{
    /// Mutates provided solution in place
    ///
    /// If a gene is to be muatated, a new locus is randomly choosen and gene values are interchanged
    ///
    /// ## Arguments
    ///
    /// * `individual` - mutable reference to to-be-mutated individual
    fn apply(&mut self, _metadata: &GAMetadata, individual: &mut IndividualT) {
        let chromosome_ref = individual.chromosome_mut();
        let chromosome_len = chromosome_ref.len();

        let dist = rand::distributions::Uniform::from(0.0..1.0);
        let index_dist = rand::distributions::Uniform::from(0..chromosome_len);

        for i in 0..chromosome_len {
            if self.rng.sample(dist) < self.mutation_rate {
                let rand_index = rand::thread_rng().sample(index_dist);
                let gene = chromosome_ref[rand_index];
                chromosome_ref[rand_index] = chromosome_ref[i];
                chromosome_ref[i] = gene;
            }
        }
    }
}

/// ### Reversing mutation operator
///
/// This struct implements [MutationOperator] trait and can be used with GA
///
/// Random locus is selected and genes next to the selection position are reversed
pub struct Reversing<R: Rng = ThreadRng> {
    mutation_rate: f64,
    rng: R,
}

impl Reversing<ThreadRng> {
    /// Returns new instance of [Reversing] mutation operator with default RNG
    ///
    /// * `mutation_rate` - probability of gene mutation
    pub fn new(mutation_rate: f64) -> Self {
        Self::with_rng(mutation_rate, rand::thread_rng())
    }
}

impl<R: Rng> Reversing<R> {
    /// Returns new instance of [Reversing] mutation operator with custom RNG
    pub fn with_rng(mutation_rate: f64, rng: R) -> Self {
        Self { mutation_rate, rng }
    }
}

impl<IndividualT, GeneT, R> MutationOperator<IndividualT> for Reversing<R>
where
    GeneT: Copy,
    IndividualT: IndividualTrait,
    IndividualT::ChromosomeT: IndexMut<usize, Output = GeneT> + Push<GeneT, PushedOut = Nothing>,
    R: Rng,
{
    /// Mutates provided solution in place
    ///
    /// Random locus is selected and genes next to the selection position are reversed
    ///
    /// ## Arguments
    ///
    /// * `individual` - mutable reference to to-be-mutated individual
    fn apply(&mut self, _metadata: &GAMetadata, individual: &mut IndividualT) {
        let dist = rand::distributions::Uniform::from(0.0..1.0);
        let chromosome_ref = individual.chromosome_mut();
        let chromosome_len = chromosome_ref.len();

        for i in 1..chromosome_len {
            if self.rng.sample(dist) < self.mutation_rate {
                let gene = chromosome_ref[i];
                chromosome_ref[i] = chromosome_ref[i - 1];
                chromosome_ref[i - 1] = gene;
            }
        }
    }
}

/// ### [Inversion] mutation operator
///
/// This struct implements [MutationOperator] trait and can be used with GA
///
/// Two random locations are chosen marking out a segment of chromosome.
/// Genes from this segment are then rotated around the segment's middle point.
pub struct Inversion<GeneT: Copy, R: Rng = ThreadRng> {
    mutation_rate: f64,
    rng: R,
    _marker: PhantomData<GeneT>,
}

impl<GeneT: Copy> Inversion<GeneT, ThreadRng> {
    /// Returns new instance of [Inversion] mutation operator with default RNG
    ///
    /// * `mutation_rate` - probability of gene mutation
    pub fn new(mutation_rate: f64) -> Self {
        Self::with_rng(mutation_rate, rand::thread_rng())
    }
}

impl<R: Rng, GeneT: Copy> Inversion<GeneT, R> {
    /// Returns new instance of [Inversion] mutation operator with custom RNG
    pub fn with_rng(mutation_rate: f64, rng: R) -> Self {
        Self {
            mutation_rate,
            rng,
            _marker: PhantomData,
        }
    }
}

impl<IndividualT: IndividualTrait, GeneT: Copy, R: Rng> MutationOperator<IndividualT> for Inversion<GeneT, R>
where
    IndividualT::ChromosomeT: Len + AsMut<[GeneT]>,
{
    /// Mutates provided solution in place
    ///
    /// Two random locations are chosen marking out a segment of chromosome.
    /// Genes from this segment are then rotated around the segment's middle point.
    ///
    /// ## Arguments
    ///
    /// * `individual` - mutable reference to to-be-mutated individual
    fn apply(&mut self, _metadata: &GAMetadata, individual: &mut IndividualT) {
        let _marker: PhantomData<GeneT> = PhantomData;

        let r: f64 = self.rng.gen();

        if r > self.mutation_rate {
            return;
        }

        let chromosome_len = individual.chromosome().len();
        let mut from: usize = self.rng.gen_range(0..chromosome_len);
        let mut to: usize = self.rng.gen_range(from..chromosome_len);

        while from < to {
            individual.chromosome_mut().as_mut().swap(from, to);
            from += 1;
            to -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ga::{individual::IndividualTrait, GAMetadata, Individual};
    use itertools::Itertools;
    use rand::{distributions::Uniform, Rng};

    use super::{FlipBit, Identity, Interchange, MutationOperator, Reversing};

    #[test]
    fn identity_does_not_change_chromosome() {
        let chromosome = rand::thread_rng()
            .sample_iter(Uniform::from(-1.0..1.0))
            .take(30)
            .collect_vec();

        let mut individual = Individual {
            chromosome: chromosome.clone(),
            fitness: f64::default(),
        };

        let mut identity_mutation = Identity;

        identity_mutation.apply(&GAMetadata::default(), &mut individual);

        assert_eq!(chromosome, individual.chromosome);
    }

    #[test]
    fn flipbit_negates_chromosome() {
        let chromosome = rand::thread_rng()
            .sample_iter(Uniform::from(-1.0..1.0))
            .take(30)
            .map(|val| val > 0.)
            .collect_vec();

        let chromosome_clone = chromosome.clone();

        let mut individual = Individual {
            chromosome,
            fitness: f64::default(),
        };

        let mut operator = FlipBit::new(1.);

        operator.apply(&GAMetadata::default(), &mut individual);

        for (actual, expected) in std::iter::zip(chromosome_clone, individual.chromosome()) {
            assert_eq!(actual, !*expected);
        }
    }

    #[test]
    fn flipbit_does_not_mutate_rate_0() {
        let chromosome = rand::thread_rng()
            .sample_iter(Uniform::from(-1.0..1.0))
            .take(30)
            .map(|val| val > 0.)
            .collect_vec();

        let chromosome_clone = chromosome.clone();

        let mut individual = Individual {
            chromosome,
            fitness: f64::default(),
        };

        let mut operator = FlipBit::new(0.);

        operator.apply(&GAMetadata::default(), &mut individual);

        for (actual, expected) in std::iter::zip(chromosome_clone, individual.chromosome()) {
            assert_eq!(actual, *expected);
        }
    }

    #[test]
    fn interchange_introduces_changes() {
        let chromosome = rand::thread_rng()
            .sample_iter(Uniform::from(-1.0..1.0))
            .take(300)
            .map(|val| val > 0.)
            .collect_vec();

        let chromosome_clone = chromosome.clone();

        let mut individual = Individual {
            chromosome,
            fitness: f64::default(),
        };

        let mut operator = Interchange::new(1.);

        operator.apply(&GAMetadata::default(), &mut individual);
        let changes = std::iter::zip(chromosome_clone, individual.chromosome())
            .filter(|p| p.0 != *p.1)
            .count();
        assert!(changes > 0);
    }

    #[test]
    fn interchange_does_not_mutate_rate_0() {
        let chromosome = rand::thread_rng()
            .sample_iter(Uniform::from(-1.0..1.0))
            .take(30)
            .map(|val| val > 0.)
            .collect_vec();

        let chromosome_clone = chromosome.clone();

        let mut individual = Individual {
            chromosome,
            fitness: f64::default(),
        };

        let mut operator = Interchange::new(0.);

        operator.apply(&GAMetadata::default(), &mut individual);

        for (actual, expected) in std::iter::zip(chromosome_clone, individual.chromosome()) {
            assert_eq!(actual, *expected);
        }
    }

    #[test]
    fn reversing_bubbles_first_gene_when_rate_1() {
        let chromosome = rand::thread_rng()
            .sample_iter(Uniform::from(-1.0..1.0))
            .take(40)
            .collect_vec();

        let mut individual = Individual {
            chromosome,
            fitness: f64::default(),
        };

        let first_gene_value = individual.chromosome()[0];

        let mut operator = Reversing::new(1.0);

        operator.apply(&GAMetadata::default(), &mut individual);

        assert_eq!(
            first_gene_value,
            individual.chromosome()[individual.chromosome().len() - 1]
        );
    }
}
