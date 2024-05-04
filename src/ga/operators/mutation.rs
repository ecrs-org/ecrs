#[cfg(feature = "ga_impl_mutation")]
pub mod impls;
#[cfg(feature = "ga_impl_mutation")]
pub use impls::*;

use crate::ga::{individual::IndividualTrait, Metrics};

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
    fn apply(&mut self, metrics: &Metrics, individual: &mut IndividualT);
}
