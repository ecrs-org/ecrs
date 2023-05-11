//! Module contains implementations of policies on how to choose the best ant
//!
//! There are [PheromoneUpdate] implementations that make use of best found solution.
//! This module was create to accommodate two main choosing policies
//! * [OverallBest] - chooses the best in all iterations
//! * [Iteration] - chooses the best in current iteration
//!
use crate::aco::Solution;

/// # Best Choosing Policy
///
/// This trait defines common behaviors of choosing the best ant.
/// You can implement this trait to provide your own choosing policy
pub trait BestPolicy {
    /// Based on provided solutions, it updates it internal state.
    ///
    /// ## Arguments
    /// * `solutions` - current iteration solutions.
    fn update_best(&mut self, solutions: &[Solution]);

    /// Returns stored best pheromone trail
    fn get_best(&self) -> &Solution;
}

/// # Iteration best choosing policy
/// Implements [ChoosingPolicy].
/// Chooses pheromone from current iteration best ant.
pub struct IterationBest {
    best_solution: Solution,
}

impl IterationBest {
    /// Crates a new instance of [IterationBest]
    pub fn new() -> Self {
        Self {
            best_solution: Solution::default(),
        }
    }
}

impl BestPolicy for IterationBest {
    fn update_best(&mut self, solutions: &[Solution]) {
        if solutions.is_empty() {
            return;
        }
        self.best_solution = find_best(solutions).clone();
    }

    fn get_best(&self) -> &Solution {
        &self.best_solution
    }
}

/// # Overall best choosing policy
/// Implements [ChoosingPolicy].
/// Chooses pheromone from all past iteration best ant.
pub struct OverallBest {
    best_solution: Solution,
}

impl OverallBest {
    /// Crates a new instance of [OverallBest]
    pub fn new() -> Self {
        Self {
            best_solution: Solution::default(),
        }
    }
}

impl BestPolicy for OverallBest {
    fn update_best(&mut self, solutions: &[Solution]) {
        if solutions.is_empty() {
            return;
        }
        let iter_best = find_best(solutions);

        if iter_best.fitness > self.best_solution.fitness {
            self.best_solution = iter_best.clone()
        }
    }

    fn get_best(&self) -> &Solution {
        &self.best_solution
    }
}

#[inline]
fn find_best(solutions: &[Solution]) -> &Solution {
    solutions
        .iter()
        .reduce(|a, b| if a.fitness > b.fitness { a } else { b })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::aco::pheromone::best_policy::{BestPolicy, IterationBest, OverallBest};
    use crate::aco::Solution;

    #[test]
    fn iteration_best_returns_correct_pheromones() {
        let gen1 = [
            Solution {
                path: vec![0],
                fitness: 0.5,
            },
            Solution {
                path: vec![0],
                fitness: 0.25,
            },
        ];

        let gen2 = [
            Solution {
                path: vec![0],
                fitness: 0.125,
            },
            Solution {
                path: vec![0],
                fitness: 0.0625,
            },
        ];

        let mut best_pol = IterationBest::new();
        best_pol.update_best(&gen1);
        let best = best_pol.get_best();

        assert_eq!(best.fitness, 0.5);

        best_pol.update_best(&gen2);
        let best = best_pol.get_best();

        assert_eq!(best.fitness, 0.125);
    }

    #[test]
    fn overall_best_returns_correct_pheromones() {
        let gen1 = [
            Solution {
                path: vec![0],
                fitness: 0.5,
            },
            Solution {
                path: vec![0],
                fitness: 0.25,
            },
        ];

        let gen2 = [
            Solution {
                path: vec![0],
                fitness: 0.125,
            },
            Solution {
                path: vec![0],
                fitness: 0.0625,
            },
        ];

        let mut best_pol = OverallBest::new();
        best_pol.update_best(&gen1);
        let best = best_pol.get_best();

        assert_eq!(best.fitness, 0.5);

        best_pol.update_best(&gen2);
        let best = best_pol.get_best();

        assert_eq!(best.fitness, 0.5);
    }
}
