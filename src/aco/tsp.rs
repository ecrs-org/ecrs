//! implementations of aco traits helpful in creating a TSP solver.

use crate::aco::colony::Colony;
use crate::aco::grader::Grader;
use crate::aco::pheromone::Pheromone;
use crate::aco::tsp::ant::Ant;
use crate::aco::tsp::ants_behaviour::AntsBehaviour;
use crate::aco::tsp::goodness::Goodness;
use crate::aco::{AdditionalArgs, FMatrix, Solution};
use itertools::Itertools;
use std::marker::PhantomData;

pub mod ant;
pub mod ants_behaviour;
pub mod goodness;
pub mod local_update;
pub mod pheromone;
pub mod util;

/// # TSP Colony
///
/// Uses [AntsBehaviour], [Ant], and [Goodness] to customize colony behavior.
pub struct TspColony<P: Pheromone, AB: AntsBehaviour<A, G, P>, A: Ant, G: Goodness<P>> {
    ants_behaviour: AB,
    goodness: G,
    ants: Vec<A>,
    _phantom: PhantomData<P>,
}

impl<P: Pheromone, AB: AntsBehaviour<A, G, P>, A: Ant, G: Goodness<P>> TspColony<P, AB, A, G> {
    pub fn new(ants_behaviour: AB, goodness: G, ants: Vec<A>) -> Self {
        Self {
            ants_behaviour,
            goodness,
            ants,
            _phantom: Default::default(),
        }
    }
}

impl<P, AB, A, G, Args> Colony<P, Args> for TspColony<P, AB, A, G>
where
    P: Pheromone,
    AB: AntsBehaviour<A, G, P>,
    A: Ant,
    G: Goodness<P>,
    Args: AdditionalArgs,
{
    fn build_solutions(&mut self, pheromone: &mut P, _: &Args) -> Vec<Vec<usize>> {
        self.ants_behaviour
            .simulate_ants(&mut self.ants, pheromone, &mut self.goodness)
    }
}

/// # PathLengthInverse
///
/// Calculates fitness as 1.0 / path_cost
pub struct PathLengthInverse {
    weights: FMatrix,
}

impl PathLengthInverse {
    pub fn new(weights: FMatrix) -> Self {
        Self { weights }
    }

    fn grade_solution(&self, sol: &mut Solution) {
        let cost: f64 = sol
            .path
            .iter()
            .cloned()
            .circular_tuple_windows::<(usize, usize)>()
            .map(|edge| self.weights[edge])
            .sum();

        sol.fitness = 1.0 / cost;
    }
}

impl<Args: AdditionalArgs> Grader<Args> for PathLengthInverse {
    fn apply(&mut self, sols: &mut [Solution], _: &Args) {
        sols.iter_mut().for_each(|sol| self.grade_solution(sol))
    }
}

#[cfg(test)]
mod tests {
    use crate::aco::tsp::PathLengthInverse;
    use crate::aco::{FMatrix, Solution};

    #[test]
    fn inverse_path_length_grader_returns_correct_fitness() {
        let weights = FMatrix::from_vec(2, 2, vec![0.0, 2.0, 4.0, 0.0]);
        let grader = PathLengthInverse::new(weights);

        let path = [0usize, 1];
        let mut sol = Solution::from_path(path.into());
        grader.grade_solution(&mut sol);
        assert_eq!(sol.fitness, 1.0 / 6.0);

        let path = [1usize, 0];
        let mut sol = Solution::from_path(path.into());
        grader.grade_solution(&mut sol);
        assert_eq!(sol.fitness, 1.0 / 6.0);
    }
}
