use std::ops::IndexMut;

use push_trait::{Nothing, Push};
use rand::Rng;
use rayon::iter::Rev;

use crate::ga::{individual::Chromosome, Individual};

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
  fn apply(&self, individual: &mut Individual<T>, mutation_rate: f64);
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
  fn apply(&self, _individual: &mut Individual<T>, _mutation_rate: f64) {}
}

/// ### Flilp bit mutation operator
///
/// This struct implements [MutationOperator] trait and can be used with GA
///
/// Genes are muatated by flipping the value - `1` becomes `0` and vice versa
pub struct FlipBit;

impl FlipBit {
  /// Returns new instance of [FlipBit] mutation operator
  pub fn new() -> Self {
    Self
  }
}

impl<T> MutationOperator<T> for FlipBit
where
  T: Chromosome + IndexMut<usize, Output = bool> + Push<bool, PushedOut = Nothing>,
{
  /// Mutates provided solution in place
  ///
  /// Genes are muatated by flipping the value - `1` becomes `0` and vice versa
  ///
  /// ## Arguments
  ///
  /// * `individual` - mutable reference to to-be-mutated individual
  /// * `mutation_rate` - probability of gene mutation
  fn apply(&self, individual: &mut Individual<T>, mutation_rate: f64) {
    let distribution = rand::distributions::Uniform::from(0.0..1.0);
    let chromosome_ref = individual.chromosome_ref_mut();
    let chromosome_len = chromosome_ref.len();

    for i in 0..chromosome_len {
      if rand::thread_rng().sample(distribution) < mutation_rate {
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
pub struct Interchange;

impl Interchange {
  /// Returns new instance of [Interchange] mutation operator
  pub fn new() -> Self {
    Self
  }
}

impl<T, G> MutationOperator<T> for Interchange
where
  G: Copy,
  T: Chromosome + IndexMut<usize, Output = G> + Push<G, PushedOut = Nothing>,
{
  /// Mutates provided solution in place
  ///
  /// If a gene is to be muatated, a new locus is randomly choosen and gene values are interchanged
  ///
  /// ## Arguments
  ///
  /// * `individual` - mutable reference to to-be-mutated individual
  /// * `mutation_rate` - probability of gene mutation
  fn apply(&self, individual: &mut Individual<T>, mutation_rate: f64) {
    let chromosome_ref = individual.chromosome_ref_mut();
    let chromosome_len = chromosome_ref.len();

    let dist = rand::distributions::Uniform::from(0.0..1.0);
    let index_dist = rand::distributions::Uniform::from(0..chromosome_len);

    for i in 0..chromosome_len {
      if rand::thread_rng().sample(dist) < mutation_rate {
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
pub struct Reversing;
impl<T, G> MutationOperator<T> for Reversing
where
	G: Copy,
  T: Chromosome + IndexMut<usize, Output = G> + Push<G, PushedOut = Nothing>,
{
  /// Mutates provided solution in place
  ///
	/// Random locus is selected and genes next to the selection position are reversed
  ///
  /// ## Arguments
  ///
  /// * `individual` - mutable reference to to-be-mutated individual
  /// * `mutation_rate` - probability of gene mutation
  fn apply(&self, individual: &mut Individual<T>, mutation_rate: f64) {
		let dist = rand::distributions::Uniform::from(0.0..1.0);
		let chromosome_ref = individual.chromosome_ref_mut();
		let chromosome_len = chromosome_ref.len();

		for i in 1..chromosome_len {
			if rand::thread_rng().sample(dist) < mutation_rate {
				let gene = chromosome_ref[i];
				chromosome_ref[i] = chromosome_ref[i - 1];
				chromosome_ref[i - 1] = gene;
			}
		}
	}
}

#[cfg(test)]
mod tests {
  use crate::ga::Individual;
  use itertools::Itertools;
  use rand::{distributions::Uniform, Rng};

  use super::{FlipBit, Identity, Interchange, MutationOperator};

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

    let identity_mutation = Identity;

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

    let mut individual = Individual {
      chromosome,
      fitness: f64::default(),
    };

    let operator = FlipBit::new();

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

    let mut individual = Individual {
      chromosome,
      fitness: f64::default(),
    };

    let operator = FlipBit::new();

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

    let mut individual = Individual {
      chromosome,
      fitness: f64::default(),
    };

    let operator = Interchange::new();

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

    let mut individual = Individual {
      chromosome,
      fitness: f64::default(),
    };

    let operator = Interchange::new();

    operator.apply(&mut individual, 0.);

    for (actual, expected) in std::iter::zip(chromosome_clone, individual.chromosome_ref()) {
      assert_eq!(actual, *expected);
    }
  }
}
