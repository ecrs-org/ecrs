use std::ops::IndexMut;

use push_trait::{Nothing, Push};
use rand::{rngs::ThreadRng, Rng};

use crate::ga::{individual::Chromosome, ConcreteIndividual};

/// # Mutation Operator
///
/// This trait defines common behaviour for mutation operators.
/// You can implement this trait to provide your custom crossover operator to the GA.
pub trait MutationOperator<T: Chromosome> {
    /// Mutates provided solution in place
    ///
    /// ## Arguments
    ///
    /// * `individual` - mutable reference to to-be-mutated individual
    /// * `mutation_rate` - probability of gene mutation
    fn apply(&mut self, individual: &mut ConcreteIndividual<T>, mutation_rate: f64);
}

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

impl<T: Chromosome> MutationOperator<T> for Identity {
    fn apply(&mut self, _individual: &mut ConcreteIndividual<T>, _mutation_rate: f64) {}
}

/// ### Flilp bit mutation operator
///
/// This struct implements [MutationOperator] trait and can be used with GA
///
/// Genes are muatated by flipping the value - `1` becomes `0` and vice versa
pub struct FlipBit<R: Rng> {
    rng: R,
}

impl FlipBit<ThreadRng> {
    /// Returns new instance of [FlipBit] mutation operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> FlipBit<R> {
    /// Returns new instance of [FlipBit] mutation operator with custom RNG
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }
}

impl<T, R> MutationOperator<T> for FlipBit<R>
where
    T: Chromosome + IndexMut<usize, Output = bool> + Push<bool, PushedOut = Nothing>,
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
    fn apply(&mut self, individual: &mut ConcreteIndividual<T>, mutation_rate: f64) {
        let distribution = rand::distributions::Uniform::from(0.0..1.0);
        let chromosome_ref = individual.chromosome_ref_mut();
        let chromosome_len = chromosome_ref.len();

        for i in 0..chromosome_len {
            if self.rng.sample(distribution) < mutation_rate {
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
pub struct Interchange<R: Rng> {
    rng: R,
}

impl Interchange<ThreadRng> {
    /// Returns new instance of [Interchange] mutation operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> Interchange<R> {
    /// Returns new instance of [Interchange] mutation operator with custom RNG
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }
}

impl<T, G, R> MutationOperator<T> for Interchange<R>
where
    G: Copy,
    T: Chromosome + IndexMut<usize, Output = G> + Push<G, PushedOut = Nothing>,
    R: Rng,
{
    /// Mutates provided solution in place
    ///
    /// If a gene is to be muatated, a new locus is randomly choosen and gene values are interchanged
    ///
    /// ## Arguments
    ///
    /// * `individual` - mutable reference to to-be-mutated individual
    /// * `mutation_rate` - probability of gene mutation
    fn apply(&mut self, individual: &mut ConcreteIndividual<T>, mutation_rate: f64) {
        let chromosome_ref = individual.chromosome_ref_mut();
        let chromosome_len = chromosome_ref.len();

        let dist = rand::distributions::Uniform::from(0.0..1.0);
        let index_dist = rand::distributions::Uniform::from(0..chromosome_len);

        for i in 0..chromosome_len {
            if self.rng.sample(dist) < mutation_rate {
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
pub struct Reversing<R: Rng> {
    rng: R,
}

impl Reversing<ThreadRng> {
    /// Returns new instance of [Reversing] mutation operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> Reversing<R> {
    /// Returns new instance of [Reversing] mutation operator with custom RNG
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }
}

impl<T, G, R> MutationOperator<T> for Reversing<R>
where
    G: Copy,
    T: Chromosome + IndexMut<usize, Output = G> + Push<G, PushedOut = Nothing>,
    R: Rng,
{
    /// Mutates provided solution in place
    ///
    /// Random locus is selected and genes next to the selection position are reversed
    ///
    /// ## Arguments
    ///
    /// * `individual` - mutable reference to to-be-mutated individual
    /// * `mutation_rate` - probability of gene mutation
    fn apply(&mut self, individual: &mut ConcreteIndividual<T>, mutation_rate: f64) {
        let dist = rand::distributions::Uniform::from(0.0..1.0);
        let chromosome_ref = individual.chromosome_ref_mut();
        let chromosome_len = chromosome_ref.len();

        for i in 1..chromosome_len {
            if self.rng.sample(dist) < mutation_rate {
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
pub struct Inversion<R: Rng> {
    rng: R,
}

impl Inversion<ThreadRng> {
    /// Returns new instance of [Inversion] mutation operator with default RNG
    pub fn new() -> Self {
        Self::with_rng(rand::thread_rng())
    }
}

impl<R: Rng> Inversion<R> {
    /// Returns new instance of [Inversion] mutation operator with custom RNG
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }
}

impl<R: Rng> MutationOperator<Vec<usize>> for Inversion<R> {
    /// Mutates provided solution in place
    ///
    /// Two random locations are chosen marking out a segment of chromosome.
    /// Genes from this segment are then rotated around the segment's middle point.
    ///
    /// ## Arguments
    ///
    /// * `individual` - mutable reference to to-be-mutated individual
    /// * `mutation_rate` - probability of gene mutation
    fn apply(&mut self, individual: &mut ConcreteIndividual<Vec<usize>>, mutation_rate: f64) {
        let r: f64 = self.rng.gen();

        if r > mutation_rate {
            return;
        }

        let chromosome_len = individual.chromosome.len();
        let mut from: usize = self.rng.gen_range(0..chromosome_len);
        let mut to: usize = self.rng.gen_range(from..chromosome_len);

        while from < to {
            individual.chromosome.swap(from, to);
            from += 1;
            to -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ga::ConcreteIndividual;
    use itertools::Itertools;
    use rand::{distributions::Uniform, Rng};

    use super::{FlipBit, Identity, Interchange, MutationOperator, Reversing};

    #[test]
    fn identity_does_not_change_chromosome() {
        let chromosome = rand::thread_rng()
            .sample_iter(Uniform::from(-1.0..1.0))
            .take(30)
            .collect_vec();

        let mut individual = ConcreteIndividual {
            chromosome: chromosome.clone(),
            fitness: f64::default(),
        };

        let mut identity_mutation = Identity;

        identity_mutation.apply(&mut individual, 1.);

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

        let mut individual = ConcreteIndividual {
            chromosome,
            fitness: f64::default(),
        };

        let mut operator = FlipBit::new();

        operator.apply(&mut individual, 1.);

        for (actual, expected) in std::iter::zip(chromosome_clone, individual.chromosome_ref()) {
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

        let mut individual = ConcreteIndividual {
            chromosome,
            fitness: f64::default(),
        };

        let mut operator = FlipBit::new();

        operator.apply(&mut individual, 0.);

        for (actual, expected) in std::iter::zip(chromosome_clone, individual.chromosome_ref()) {
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

        let mut individual = ConcreteIndividual {
            chromosome,
            fitness: f64::default(),
        };

        let mut operator = Interchange::new();

        operator.apply(&mut individual, 1.);
        let changes = std::iter::zip(chromosome_clone, individual.chromosome_ref())
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

        let mut individual = ConcreteIndividual {
            chromosome,
            fitness: f64::default(),
        };

        let mut operator = Interchange::new();

        operator.apply(&mut individual, 0.);

        for (actual, expected) in std::iter::zip(chromosome_clone, individual.chromosome_ref()) {
            assert_eq!(actual, *expected);
        }
    }

    #[test]
    fn reversing_bubbles_first_gene_when_rate_1() {
        let chromosome = rand::thread_rng()
            .sample_iter(Uniform::from(-1.0..1.0))
            .take(40)
            .collect_vec();

        let mut individual = ConcreteIndividual {
            chromosome,
            fitness: f64::default(),
        };

        let first_gene_value = individual.chromosome_ref()[0];

        let mut operator = Reversing::new();

        operator.apply(&mut individual, 1.0);

        assert_eq!(
            first_gene_value,
            individual.chromosome_ref()[individual.chromosome_ref().len() - 1]
        );
    }
}
