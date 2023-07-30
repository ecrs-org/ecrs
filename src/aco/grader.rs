use crate::aco::{AdditionalArgs, FMatrix, Solution};
use itertools::Itertools;

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
    use crate::aco::grader::PathLengthInverse;
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
