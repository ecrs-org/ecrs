use super::{individual::Chromosome, GAMetadata, Individual};

#[cfg(feature = "ops_crossover")]
pub mod crossover;
pub mod fitness;
#[cfg(feature = "ops_mutation")]
pub mod mutation;
#[cfg(feature = "ops_replacement")]
pub mod replacement;
#[cfg(feature = "ops_selection")]
pub mod selection;

/// # Crossover Operator
///
/// This trait defines common behaviour for crossover operators.
/// You can implement this trait to provide your custom crossover operator to the GA.
pub trait CrossoverOperator<T: Chromosome> {
  /// Returns a tuple of children
  ///
  /// ## Arguments
  ///
  /// * `parent_1` - First parent to take part in recombination
  /// * `parent_2` - Second parent to take part in recombination
  fn apply(&mut self, parent_1: &Individual<T>, parent_2: &Individual<T>) -> (Individual<T>, Individual<T>);
}

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
  fn apply(&mut self, individual: &mut Individual<T>, mutation_rate: f64);
}

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

/// ### Selection operator
///
/// This trait defines common behaviour for selection operators.
/// You can implement this trait to provide your custom crossover opertator to the GA.
///
/// Following operators are implemented in the library:
///
/// * [RouletteWheel]
/// * [Random]
/// * [Rank]
/// * [RankR]
/// * [Tournament]
/// * [StochasticUniversalSampling]
/// * [Boltzmann]
///
/// See their respecitve docs for details.
pub trait SelectionOperator<T: Chromosome> {
  /// Returns a vector of references to individuals selected to mating pool
  ///
  /// ### Arguments
  ///
  /// * `metadata` - [crate::ga::GAMetadata] information on current stage of the algorithm (iteration, elapsed time, etc.)
  /// * `population` - individuals to choose mating pool from
  /// * `count` - target number of individuals in mating pool
  fn apply<'a>(
    &mut self,
    metadata: &GAMetadata,
    population: &'a [Individual<T>],
    count: usize,
  ) -> Vec<&'a Individual<T>>;
}
