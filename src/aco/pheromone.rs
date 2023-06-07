//! Implementation of pheromone calculations strategies.
//!
use crate::aco::pheromone::best_policy::{BestPolicy, OverallBest};
use crate::aco::{FMatrix, Solution};
use itertools::{izip, Itertools};
use std::ops::Add;
use nalgebra::clamp;

pub mod best_policy;

pub trait Pheromone {}

impl Pheromone for FMatrix {}

impl Pheromone for Vec<FMatrix> {}

/// # Pheromone Update
///
/// This trait defines common behaviour for pheromone update calculations.
/// You can implement this trait to provide your custom way of calculating new pheromone to the ACO.
pub trait PheromoneUpdate<P: Pheromone> {
    /// Returns the new pheromone
    ///
    /// ## Arguments
    ///
    /// * `old_pheromone` - Pheromone used to generate current solutions
    /// * `solutions` - Current generated solution.
    /// * `evaporation_rate` - rate of old pheromone evaporation
    fn apply(&mut self, pheromone: &mut P, solutions: &[Solution], evaporation_rate: f64);
}

/// # Ant System Pheromone Update
///
/// Implements [PheromoneUpdate]. The pheromone is updated as first proposed by Marco Dorigo,
/// every ant leaves pheromone trail on its way, the pheromone trail strength is proportional
/// to the way fitness. New pheromone is a sum of old pheromone scaled by (1 - evaporation rate) and sum
/// of pheromone trails left by ants.
pub struct AntSystemPU;

impl AntSystemPU {
    /// Creates a new instance of [AntSystemPU]
    pub fn new() -> Self {
        AntSystemPU
    }
}

impl PheromoneUpdate<FMatrix> for AntSystemPU {
    fn apply(&mut self, pheromone: &mut FMatrix, solutions: &[Solution], evaporation_rate: f64) {
        pheromone.scale_mut(1.0 - evaporation_rate);
        leave_trail(solutions, pheromone);
    }
}

/// # Elitist Ant System Pheromone Update
///
/// Implements [PheromoneUpdate]. Similarity to [AntSystemPU], every ant leaves pheromone trail on its way,
/// the pheromone trail strength is proportional
/// to the way fitness. New pheromone is a sum of old pheromone scaled by (1 - evaporation rate) and sum
/// of pheromone trails left by ants, additionally we are adding pheromone left by the best ant overall.
pub struct ElitistAntSystemPU {
    overall_best: OverallBest,
}

impl ElitistAntSystemPU {
    /// Creates a new instance of [ElitistAntSystemPU]
    pub fn new() -> Self {
        ElitistAntSystemPU {
            overall_best: OverallBest::new(),
        }
    }
}

impl PheromoneUpdate<FMatrix> for ElitistAntSystemPU {
    fn apply(&mut self, pheromone: &mut FMatrix, solutions: &[Solution], evaporation_rate: f64) {
        self.overall_best.update_best(solutions);

        pheromone.scale_mut(1.0 - evaporation_rate);
        leave_trail(solutions, pheromone);
        leave_single_trail(self.overall_best.get_best(), pheromone);

    }
}

/// # MAX-MIN Ant System Pheromone Update
///
/// Implements [PheromoneUpdate].
/// the pheromone trail strength is proportional
/// to the way fitness. New pheromone is a sum of old pheromone scaled by (1 - evaporation rate) and
/// pheromone trail left by ant chosen by [BestPolicy], additionally the pheromone value is clamped.
pub struct MMAntSystemPU<B: BestPolicy> {
    pub(in crate::aco) best_policy: B,
    pub(in crate::aco) lower_bound: f64,
    pub(in crate::aco) upper_bound: f64,
}

impl<B: BestPolicy> MMAntSystemPU<B> {
    /// Creates an [MMAntSystemPU] with user provided implementation of [BestPolicy].
    ///
    /// ## Arguments
    /// * `lower_bound` - Minimal possible pheromone value.
    /// * `upper_bound` - Maximal possible pheromone value.
    /// * `best_policy` - Implementation of [BestPolicy]
    pub fn with_best_policy(lower_bound: f64, upper_bound: f64, best_policy: B) -> Self {
        assert!(lower_bound >= 0.0, "Lower bound must be grater or equal 0");
        assert!(
            upper_bound > lower_bound,
            "Lower bound must be smaller than upper bound"
        );

        Self {
            lower_bound,
            upper_bound,
            best_policy,
        }
    }
}

impl MMAntSystemPU<OverallBest> {
    /// Creates an [MMAntSystemPU] with [OverallBest] best ant choosing policy
    ///
    /// ## Arguments
    /// * `lower_bound` - Minimal possible pheromone value.
    /// * `upper_bound` - Maximal possible pheromone value.
    pub fn new(lower_bound: f64, upper_bound: f64) -> Self {
        Self::with_best_policy(lower_bound, upper_bound, OverallBest::new())
    }
}

impl<B: BestPolicy> PheromoneUpdate<FMatrix> for MMAntSystemPU<B> {
    fn apply(&mut self, pheromone: &mut FMatrix, solutions: &[Solution], evaporation_rate: f64) {
        self.best_policy.update_best(solutions);

        pheromone.scale_mut(1.0 - evaporation_rate);
        let s = self.best_policy.get_best();
        for (i, j) in s.path.iter().circular_tuple_windows::<(&usize, &usize)>() {
            pheromone[(*i, *j)] = clamp(pheromone[(*i, *j)] + s.fitness, self.lower_bound, self.upper_bound);
            pheromone[(*j, *i)] = pheromone[(*i, *j)];
        }

    }
}

/// # Ant Colony System Pheromone Update
///
/// Implements [PheromoneUpdate].
/// the pheromone trail strength is  proportional
/// to the way fitness. New pheromone is a sum of old pheromone scaled by (1 - evaporation rate) and
/// best ant pheromone trail scaled by evaporation rate. Best ant pheromone is selected based
/// on [BestPolicy] implementation.
pub struct AntColonySystemPU<B: BestPolicy> {
    pub(in crate::aco) best_policy: B,
}

impl AntColonySystemPU<OverallBest> {
    /// Creates an [AntColonySystemPU] with [OverallBest] best ant choosing policy
    pub fn new() -> Self {
        Self {
            best_policy: OverallBest::new(),
        }
    }
}

impl<B: BestPolicy> AntColonySystemPU<B> {
    /// Creates an [AntColonySystemPU] with user provided implementation of [BestPolicy].
    ///
    /// ## Arguments
    /// * `best_policy` - Implementation of [BestPolicy]
    pub fn with_policy(best_policy: B) -> Self {
        Self { best_policy }
    }
}

impl<B: BestPolicy> PheromoneUpdate<FMatrix> for AntColonySystemPU<B> {
    fn apply(&mut self, pheromone: &mut FMatrix, solutions: &[Solution], evaporation_rate: f64) {
        self.best_policy.update_best(solutions);

        pheromone.scale_mut(1.0 - evaporation_rate);

        let s = self.best_policy.get_best();
        let change =  s.fitness * evaporation_rate;
        for (i, j) in s.path.iter().circular_tuple_windows::<(&usize, &usize)>() {
            pheromone[(*i, *j)] += change;
            pheromone[(*j, *i)] += change;
        }
    }
}

/// # Part From Evaluation Pheromone Update
///
/// Implements [PheromoneUpdate].
/// The solution are split into the number of pheromone traits by value range
/// First pheromone in vec is updated by worst solutions.
pub struct PartFromEvalPU {
    pheromone_updates: Vec<Box<dyn PheromoneUpdate<FMatrix>>>,
    groups: Vec<Vec<Solution>>
}

impl PartFromEvalPU {
    pub fn new(pheromone_updates: Vec<Box<dyn PheromoneUpdate<FMatrix>>>) -> Self {
        let groups = pheromone_updates.iter().map(|_| Vec::<Solution>::new()).collect_vec();
        Self { pheromone_updates, groups}
    }
}

impl PheromoneUpdate<Vec<FMatrix>> for PartFromEvalPU {
    fn apply(
        &mut self,
        pheromone: &mut Vec<FMatrix>,
        solutions: &[Solution],
        evaporation_rate: f64,
    ){
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
        .for_each(|(pu, p, sg)| pu.apply(p, sg, evaporation_rate));

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

#[inline]
fn leave_trail(solutions: &[Solution], pheromone: &mut FMatrix) {
    for s in solutions.iter() {
        leave_single_trail(s, pheromone);
    }
}

// THINK: add scale parameter
#[inline]
fn leave_single_trail(s: &Solution, pheromone: &mut FMatrix) {
    for (i, j) in s.path.iter().circular_tuple_windows::<(&usize, &usize)>() {
        pheromone[(*i, *j)] += s.fitness;
        pheromone[(*j, *i)] += s.fitness;
    }
}

#[cfg(test)]
mod tests {
    use crate::aco::pheromone::{
        AntColonySystemPU, AntSystemPU, ElitistAntSystemPU, MMAntSystemPU, PheromoneUpdate,
    };
    use crate::aco::{FMatrix, Solution};

    fn get_test_data() -> (FMatrix, [Solution; 2]) {
        let mut pheromone = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 2.0, 1.0, 0.0, 4.0, 2.0, 4.0, 0.0]);

        let sols = [
            Solution {
                path: vec![0, 1, 2],
                fitness: 0.125,
            },
            Solution {
                path: vec![0, 1, 2],
                fitness: 0.25,
            },
        ];

        (pheromone, sols)
    }

    fn are_same(pher: FMatrix, pher_expt: &[f64]) {
        for (p_calc, p_expt) in pher.iter().zip(pher_expt.iter()) {
            assert_eq!(p_calc, p_expt);
        }
    }

    #[test]
    fn check_ant_system_pu_with_example() {
        let (mut pher, sols) = get_test_data();

        let mut pu = AntSystemPU;
        pu.apply(&mut pher, &sols, 0.25);
        let pher_expt = vec![0.0, 1.125, 1.875, 1.125, 0.0, 3.375, 1.875, 3.375, 0.0];

        are_same(pher, &pher_expt);
    }

    #[test]
    fn check_elitist_ant_system_pu_with_example() {
        let pher_expt = vec![0.0, 1.375, 2.125, 1.375, 0.0, 3.625, 2.125, 3.625, 0.0];
        let (mut pher, sols) = get_test_data();

        let mut pu = ElitistAntSystemPU::new();

        pu.apply(&mut pher, &sols, 0.25);
        are_same(pher, &pher_expt);
    }

    #[test]
    fn check_max_min_ant_system_pu_with_example() {
        let pher_expt = vec![1.5, 1.5, 1.75, 1.5, 1.5, 3.0, 1.75, 3.0, 1.5];
        let (mut pher, sols) = get_test_data();

        let mut pu = MMAntSystemPU::new(1.5, 3.0);
        pu.apply(&mut pher, &sols, 0.25);
        are_same(pher, &pher_expt);

    }

    #[test]
    fn check_ant_colony_system_pu_with_example() {
        let pher_expt = vec![0.0, 0.8125, 1.5625, 0.8125, 0.0, 3.0625, 1.5625, 3.0625, 0.0];
        let (mut pher, sols) = get_test_data();

        let mut pu = AntColonySystemPU::new();
        pu.apply(&mut pher, &sols, 0.25);
        are_same(pher, &pher_expt);

    }
}
