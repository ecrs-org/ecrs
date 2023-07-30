//! Implementation of pheromone calculations strategies.
//!
use crate::aco::{AdditionalArgs, FMatrix, Solution};
use itertools::{izip, Itertools};

pub trait Pheromone {}

impl Pheromone for FMatrix {}

impl Pheromone for Vec<FMatrix> {}

/// # Pheromone Update
///
/// This trait defines common behaviour for pheromone update calculations.
/// You can implement this trait to provide your custom way of calculating new pheromone to the ACO.
pub trait PheromoneUpdate<P: Pheromone, Args: AdditionalArgs = ()> {
    /// Returns the new pheromone
    ///
    /// ## Arguments
    ///
    /// * `old_pheromone` - Pheromone used to generate current solutions
    /// * `solutions` - Current generated solution.
    fn apply(&mut self, pheromone: &mut P, solutions: &[Solution], args: &Args);
}

/// # Part From Evaluation Pheromone Update
///
/// Implements [PheromoneUpdate].
/// The solution are split into the number of pheromone traits by value range
/// First pheromone in vec is updated by worst solutions.
pub struct PartFromEvalPU<Args: AdditionalArgs = ()> {
    pheromone_updates: Vec<Box<dyn PheromoneUpdate<FMatrix, Args>>>,
    groups: Vec<Vec<Solution>>,
}

impl<Args: AdditionalArgs> PartFromEvalPU<Args> {
    pub fn new(pheromone_updates: Vec<Box<dyn PheromoneUpdate<FMatrix, Args>>>) -> Self {
        let groups = pheromone_updates
            .iter()
            .map(|_| Vec::<Solution>::new())
            .collect_vec();
        Self {
            pheromone_updates,
            groups,
        }
    }
}

impl<Args: AdditionalArgs> PheromoneUpdate<Vec<FMatrix>, Args> for PartFromEvalPU<Args> {
    fn apply(&mut self, pheromone: &mut Vec<FMatrix>, solutions: &[Solution], args: &Args) {
        let parts_num = pheromone.len() as f64;
        let (min, max) = find_bounds(solutions);
        let increment = (max - min) / parts_num;

        self.groups.iter_mut().for_each(Vec::clear);

        for s in solutions.iter() {
            let part = ((s.fitness - min) / increment) as usize;
            let i = part.clamp(0, pheromone.len() - 1);
            self.groups[i].push(s.clone())
        }

        izip!(
            self.pheromone_updates.iter_mut(),
            pheromone.iter_mut(),
            self.groups.iter()
        )
        .for_each(|(pu, p, sg)| pu.apply(p, sg, args));
    }
}

fn find_bounds(solutions: &[Solution]) -> (f64, f64) {
    let mut min = solutions[0].fitness;
    let mut max = min;
    for sol in solutions {
        if sol.fitness < min {
            min = sol.fitness
        }
        if sol.fitness > max {
            max = sol.fitness
        }
    }
    (min, max)
}
