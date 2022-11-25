use std::ops::IndexMut;

use push_trait::{Nothing, Push};
use rand::Rng;

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
  fn apply(&self, indivudial: &mut Individual<T>, mutation_rate: f64);
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
  fn apply(&self, _indivudial: &mut Individual<T>, _mutation_rate: f64) {}
}

#[cfg(test)]
mod tests {
  use crate::ga::Individual;
  use itertools::Itertools;
  use rand::{distributions::Uniform, Rng};

  use super::{Identity, MutationOperator};

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

    identity_mutation.apply(&mut individual, 0.1);

    assert_eq!(chromosome, individual.chromosome);
  }
}

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
  fn apply(&self, indivudial: &mut Individual<T>, mutation_rate: f64) {
    let distribution = rand::distributions::Uniform::from(0.0..1.0);
    let chromosome_ref = indivudial.chromosome_ref_mut();
    let chromosome_len = chromosome_ref.len();

    for i in 0..chromosome_len {
      if rand::thread_rng().sample(distribution) < mutation_rate {
        chromosome_ref[i] = !chromosome_ref[i];
      }
    }
  }
}
