use crate::aco::pheromone::PheromoneUpdate;
use crate::aco::tsp::pheromone::best_policy::{BestPolicy, OverallBest};
use crate::aco::{AdditionalArgs, FMatrix, Solution};
use itertools::Itertools;
use nalgebra::clamp;

pub mod best_policy;

/// # Ant System Pheromone Update
///
/// Implements [PheromoneUpdate]. The pheromone is updated as first proposed by Marco Dorigo,
/// every ant leaves pheromone trail on its way, the pheromone trail strength is proportional
/// to the way fitness. New pheromone is a sum of old pheromone scaled by (1 - evaporation rate) and sum
/// of pheromone trails left by ants.
pub struct AntSystemPU {
    evaporation_rate: f64,
}

impl AntSystemPU {
    /// Creates a new instance of [AntSystemPU]
    ///
    /// ## Arguments
    /// * `evaporation_rate` - rate of old pheromone evaporation
    pub fn new(evaporation_rate: f64) -> Self {
        AntSystemPU { evaporation_rate }
    }
}

impl<Args: AdditionalArgs> PheromoneUpdate<FMatrix, Args> for AntSystemPU {
    fn apply(&mut self, pheromone: &mut FMatrix, solutions: &[Solution], _: &Args) {
        pheromone.scale_mut(1.0 - self.evaporation_rate);
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
    evaporation_rate: f64,
    overall_best: OverallBest,
}

impl ElitistAntSystemPU {
    /// Creates a new instance of [ElitistAntSystemPU]
    ///
    /// ## Arguments
    /// * `evaporation_rate` - rate of old pheromone evaporation
    pub fn new(evaporation_rate: f64) -> Self {
        ElitistAntSystemPU {
            overall_best: OverallBest::new(),
            evaporation_rate,
        }
    }
}

impl<Args: AdditionalArgs> PheromoneUpdate<FMatrix, Args> for ElitistAntSystemPU {
    fn apply(&mut self, pheromone: &mut FMatrix, solutions: &[Solution], _: &Args) {
        self.overall_best.update_best(solutions);

        pheromone.scale_mut(1.0 - self.evaporation_rate);
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
    best_policy: B,
    lower_bound: f64,
    upper_bound: f64,
    evaporation_rate: f64,
}

impl<B: BestPolicy> MMAntSystemPU<B> {
    /// Creates an [MMAntSystemPU] with user provided implementation of [BestPolicy].
    ///
    /// ## Arguments
    /// * `evaporation_rate` - rate of old pheromone evaporation
    /// * `lower_bound` - Minimal possible pheromone value.
    /// * `upper_bound` - Maximal possible pheromone value.
    /// * `best_policy` - Implementation of [BestPolicy]
    pub fn with_best_policy(
        evaporation_rate: f64,
        lower_bound: f64,
        upper_bound: f64,
        best_policy: B,
    ) -> Self {
        assert!(lower_bound >= 0.0, "Lower bound must be grater or equal 0");
        assert!(
            upper_bound > lower_bound,
            "Lower bound must be smaller than upper bound"
        );

        Self {
            lower_bound,
            upper_bound,
            best_policy,
            evaporation_rate,
        }
    }
}

impl MMAntSystemPU<OverallBest> {
    /// Creates an [MMAntSystemPU] with [OverallBest] best ant choosing policy
    ///
    /// ## Arguments
    /// * `evaporation_rate` - rate of old pheromone evaporation
    /// * `lower_bound` - Minimal possible pheromone value.
    /// * `upper_bound` - Maximal possible pheromone value.
    pub fn new(evaporation_rate: f64, lower_bound: f64, upper_bound: f64) -> Self {
        Self::with_best_policy(evaporation_rate, lower_bound, upper_bound, OverallBest::new())
    }
}

impl<B: BestPolicy, Args: AdditionalArgs> PheromoneUpdate<FMatrix, Args> for MMAntSystemPU<B> {
    fn apply(&mut self, pheromone: &mut FMatrix, solutions: &[Solution], _: &Args) {
        self.best_policy.update_best(solutions);

        pheromone.scale_mut(1.0 - self.evaporation_rate);
        let s = self.best_policy.get_best();
        for (i, j) in s.path.iter().circular_tuple_windows::<(&usize, &usize)>() {
            pheromone[(*i, *j)] = clamp(
                pheromone[(*i, *j)] + s.fitness,
                self.lower_bound,
                self.upper_bound,
            );
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
    evaporation_rate: f64,
    best_policy: B,
}

impl<B: BestPolicy> AntColonySystemPU<B> {
    pub fn with_best_policy(evaporation_rate: f64, best_policy: B) -> Self {
        Self {
            evaporation_rate,
            best_policy,
        }
    }
}

impl AntColonySystemPU<OverallBest> {
    /// Creates an [AntColonySystemPU] with [OverallBest] best ant choosing policy
    ///
    /// ## Arguments
    /// * `evaporation_rate` - rate of old pheromone evaporation
    pub fn new(evaporation_rate: f64) -> Self {
        Self {
            evaporation_rate,
            best_policy: OverallBest::new(),
        }
    }
}

impl<B: BestPolicy, Args: AdditionalArgs> PheromoneUpdate<FMatrix, Args> for AntColonySystemPU<B> {
    fn apply(&mut self, pheromone: &mut FMatrix, solutions: &[Solution], _: &Args) {
        self.best_policy.update_best(solutions);

        pheromone.scale_mut(1.0 - self.evaporation_rate);

        let s = self.best_policy.get_best();
        let change = s.fitness * self.evaporation_rate;
        for (i, j) in s.path.iter().circular_tuple_windows::<(&usize, &usize)>() {
            pheromone[(*i, *j)] += change;
            pheromone[(*j, *i)] += change;
        }
    }
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
    use crate::aco::pheromone::PheromoneUpdate;
    use crate::aco::tsp::pheromone::{AntColonySystemPU, AntSystemPU, ElitistAntSystemPU, MMAntSystemPU};
    use crate::aco::{FMatrix, Solution};

    fn get_test_data() -> (FMatrix, [Solution; 2]) {
        let pheromone = FMatrix::from_column_slice(3, 3, &[0.0, 1.0, 2.0, 1.0, 0.0, 4.0, 2.0, 4.0, 0.0]);

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

        let mut pu = AntSystemPU::new(0.25);
        pu.apply(&mut pher, &sols, &());
        let pher_expt = vec![0.0, 1.125, 1.875, 1.125, 0.0, 3.375, 1.875, 3.375, 0.0];

        are_same(pher, &pher_expt);
    }

    #[test]
    fn check_elitist_ant_system_pu_with_example() {
        let pher_expt = vec![0.0, 1.375, 2.125, 1.375, 0.0, 3.625, 2.125, 3.625, 0.0];
        let (mut pher, sols) = get_test_data();

        let mut pu = ElitistAntSystemPU::new(0.25);

        pu.apply(&mut pher, &sols, &());
        are_same(pher, &pher_expt);
    }

    #[test]
    fn check_max_min_ant_system_pu_with_example() {
        let pher_expt = vec![0.0, 1.5, 1.75, 1.5, 0.0, 3.0, 1.75, 3.0, 0.0];
        let (mut pher, sols) = get_test_data();

        let mut pu = MMAntSystemPU::new(0.25, 1.5, 3.0);
        pu.apply(&mut pher, &sols, &());
        are_same(pher, &pher_expt);
    }

    #[test]
    fn check_ant_colony_system_pu_with_example() {
        let pher_expt = vec![0.0, 0.8125, 1.5625, 0.8125, 0.0, 3.0625, 1.5625, 3.0625, 0.0];
        let (mut pher, sols) = get_test_data();

        let mut pu = AntColonySystemPU::new(0.25);
        pu.apply(&mut pher, &sols, &());
        are_same(pher, &pher_expt);
    }
}
