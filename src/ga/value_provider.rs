pub mod flat;

use num_traits::{Float, Num};

use super::Metrics;

// The `sized` bound here was not thinked through. It is possible
// to resign from it
pub trait ValueProvider<T: Sized + Clone> {
    fn get(&mut self, metrics: &Metrics) -> T;
}
