use crate::aco::{AdditionalArgs, Solution};

/// # Grader
///
/// Trait defining method needed for grading.
pub trait Grader<Args: AdditionalArgs> {
    /// Calculates fitness of given solutions.
    ///
    ///  ## Arguments
    /// * `sols` - solutions with unknown fitness
    /// * `args` - problem specific args
    fn apply(&mut self, sols: &mut [Solution], args: &Args);
}
