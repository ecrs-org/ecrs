#[cfg(feature = "ga_impl_selection")]
pub mod impls;
#[cfg(feature = "ga_impl_selection")]
pub use impls::*;

use crate::ga::{individual::IndividualTrait, Metrics};

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
pub trait SelectionOperator<IndividualT: IndividualTrait> {
    /// Returns a vector of references to individuals selected to mating pool
    ///
    /// ### Arguments
    ///
    /// * `metrics` - [crate::ga::Metrics] information on current stage of the algorithm (iteration, elapsed time, etc.)
    /// * `population` - individuals to choose mating pool from
    fn apply<'a>(
        &mut self,
        metrics: &Metrics,
        population: &'a [IndividualT],
    ) -> Vec<&'a IndividualT>;
}
