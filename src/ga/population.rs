#[cfg(feature = "ga_impl_population")]
pub mod impls;
#[cfg(feature = "ga_impl_population")]
pub use impls::*;

pub mod tools;

use std::vec::IntoIter;

use super::individual::IndividualTrait;

/// Implement this trait in order to provide custom population generator
/// and feed it to an solver.
pub trait PopulationGenerator<IndividualT: IndividualTrait> {
    fn generate(&mut self, count: usize) -> Vec<IndividualT>;

    fn generate_iter(&mut self, count: usize) -> IntoIter<IndividualT> {
        self.generate(count).into_iter()
    }
}

