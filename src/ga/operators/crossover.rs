#[cfg(feature = "ga_impl_crossover")]
pub mod impls;
#[cfg(feature = "ga_impl_crossover")]
pub use impls::*;

use crate::ga::individual::IndividualTrait;
use crate::ga::Metrics;

/// # Crossover Operator
///
/// This trait defines common behaviour for crossover operators.
/// You can implement this trait to provide your custom crossover operator to the GA.
pub trait CrossoverOperator<IndividualT: IndividualTrait> {
    /// FIXME: Understand lifetimes here!
    // fn apply_iter<'i, InputIter, OutputIter>(
    //     &mut self,
    //     metadata: &GAMetadata,
    //     selected: InputIter,
    // ) -> OutputIter
    // where
    //     InputIter: Iterator<Item = &'i IndividualT>,
    //     OutputIter: Iterator<Item = IndividualT>,
    //     IndividualT: 'i;

    /// Apply crossover operator to the selected population part.
    ///
    /// ## Arguments
    ///
    /// * `metadata` - metadata provided by the GA runtime,
    /// * `selected` - result of running selection operator,
    ///
    /// ## Returns
    ///
    /// Vector of individuals created during the crossover stage.
    fn apply(&mut self, metadata: &Metrics, selected: &[&IndividualT]) -> Vec<IndividualT>;
}
