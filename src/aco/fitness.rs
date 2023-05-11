//! Ants paths grading.
//!
//! Contains trait [Fitness] that should be implemented for grading structure.
//! Higher fitness means beater path.
//! <b> Fitness must be grater or equal 0 </b>
use crate::aco::FMatrix;
use itertools::Itertools;

/// # Fitness
///
/// Trait defining method needed for grading.
pub trait Fitness {
    /// Calculates fitness of given path.
    ///
    ///  ## Arguments
    /// * `path` - path in sequence of vertices form.
    fn apply(&mut self, path: &[usize]) -> f64;
}

/// # Canonical Fitness
///
/// Calculates fitness as 1.0 / path_cost
pub struct CanonicalFitness {
    pub(in crate::aco) weights: FMatrix,
}

impl CanonicalFitness {
    /// Creates a new instance of [CanonicalFitness].
    pub fn new(weights: FMatrix) -> Self {
        Self { weights }
    }
}

impl Fitness for CanonicalFitness {
    fn apply(&mut self, path: &[usize]) -> f64 {
        let mut cost = 0.0f64;
        for (i, j) in path.iter().circular_tuple_windows::<(&usize, &usize)>() {
            cost += self.weights[(*i, *j)];
        }

        1.0 / cost
    }
}

#[cfg(test)]
mod tests {
    use crate::aco::fitness::{CanonicalFitness, Fitness};
    use crate::aco::FMatrix;

    #[test]
    fn canonical_fitness_returns_correct_fitness() {
        let weights = FMatrix::from_vec(2, 2, vec![0.0, 2.0, 4.0, 0.0]);
        let mut fittness = CanonicalFitness::new(weights);
        let path = [0usize, 1];
        assert_eq!(fittness.apply(&path), 1.0 / 6.0);
        let path = [1usize, 0];
        assert_eq!(fittness.apply(&path), 1.0 / 6.0);
    }
}
