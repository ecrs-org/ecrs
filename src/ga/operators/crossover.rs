#[cfg(feature = "ga_impl_crossover")]
pub mod impls;
#[cfg(feature = "ga_impl_crossover")]
pub use impls::*;

use crate::ga::individual::IndividualTrait;
use crate::ga::GAMetadata;

/// # Crossover Operator
///
/// This trait defines common behaviour for crossover operators.
/// You can implement this trait to provide your custom crossover operator to the GA.
pub trait CrossoverOperator<IndividualT: IndividualTrait> {
    /// Returns a tuple of children
    ///
    /// ## Arguments
    ///
    /// * `parent_1` - First parent to take part in recombination
    /// * `parent_2` - Second parent to take part in recombination
    fn apply_legacy(
        &mut self,
        metadata: &GAMetadata,
        parent_1: &IndividualT,
        parent_2: &IndividualT,
    ) -> (IndividualT, IndividualT);

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
    /// * `output` - buffer for the individuals created by the crossover operator.
    fn apply(&mut self, metadata: &GAMetadata, selected: &[&IndividualT], output: &mut Vec<IndividualT>);
}
