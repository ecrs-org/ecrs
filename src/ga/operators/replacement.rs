//! Replacement operators module
//!
//! Purpose of the replacement operator is to merge two populations:
//! original one and the result of crossover phase to a single one,
//! which will be the next generation

use crate::ga::{individual::Chromosome, Individual};

/// # Replacement Operator
///
/// This trait defines common behaviour for crossover operators.
/// You can implement this trait to provide your custom replacement
/// operator to the genetic algorithm.
///
/// **NOTE**: In current implementation, all library-implemented operators assume that
/// at indices i, i+1 in `population` collection there are parents of children i, i+1
/// from `children` collection. Any violation of this invariant may lead to bugs - it can
/// be considered an undefined behaviour. We'll work towards improving this case in the future.
pub trait ReplacementOperator<T: Chromosome> {
  /// Merges `children` - output of crossover operator with current population.
  ///
  /// **NOTE**: In current implementation, all library-implemented operators assume that
  /// at indices i, i+1 in `population` collection there are parents of children i, i+1
  /// from `children` collection. Any violation of this invariant may lead to bugs - it can
  /// be considered an undefined behaviour. We'll work towards improving this case in the future.
  ///
  /// ### Arguments
  ///
  /// * `population` - Original population, input to the crossover phase.
  /// This collection should be modified in place by the operator.
  /// * `children` - Result of the crossover phase.
  fn apply(&self, population: Vec<Individual<T>>, children: Vec<Individual<T>>) -> Vec<Individual<T>>;

  /// Returns `true` when the operator requires children to possess valid fitness values.
  ///
  /// Default implementation returns `false`
  fn requires_children_fitness(&self) -> bool {
    true
  }
}

/// # BothParents replacement operator
///
/// This struct implements [ReplacementOperator] trait and can be used with genetic algorithm.
///
/// It works simply by replacing parents with their children. In effect, each individual
/// only gets to breed once.
/// 
/// **NOTE**: In current implementation, all library-implemented operators assume that
/// at indices i, i+1 in `population` collection there are parents of children i, i+1
/// from `children` collection. Any violation of this invariant may lead to bugs - it can
/// be considered an undefined behaviour. We'll work towards improving this case in the future.
pub struct BothParents;

impl BothParents {
  /// Returns new instance of [BothParents] replacement operator.
  pub fn new() -> Self {
    Self
  }
}

impl<T: Chromosome> ReplacementOperator<T> for BothParents {
  /// Works simply by replacing parents with their children
  ///
  /// **NOTE**: In current implementation, all library-implemented operators assume that
  /// at indices i, i+1 in `population` collection there are parents of children i, i+1
  /// from `children` collection. Any violation of this invariant may lead to bugs - it can
  /// be considered an undefined behaviour. We'll work towards improving this case in the future.
  ///
  /// ### Arguments
  ///
  /// * `population` - Original population, input to the crossover phase.
  /// This collection should be modified in place by the operator.
  /// * `children` - Result of the crossover phase
  #[inline(always)]
  fn apply(&self, _population: Vec<Individual<T>>, children: Vec<Individual<T>>) -> Vec<Individual<T>> {
    children
  }

  /// Returns `true` when the operator requires children to possess valid fitness values.
  ///
  /// This implementation returns `false`.
  #[inline(always)]
  fn requires_children_fitness(&self) -> bool {
    false
  }
}

/// # Noop replacement operator
///
/// This struct implements [ReplacementOperator] trait and can be used with genetic algorithm.
///
/// It does nothing. Implementation is a noop.
pub struct Noop;

impl Noop {
  /// Returns new instance of [Noop] replacement operator
  pub fn new() -> Self {
    Self
  }
}

impl<T: Chromosome> ReplacementOperator<T> for Noop {
  /// Returns input `population`.
  #[inline(always)]
  fn apply(&self, population: Vec<Individual<T>>, _children: Vec<Individual<T>>) -> Vec<Individual<T>> {
    population
  }

  /// Returns `true` when the operator requires children to possess valid fitness values.
  ///
  /// This implementation returns `false`.
  #[inline(always)]
  fn requires_children_fitness(&self) -> bool {
    false
  }
}
