#[cfg(feature = "ga_impl_mutation")]
pub mod impls;
#[cfg(feature = "ga_impl_mutation")]
pub use impls::*;

use crate::ga::{individual::IndividualTrait, GAMetadata};

/// # Mutation Operator
///
/// This trait defines common behaviour for mutation operators.
/// You can implement this trait to provide your custom crossover operator to the GA.
pub trait MutationOperator<IndividualT: IndividualTrait> {
    /// Mutates provided solution in place
    ///
    /// ## Arguments
    ///
    /// * `individual` - mutable reference to to-be-mutated individual
    /// * `mutation_rate` - probability of gene mutation
    fn apply(&mut self, metadata: &GAMetadata, individual: &mut IndividualT, mutation_rate: f64);
}
