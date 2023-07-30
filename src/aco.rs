//! Implementation of Ant based algorithms
//!
//! # Ant Colony Optimization
//! Implementation is based on those sources:
//! * <https://ieeexplore.ieee.org/document/4129846> DOI: 10.1109/MCI.2006.329691
//! * <http://www.scholarpedia.org/article/Ant_colony_optimization>
//!
//! Look at [Builder](Builder) for parameters overview and
//! at [AntColonyOptimization] for interface details
//!
//! Logging system details can be found [here](probe)
//!
//! ```
pub mod ant;
pub mod ants_behaviour;
pub mod builder;
pub mod colony;
pub mod fitness;
pub mod goodness;
pub mod grader;
pub mod local_update;
pub mod pheromone;
pub mod probe;
mod solution;
pub mod termination_condition;
pub mod util;

pub use builder::Builder;
use itertools::Itertools;
pub use solution::Solution;

use crate::aco::colony::Colony;
use crate::aco::grader::Grader;
use crate::aco::pheromone::{Pheromone, PheromoneUpdate};
use crate::aco::probe::Probe;
use crate::aco::termination_condition::TerminationCondition;
use nalgebra::{Dyn, OMatrix};

pub type FMatrix = OMatrix<f64, Dyn, Dyn>;

pub trait AdditionalArgs {}

impl AdditionalArgs for () {}

/// # Ant Colony Optimization
///
/// Encapsulates common ACO algorithm patterns.
///
/// To extract data use a [probe](probe)
pub struct AntColonyOptimization<P, C, G, T, Pr, Ph, Args = ()>
where
    P: PheromoneUpdate<Ph, Args>,
    C: Colony<Ph, Args>,
    G: Grader<Args>,
    T: TerminationCondition<Ph, Args>,
    Pr: Probe<Ph, Args>,
    Ph: Pheromone,
    Args: AdditionalArgs,
{
    colony: C,
    pheromone_update: P,
    pheromone: Ph,
    grader: G,
    termination_cond: T,
    probe: Pr,
    additional_args: Args,
}

impl<P, C, G, T, Pr, Ph, Args> AntColonyOptimization<P, C, G, T, Pr, Ph, Args>
where
    P: PheromoneUpdate<Ph, Args>,
    C: Colony<Ph, Args>,
    G: Grader<Args>,
    T: TerminationCondition<Ph, Args>,
    Pr: Probe<Ph, Args>,
    Ph: Pheromone,
    Args: AdditionalArgs,
{
    /// Executes the algorithm
    pub fn run(mut self) {
        self.termination_cond.init(&self.pheromone, &self.additional_args);
        while !self
            .termination_cond
            .update_and_check(&self.pheromone, &self.additional_args)
        {
            self.probe.on_iteration_start(&self.additional_args);
            self.iterate();
            self.probe.on_iteration_end(&self.additional_args);
        }

        self.end()
    }

    fn iterate(&mut self) {
        let paths = self
            .colony
            .build_solutions(&mut self.pheromone, &self.additional_args);
        let sols = self.grade(paths);

        let best = self.find_best(&sols);
        self.probe.on_current_best(best, &self.additional_args);

        self.pheromone_update
            .apply(&mut self.pheromone, &sols, &self.additional_args);

        self.probe
            .on_pheromone_update(&self.pheromone, &self.additional_args);
    }

    fn find_best<'a>(&mut self, sols: &'a [Solution]) -> &'a Solution {
        let best = sols
            .iter()
            .reduce(|a, b| if a.fitness > b.fitness { a } else { b });
        best.unwrap()
    }

    fn grade(&mut self, paths: Vec<Vec<usize>>) -> Vec<Solution> {
        let mut sols = paths.into_iter().map(Solution::from_path).collect_vec();
        self.grader.apply(&mut sols, &self.additional_args);
        sols
    }

    fn end(mut self) {
        self.probe.on_end(&self.additional_args);
    }
}
