//! Replacement operators module
//!
//! Purpose of the replacement operator is to merge two populations:
//! original one and the result of crossover phase to a single one,
//! which will be the next generation

use crate::ga::{individual::Chromosome, Individual};

/// # Replacement Operator
///
/// This trait defines common behavirou for crossover operators.
/// You can implement this trait to provide your custom replacement
/// operator to the genetic algortihtm.
///
/// **NOTE**: In current implementation, all library-implemented operators assume that
/// at indices i, i+1 in `population` collection there are parents of children i, i+1
/// from `children` collection. Any violation of this invariant may lead to bugs - it can
/// be considered an undefined behaviour. We'll work toward improving this case in the future.
pub trait ReplacementOperator<T: Chromosome> {
  /// Merges `children` - output of crossover operator with current population.
  /// In current implementation (and design) the merge happens in place (where possible)
  /// to avoid necessity of copying whole population each time the operator is called.
  /// This might be a premature optimization and we will consider benchmarking this approach
  /// and possibly making API more intuitive by making this function return the merged
  /// population instead of modyfying in-place.
  ///
  /// **NOTE**: In current implementation, all library-implemented operators assume that
  /// at indices i, i+1 in `population` collection there are parents of children i, i+1
  /// from `children` collection. Any violation of this invariant may lead to bugs - it can
  /// be considered an undefined behaviour. We'll work toward improving this case in the future.
  ///
  /// ### Arguements
  ///
  /// * `population` - Original population, input to the crossover phase.
  /// This collection should be modified in place by the operator.
  /// * `children` - Result of the crossover phase
  fn apply(population: &mut [Individual<T>], children: &[Individual<T>]);
}

/// # BothParents replacement operator
///
/// This struct implements [ReplacementOperator] trait and can be used with genetic algorithm.
///
/// It works simply by replacing parents with their children. In effect, each individual
/// only gets to breed once.
///
/// This operator has the same effect as [Noop].
pub struct BothParents;

impl BothParents {
  pub fn new() -> Self {
    Self
  }
}

impl<T: Chromosome> ReplacementOperator<T> for BothParents {
  #[inline(always)]
  fn apply(_population: &mut [Individual<T>], _children: &[Individual<T>]) {}
}

/// # Noop replacement operator
///
/// This struct implements [ReplacementOperator] trait and can be used with genetic algorithm.
///
/// It does nothing. Implementation is a noop.
///
/// This operator is the same as [BothParents]. Can be treated as an alias.
pub struct Noop;

impl Noop {
  pub fn new() -> Self {
    Self
  }
}

impl<T: Chromosome> ReplacementOperator<T> for Noop {
  #[inline(always)]
  fn apply(_population: &mut [Individual<T>], _children: &[Individual<T>]) {}
}
