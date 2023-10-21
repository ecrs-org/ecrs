use std::collections::HashSet;

use ecrs::ga::individual::IndividualTrait;
use itertools::Itertools;
use log::{debug, info, trace, warn};

use super::{Edge, EdgeKind, Machine, Operation};

/// Models single solution to the JSSP problem instance
#[derive(Debug, Clone)]
pub struct JsspIndividual {
    /// Encoding of the solution. This can be decoded to the proper solution
    pub chromosome: Vec<f64>,
    /// Clone of all operations from the problem instance
    pub operations: Vec<Operation>,
    /// Clone of all machines from the problem instance
    pub machines: Vec<Machine>,
    /// If computed - fitness value of this solution. Check `is_fitness_valid`
    /// property to determine whether this value is up to date
    /// This is not an Option for some practical reasons
    /// TODO: But this should be an Option or some enum with additional information
    pub fitness: usize,
    /// If `true` the `fitness` field holds the value for the current `chromosome`
    /// and does need to be recomputed. This must be kept in sync!
    pub is_fitness_valid: bool,
    /// TODO: Determine what I've used it for
    pub is_dirty: bool,
}

impl JsspIndividual {
    pub fn new(chromosome: Vec<f64>, ops: Vec<Operation>, machines: Vec<Machine>, fitness: usize) -> Self {
        Self {
            chromosome,
            operations: ops,
            machines,
            fitness,
            is_fitness_valid: false,
            is_dirty: false,
        }
    }

    /// Resets all machines & operations associated with this individual
    pub(super) fn reset(&mut self) {
        self.machines.iter_mut().for_each(|machine| machine.reset());
        self.operations.iter_mut().for_each(|op| op.reset());
    }
}

impl PartialEq for JsspIndividual {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Eq for JsspIndividual {}

impl PartialOrd for JsspIndividual {
    #[allow(clippy::incorrect_partial_ord_impl_on_ord_type)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl Ord for JsspIndividual {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fitness.cmp(&other.fitness)
    }
}

impl IndividualTrait for JsspIndividual {
    type ChromosomeT = Vec<f64>;
    type FitnessValueT = usize;

    #[inline]
    fn chromosome(&self) -> &Self::ChromosomeT {
        &self.chromosome
    }

    #[inline]
    fn chromosome_mut(&mut self) -> &mut Self::ChromosomeT {
        &mut self.chromosome
    }

    #[inline]
    fn fitness(&self) -> Self::FitnessValueT {
        self.fitness
    }

    #[inline]
    fn fitness_mut(&mut self) -> &mut Self::FitnessValueT {
        &mut self.fitness
    }

    #[inline]
    fn requires_evaluation(&self) -> bool {
        !self.is_fitness_valid
    }
}

impl From<Vec<f64>> for JsspIndividual {
    fn from(chromosome: Vec<f64>) -> Self {
        Self {
            chromosome,
            operations: Vec::new(),
            machines: Vec::new(),
            fitness: usize::MAX,
            is_fitness_valid: false,
            is_dirty: false,
        }
    }
}
