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
pub mod local_update;
pub mod pheromone;
pub mod probe;
mod solution;
pub mod termination_condition;
pub mod util;

pub use builder::Builder;
pub use solution::Solution;

use crate::aco::colony::Colony;
use crate::aco::fitness::Fitness;
use crate::aco::pheromone::{Pheromone, PheromoneUpdate};
use crate::aco::probe::Probe;
use crate::aco::termination_condition::TerminationCondition;
use nalgebra::{Dyn, OMatrix};

pub type FMatrix = OMatrix<f64, Dyn, Dyn>;

/// # Ant Colony Optimization
///
/// Encapsulates common ACO algorithm patterns.
///
/// To extract data use a [probe](probe)
pub struct AntColonyOptimization<P, C, F, T, Pr, Ph>
where
    P: PheromoneUpdate<Ph>,
    C: Colony<Ph>,
    F: Fitness,
    T: TerminationCondition<Ph>,
    Pr: Probe<Ph>,
    Ph: Pheromone,
{
    colony: C,
    pheromone_update: P,
    evaporation_rate: f64,
    pheromone: Ph,
    fitness: F,
    termination_cond: T,
    probe: Pr,
}

impl<P, C, F, T, Pr, Ph> AntColonyOptimization<P, C, F, T, Pr, Ph>
where
    P: PheromoneUpdate<Ph>,
    C: Colony<Ph>,
    F: Fitness,
    T: TerminationCondition<Ph>,
    Pr: Probe<Ph>,
    Ph: Pheromone,
{
    /// Executes the algorithm
    pub fn run(mut self) {
        self.termination_cond.init(&self.pheromone);
        while !self.termination_cond.update_and_check(&self.pheromone) {
            self.probe.on_iteration_start();
            self.iterate();
            self.probe.on_iteration_end();
        }

        self.end()
    }

    fn iterate(&mut self) {
        let paths = self.colony.build_solutions(&mut self.pheromone);
        let sols = self.grade(paths);

        let best = self.find_best(&sols);
        self.probe.on_current_best(best);

        let new_pheromone = self
            .pheromone_update
            .apply(&self.pheromone, &sols, self.evaporation_rate);

        self.probe.on_pheromone_update(&self.pheromone, &new_pheromone);
        self.pheromone = new_pheromone;
    }

    fn find_best<'a>(&mut self, sols: &'a [Solution]) -> &'a Solution {
        let best = sols
            .iter()
            .reduce(|a, b| if a.fitness > b.fitness { a } else { b });

        best.unwrap()
    }

    fn grade(&mut self, paths: Vec<Vec<usize>>) -> Vec<Solution> {
        let mut sols: Vec<Solution> = Vec::with_capacity(paths.len());

        for path in paths {
            let fitness = self.fitness.apply(&path);

            let mut solution = Solution::from_path(path);
            solution.fitness = fitness;
            sols.push(solution);
        }

        sols
    }

    fn end(mut self) {
        self.probe.on_end();
    }
}
