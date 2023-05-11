//! Contains stuff related to algorithm termination
use crate::aco::pheromone::Pheromone;
use std::time::{Duration, Instant};

/// # Termination Condition
///
/// Represents common interface between termination conditions
pub trait TerminationCondition<P: Pheromone> {
    /// Initialises condition internal state.
    ///
    /// ## Arguments
    /// * `pheromone` - start_pheromone.
    fn init(&mut self, pheromone: &P);

    /// Updates internal state and then checks if condition is met.
    ///
    /// ## Arguments
    /// * `pheromone` - current pheromone.
    /// * `ants` - Ants containing solutions.
    fn update_and_check(&mut self, pheromone: &P) -> bool;
}

/// # Iteration Condition
///
/// Implements [TerminationCondition].
///
/// Adds an limit on maximal number of iterations.
pub struct IterationCond {
    curr_iteration: usize,
    iterations_limit: usize,
}

impl IterationCond {
    /// Creates a new instance of [IterationCond]. Implements [TerminationCondition]
    ///
    /// ## Arguments
    /// * `iterations_limit` - how many iterations should take place;
    pub fn new(iterations_limit: usize) -> Self {
        Self {
            curr_iteration: 0,
            iterations_limit,
        }
    }
}

impl<P: Pheromone> TerminationCondition<P> for IterationCond {
    fn init(&mut self, _pheromone: &P) {
        self.curr_iteration = 0;
    }

    fn update_and_check(&mut self, _pheromone: &P) -> bool {
        self.curr_iteration += 1;

        self.curr_iteration > self.iterations_limit
    }
}

/// # Time Condition
/// Implements [TerminationCondition].
///
/// If by the end of iteration the algorithm is running longer than specified duration,
/// this condition will be met.
pub struct TimeCond {
    start_time: Instant,
    duration: Duration,
}

impl TimeCond {
    /// Creates a new instance of [TimeCond]. Implements [TerminationCondition]
    ///
    /// ## Arguments
    /// * `duration` - minimal amount of time to terminate algorithm.
    pub fn new(duration: Duration) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
        }
    }
}

impl<P: Pheromone> TerminationCondition<P> for TimeCond {
    fn init(&mut self, _pheromone: &P) {
        self.start_time = Instant::now()
    }

    fn update_and_check(&mut self, _pheromone: &P) -> bool {
        let curr_duration = Instant::now() - self.start_time;

        curr_duration > self.duration
    }
}
