//! Replacement operators module
//!
//! Purpose of the replacement operator is to merge two populations:
//! original one and the result of crossover phase to a single one,
//! which will be the next generation

#[cfg(feature = "ga_impl_mutation")]
pub mod impls;
#[cfg(feature = "ga_impl_mutation")]
pub use impls::*;

use crate::ga::{individual::IndividualTrait, GAMetadata};

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
pub trait ReplacementOperator<IndividualT: IndividualTrait> {
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
    fn apply(
        &mut self,
        metadata: &GAMetadata,
        population: Vec<IndividualT>,
        children: Vec<IndividualT>,
    ) -> Vec<IndividualT>;

    /// Returns `true` when the operator requires children to possess valid fitness values.
    ///
    /// Default implementation returns `true`
    fn requires_children_fitness(&self) -> bool {
        true
    }
}

