use std::{iter::Sum, ops::Index};

use num_traits::{identities::Zero, NumAssignOps};
use rand::{
    distributions::{Standard, Uniform},
    prelude::Distribution,
    rngs::ThreadRng,
    Rng,
};

use crate::ga::{individual::IndividualTrait, value_provider::ValueProvider, Metrics};

use super::SelectionOperator;

/// ### Routelle wheel selection operator
///
/// This struct implements [SelectionOperator] trait and can be used with GA.
///
/// **Note 1**: This selection operator requires positive fitness function. No runtime checks are performed
/// to assert this invariant. If aggregated fitness in whole population is <= 0 the behaviour is undefined,
/// implementation dependent and might change without any notice.
///
/// **Note 2**: The same individual can be selected multiple times.
///
/// Individuals are selected with probability proportional to their fitness value. More specifically:
/// probability of selecting chromosome `C` from population `P` is `fitness(C)` / `sum_of_fitness_in_whole_population`.
pub struct RouletteWheel<SizeValue: ValueProvider<usize>, R: Rng = ThreadRng> {
    selection_size: SizeValue,
    rng: R,
}

impl<SizeValue: ValueProvider<usize>> RouletteWheel<SizeValue, ThreadRng> {
    /// Returns new instance of [RouletteWheel] selection operator with default RNG.
    ///
    /// ## Arguments
    ///
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    pub fn new(selection_size: SizeValue) -> Self {
        RouletteWheel::with_rng(selection_size, rand::thread_rng())
    }
}

impl<SizeValue: ValueProvider<usize>, R: Rng> RouletteWheel<SizeValue, R> {
    /// Returns new instance of [RouletteWheel] selection operator with custom RNG
    ///
    /// ## Arguments
    ///
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    pub fn with_rng(selection_size: SizeValue, rng: R) -> Self {
        RouletteWheel { selection_size, rng }
    }
}

// FIXME: It will return empty vector if total_fitness == 0
// WORKING CHANGE: crt >= threshold instead of crt_sum > threshold
// But this should be resolved some other way
impl<IndividualT: IndividualTrait, SizeValue: ValueProvider<usize>, R: Rng> SelectionOperator<IndividualT>
    for RouletteWheel<SizeValue, R>
where
    IndividualT::FitnessValueT: NumAssignOps + Sum<IndividualT::FitnessValueT> + PartialOrd + Copy,
    Standard: Distribution<IndividualT::FitnessValueT>,
{
    /// Returns a vector of references to individuals selected to mating pool
    ///
    /// **Note 1**: This selection operator requires positive fitness function. No runtime checks are performed
    /// to assert this invariant. If aggregated fitness in whole population is <= 0 the behaviour is undefined,
    /// implementation dependent and might change without any notice.
    ///
    /// **Note 2**: The same individual can be selected multiple times.
    ///
    /// Individuals are selected with probability proportional to their fitness value. More specifically:
    /// probability of selecting chromosome `C` from population `P` is `fitness(C)` / `sum_of_fitness_in_whole_population`.
    ///
    /// ### Arguments
    ///
    /// * `metrics` - [crate::ga::Metrics] information on current stage of the algorithm (iteration, elapsed time, etc.)
    /// * `population` - individuals to choose mating pool from
    fn apply<'a>(&mut self, metrics: &Metrics, population: &'a [IndividualT]) -> Vec<&'a IndividualT> {
        let total_fitness: IndividualT::FitnessValueT = population.iter().map(|indiv| indiv.fitness()).sum();
        let count = self.selection_size.get(metrics);

        let mut selected: Vec<&IndividualT> = Vec::with_capacity(count);

        for _ in 0..count {
            let threshold = total_fitness * self.rng.gen::<IndividualT::FitnessValueT>();

            let mut crt_sum: IndividualT::FitnessValueT = IndividualT::FitnessValueT::zero();
            for indiv in population {
                crt_sum += indiv.fitness();

                if crt_sum >= threshold {
                    selected.push(indiv);
                    break;
                }
            }
        }
        selected
    }
}

/// ### Random selection operator
///
/// This struct implements [SelectionOperator] trait and can be used with GA.
///
/// Individuals are selected with uniform probability.
///
/// **Note**: The same individual *can* be selected mutiple times.
pub struct Random<SizeValue: ValueProvider<usize>, R: Rng + Clone = ThreadRng> {
    selection_size: SizeValue,
    rng: R,
}

impl<SizeValue: ValueProvider<usize>> Random<SizeValue, ThreadRng> {
    /// Returns new instance of [Random] selection operator with default RNG
    ///
    /// ## Arguments
    ///
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    pub fn new(selection_size: SizeValue) -> Self {
        Random::with_rng(selection_size, rand::thread_rng())
    }
}

impl<SizeValue: ValueProvider<usize>, R: Rng + Clone> Random<SizeValue, R> {
    /// Returns new instance of [Random] selection operator with custom RNG
    ///
    /// ## Arguments
    ///
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    pub fn with_rng(selection_size: SizeValue, rng: R) -> Self {
        Random { selection_size, rng }
    }
}

impl<IndividualT: IndividualTrait, SizeValue: ValueProvider<usize>, R: Rng + Clone>
    SelectionOperator<IndividualT> for Random<SizeValue, R>
{
    /// Returns a vector of references to individuals selected to mating pool.
    ///
    /// Individuals are selected with uniform probability.
    ///
    /// ### Arguments
    ///
    /// * `metrics` - [crate::ga::Metrics] information on current stage of the algorithm (iteration, elapsed time, etc.)
    /// * `population` - individuals to choose mating pool from
    fn apply<'a>(&mut self, metrics: &Metrics, population: &'a [IndividualT]) -> Vec<&'a IndividualT> {
        let count = self.selection_size.get(metrics);
        let distr_ind = Uniform::new(0, population.len());
        let selection_iter = self
            .rng
            .clone()
            .sample_iter(distr_ind)
            .take(count)
            .map(|i| &population[i]);
        Vec::<&'a IndividualT>::from_iter(selection_iter)

        // We must use index API, as we want to return vector of references, not vector of actual items
        // let indices = rand::seq::index::sample(&mut self.rng, population.len(), count);
        // let mut selected: Vec<&IndividualT> = Vec::with_capacity(count);
        //
        // for i in indices {
        //     selected.push(&population[i]);
        // }
        // selected
    }
}

/// ### UniqueRandom selection operator
///
/// This struct implements [SelectionOperator] trait and can be used with GA.
///
/// Individuals are selected with uniform probability.
///
/// **Note**: The same individual *can not* be selected mutiple times.
pub struct UniqueRandom<SizeValue: ValueProvider<usize>, R: Rng = ThreadRng> {
    selection_size: SizeValue,
    rng: R,
}

impl<SizeValue: ValueProvider<usize>> UniqueRandom<SizeValue, ThreadRng> {
    pub fn new(selection_size: SizeValue) -> Self {
        Self::with_rng(selection_size, rand::thread_rng())
    }
}

impl<SizeValue: ValueProvider<usize>, R: Rng> UniqueRandom<SizeValue, R> {
    pub fn with_rng(selection_size: SizeValue, rng: R) -> Self {
        Self { selection_size, rng }
    }
}

impl<IndividualT: IndividualTrait, SizeValue: ValueProvider<usize>, R: Rng + Clone>
    SelectionOperator<IndividualT> for UniqueRandom<SizeValue, R>
{
    /// Returns a vector of references to individuals selected to mating pool.
    ///
    /// Individuals are selected with uniform probability.
    ///
    /// **Note**: The same individual *can not* be selected multiple times.
    /// **Note**: Selection size must not be greater than population size, in such case
    /// this operator panics.
    ///
    /// ### Arguments
    ///
    /// * `metrics` - [crate::ga::Metrics] information on current stage of the algorithm (iteration, elapsed time, etc.)
    /// * `population` - individuals to choose mating pool from
    ///
    /// ### Panics
    ///
    /// When selection size is greater than population size.
    fn apply<'a>(&mut self, metrics: &Metrics, population: &'a [IndividualT]) -> Vec<&'a IndividualT> {
        let count = self.selection_size.get(metrics);

        // We must use index API, as we want to return vector of references, not vector of actual items
        let indices = rand::seq::index::sample(&mut self.rng, population.len(), count);
        let mut selected: Vec<&IndividualT> = Vec::with_capacity(count);

        for i in indices {
            selected.push(&population[i]);
        }
        selected
    }
}

/// ### Rank selection operator
///
/// This struct implements [SelectionOperator] trait and can be used with GA.
///
/// Individuals are selected by randomly (uniform distribution) choosing pairs of individuals - better
/// rated individual from selected pair goes to mating pool. In case of equal fitness - only one goes to mating pool.
///
/// **Note**: The same individual *can* be selected multiple times.
pub struct Rank<SizeValue: ValueProvider<usize>, R: Rng = ThreadRng> {
    selection_size: SizeValue,
    rng: R,
}

impl<SizeValue: ValueProvider<usize>> Rank<SizeValue, ThreadRng> {
    /// Returns new instance of [Rank] selection operator with default RNG
    ///
    /// ## Arguments
    ///
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    pub fn new(selection_size: SizeValue) -> Self {
        Rank::with_rng(selection_size, rand::thread_rng())
    }
}

impl<SizeValue: ValueProvider<usize>, R: Rng> Rank<SizeValue, R> {
    /// Returns new instance of [Rank] selection operator with custom RNG
    ///
    /// ## Arguments
    ///
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    pub fn with_rng(selection_size: SizeValue, rng: R) -> Self {
        Rank { selection_size, rng }
    }
}

impl<IndividualT: IndividualTrait, SizeValue: ValueProvider<usize>, R: Rng> SelectionOperator<IndividualT>
    for Rank<SizeValue, R>
where
    IndividualT::FitnessValueT: PartialOrd,
{
    /// Returns a vector of references to individuals selected to mating pool.
    ///
    /// Individuals are selected by randomly (uniform distribution) choosing pairs of individuals - better
    /// rated individual from selected pair goes to mating pool. In case of equal fitness - only one goes to mating pool.
    ///
    /// **Note**: The same individual *can* be selected multiple times.
    ///
    /// ### Arguments
    ///
    /// * `metrics` - [crate::ga::Metrics] information on current stage of the algorithm (iteration, elapsed time, etc.)
    /// * `population` - individuals to choose mating pool from
    fn apply<'a>(&mut self, metrics: &Metrics, population: &'a [IndividualT]) -> Vec<&'a IndividualT> {
        let count = self.selection_size.get(metrics);
        let mut selected: Vec<&IndividualT> = Vec::with_capacity(count);

        let population_len = population.len();
        for _ in 0..count {
            // TODO: Consider creating two random index permutations and then iterating over them
            // instead of N times using random.
            let p1 = &population[self.rng.gen_range(0..population_len)];
            let p2 = &population[self.rng.gen_range(0..population_len)];

            selected.push(if p1.fitness() >= p2.fitness() { p1 } else { p2 })
        }

        selected
    }
}

/// ### RankR selection operator
///
/// This struct implements [SelectionOperator] trait and can be used with GA
///
/// Individuals are selected in following process:
///
/// 1. Select two random individuals (uniform distribution)
/// 2. Select random number `R` from [0, 1] (uniform distribution)
/// 3. If `R` < `r` then select first individual, second otherwise
/// 4. Repeat 1-3 necessary number of times to create mating pool of demanded size
///
/// **Note**: The same individual can be selected multiple times
pub struct RankR<SizeValue: ValueProvider<usize>, R: Rng = ThreadRng> {
    r: f64,
    selection_size: SizeValue,
    rng: R,
}

impl<SizeValue: ValueProvider<usize>> RankR<SizeValue, ThreadRng> {
    /// Returns new instance of [RankR] selection operator with default RNG
    ///
    /// ### Arguments
    ///
    /// * `r` - threshold in range [0, 1]; see [RankR] description for explaination
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    pub fn new(r: f64, selection_size: SizeValue) -> Self {
        RankR::with_rng(r, selection_size, rand::thread_rng())
    }
}

impl<SizeValue: ValueProvider<usize>, R: Rng> RankR<SizeValue, R> {
    /// Returns new instance of [RankR] selection operator with custom RNG
    ///
    /// ### Arguments
    ///
    /// * `r` - threshold in range [0, 1]; see [RankR] description for details
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    /// * `rng` - custom random number generator
    pub fn with_rng(r: f64, selection_size: SizeValue, rng: R) -> Self {
        assert!((0.0..=1.0).contains(&r));
        RankR {
            r,
            selection_size,
            rng,
        }
    }
}

impl<IndividualT: IndividualTrait, SizeValue: ValueProvider<usize>, R: Rng> SelectionOperator<IndividualT>
    for RankR<SizeValue, R>
{
    /// Returns a vector of references to individuals selected to mating pool.
    ///
    /// Individuals are selected in following process:
    ///
    /// 1. Select two random individuals (uniform distribution)
    /// 2. Select random number `R` from [0, 1] (uniform distribution)
    /// 3. If `R` < `r` then select first individual, second otherwise
    /// 4. Repeat 1-3 necessary number of times to create mating pool of demanded size
    ///
    /// **Note**: The same individual can be selected multiple times
    ///
    /// ### Arguments
    ///
    /// * `metrics` - [crate::ga::Metrics] information on current stage of the algorithm (iteration, elapsed time, etc.)
    /// * `population` - individuals to choose mating pool from
    fn apply<'a>(&mut self, metrics: &Metrics, population: &'a [IndividualT]) -> Vec<&'a IndividualT> {
        let count = self.selection_size.get(metrics);
        let mut selected: Vec<&IndividualT> = Vec::with_capacity(count);
        let population_len = population.len();
        let distribution_for_ind = rand::distributions::Uniform::from(0..population_len);
        let distribution_for_rand = rand::distributions::Uniform::from(0.0..1.0);

        for _ in 0..count {
            // TODO: Consider creating two random index permutations and then iterating over them
            // instead of N times using random.
            let p1 = &population[self.rng.sample(distribution_for_ind)];
            let p2 = &population[self.rng.sample(distribution_for_ind)];

            selected.push(if self.rng.sample(distribution_for_rand) < self.r {
                p1
            } else {
                p2
            })
        }
        selected
    }
}

/// ### Tournament selection operator
///
/// This struct implements [SelectionOperator] and can be used with GA
///
/// Individuals are selected by conducting given number of tournaments with single winner:
///
/// *Note*: The same individual can be selected multiple times
///
/// 1. Select `ceil(size_factor * population_size)` distinct, random individuals
/// 2. Select one with the highest fitness
/// 3. Repeat 1-2 number of times necessary to fill mating pool
pub struct Tournament<SizeValue: ValueProvider<usize>, R: Rng = ThreadRng> {
    size_factor: f64,
    selection_size: SizeValue,
    rng: R,
}

impl<SizeValue: ValueProvider<usize>> Tournament<SizeValue, ThreadRng> {
    /// Returns new instance of [Tournament] selection operator with default RNG
    ///
    /// ### Arguments
    ///
    /// * `size_factor` - part of population to take part in tournament for choosing single individual; must be in range [0, 1]
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    pub fn new(size_factor: f64, selection_size: SizeValue) -> Self {
        Self::with_rng(size_factor, selection_size, rand::thread_rng())
    }
}

impl<SizeValue: ValueProvider<usize>, R: Rng> Tournament<SizeValue, R> {
    /// Returns new instance of [Tournament] selection operator with custom RNG
    ///
    /// ### Arguments
    ///
    /// * `size_factor` - part of population to take part in tournament for choosing single individual; must be in range [0, 1]
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    pub fn with_rng(size_factor: f64, selection_size: SizeValue, rng: R) -> Self {
        assert!((0.0..=1.0).contains(&size_factor));
        Tournament {
            size_factor,
            selection_size,
            rng,
        }
    }
}

impl<IndividualT: IndividualTrait, SizeValue: ValueProvider<usize>, R: Rng> SelectionOperator<IndividualT>
    for Tournament<SizeValue, R>
{
    /// Returns a vector of references to individuals selected to mating pool
    ///
    /// Individuals are selected by conducting given number of tournaments with single winner:
    ///
    /// 1. Select `ceil(size_factor * population_size)` distinct, random individuals
    /// 2. Select one with the highest fitness
    /// 3. Repeat 1-2 number of times necessary to fill mating pool
    ///
    /// *Note*: The same individual can be selected multiple times
    ///
    /// ### Arguments
    ///
    /// * `metrics` - [crate::ga::Metrics] information on current stage of the algorithm (iteration, elapsed time, etc.)
    /// * `population` - individuals to choose mating pool from
    /// * `count` - target number of individuals in mating pool
    fn apply<'a>(&mut self, metrics: &Metrics, population: &'a [IndividualT]) -> Vec<&'a IndividualT> {
        let count = self.selection_size.get(metrics);
        let tournament_size = (population.len() as f64 * self.size_factor) as usize;
        let tournament_size = tournament_size.max(1);

        let mut selected: Vec<&IndividualT> = Vec::with_capacity(count);

        for _ in 0..count {
            let tournament_indices =
                rand::seq::index::sample(&mut self.rng, population.len(), tournament_size);
            // FIXME: Check wheter the tournament_indices is empty or handle option below.
            let best_idv = tournament_indices
                .into_iter()
                .map(|i| &population[i])
                .max()
                .unwrap();
            selected.push(best_idv);
        }

        selected
    }
}

/// ### Stochastic universal sampling selection operator
///
/// This struct implements [SelectionOperator] trati and can be used with GA
///
/// **Note**: This selection operator requires positive fitenss function. No runtime checks
/// are performed to assert this invariant. If aggregated fitenss in whole population is <= the
/// behaviour is undefined, implementation dependent and might change without any notice.
///
/// **Note**: The same individual can be selected multiple times
///
/// Individuals are selected in process similar to described below:
///
/// 1. Individuals are laid on real axis, in order they appear in population,
/// to interval \[0, `total_fitness`\]; each individual is represented by sub
/// interval of lengths equal to its fitness
/// 2. `count` virtual pointers are placed along interval \[0, `total_fitness`\];
/// distance between pointers `d` is `total_fitness` / `mating_pool_size`;
/// first pointer position is selected randomly from interval \[0, `d`\]
/// 3. Iterate over the pointers and select the individuals they point to
///
/// See the source code for implemenation details
pub struct StochasticUniversalSampling<SizeValue: ValueProvider<usize>, R: Rng = ThreadRng> {
    selection_size: SizeValue,
    rng: R,
}

impl<SizeValue: ValueProvider<usize>> StochasticUniversalSampling<SizeValue, ThreadRng> {
    /// Returns new instance of [StochasticUniversalSampling] selection operator with default RNG
    ///
    /// ## Arguments
    ///
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    pub fn new(selection_size: SizeValue) -> Self {
        Self::with_rng(selection_size, rand::thread_rng())
    }
}

impl<SizeValue: ValueProvider<usize>, R: Rng> StochasticUniversalSampling<SizeValue, R> {
    /// Returns new instance of [StochasticUniversalSampling] selection operator with custom RNG
    ///
    /// ## Arguments
    ///
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    pub fn with_rng(selection_size: SizeValue, rng: R) -> Self {
        Self { selection_size, rng }
    }
}

// FIXME: Panics then total_fitness == 0
// Should this be expected or do we want to handle this?
impl<IndividualT: IndividualTrait<FitnessValueT = f64>, SizeValue: ValueProvider<usize>, R: Rng>
    SelectionOperator<IndividualT> for StochasticUniversalSampling<SizeValue, R>
{
    /// Returns a vector of references to individuals selected to mating pool
    ///
    /// **Note**: This selection operator requires positive fitenss function. No runtime checks
    /// are performed to assert this invariant. If aggregated fitenss in whole population is <= the
    /// behaviour is undefined, implementation dependent and might change without any notice.
    ///
    /// **Note**: The same individual can be selected multiple times
    ///
    /// Individuals are selected in process similar to described below:
    ///
    /// 1. Individuals are laid on real axis, in order they appear in population,
    /// to interval \[0, `total_fitness`\]; each individual is represented by sub
    /// interval of lengths equal to its fitness
    /// 2. `count` virtual pointers are placed along interval \[0, `total_fitness`\];
    /// distance between pointers `d` is `total_fitness` / `mating_pool_size`;
    /// first pointer position is selected randomly from interval \[0, `d`\]
    /// 3. Iterate over the pointers and select the individuals they point to
    ///
    /// See the source code for implemenation details
    ///
    /// ### Arguments
    ///
    /// * `metrics` - [crate::ga::Metrics] information on current stage of the algorithm (iteration, elapsed time, etc.)
    /// * `population` - individuals to choose mating pool from
    fn apply<'a>(&mut self, metrics: &Metrics, population: &'a [IndividualT]) -> Vec<&'a IndividualT> {
        let count = self.selection_size.get(metrics);
        let total_fitness: f64 = population.iter().map(|indiv| indiv.fitness()).sum();

        let mut selected: Vec<&IndividualT> = Vec::with_capacity(count);

        let distance_between_pointers = total_fitness / (count as f64);

        assert!(distance_between_pointers > 0.0);

        let mut pointer_pos = self.rng.gen_range(0.0..=distance_between_pointers);

        let mut curr_sum = 0.0;
        for idv in population {
            curr_sum += idv.fitness();

            while curr_sum >= pointer_pos {
                selected.push(idv);
                pointer_pos += distance_between_pointers;
            }
        }

        assert_eq!(selected.len(), count);

        selected
    }
}

/// ### Boltzmann selection operator
///
/// This struct implements [SelectionOperator] trait and can be used with GA
///
pub struct Boltzmann<SizeValue: ValueProvider<usize>, R: Rng = ThreadRng> {
    selection_size: SizeValue,
    alpha: f64,
    max_gen_count: usize, // FIXME: This should be removed after operators are passed whole algorithm state & config
    temp_0: f64,
    elitism: bool, // FIXME: Make use of elitism strategy
    rng: R,
}

impl<SizeValue: ValueProvider<usize>> Boltzmann<SizeValue, ThreadRng> {
    /// Returns new instance of [Boltzmann] selection operator with default RNG
    ///
    /// ### Arguments
    ///
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    /// * `alpha` - prameter that controlls temperature scaling; must be in [0, 1] range
    /// * `temp_0` - initial temperature for the operator
    /// * `max_gen_count` - maximum number of generations GA can run; this param will be removed in future version of the library
    /// * `elitism` - set to true to ensure that best individuals end in mating pool no matter operator results; **not supported yet**
    pub fn new(
        selection_size: SizeValue,
        alpha: f64,
        temp_0: f64,
        max_gen_count: usize,
        elitism: bool,
    ) -> Self {
        Self::with_rng(
            selection_size,
            alpha,
            temp_0,
            max_gen_count,
            elitism,
            rand::thread_rng(),
        )
    }
}

impl<SizeValue: ValueProvider<usize>, R: Rng> Boltzmann<SizeValue, R> {
    /// Returns new instance of [Boltzmann] selection operator with default RNG
    ///
    /// ### Arguments
    ///
    /// * `selection_size` - value provider deciding how many individuals will selection operator
    /// produce
    /// * `alpha` - prameter that controlls temperature scaling; must be in [0, 1] range
    /// * `temp_0` - initial temperature for the operator
    /// * `max_gen_count` - maximum number of generations GA can run; this param will be removed in future version of the library
    /// * `elitism` - set to true to ensure that best individuals end in mating pool no matter operator results; **not supported yet**
    /// * `rng` - custom random number generator
    pub fn with_rng(
        selection_size: SizeValue,
        alpha: f64,
        temp_0: f64,
        max_gen_count: usize,
        elitism: bool,
        rng: R,
    ) -> Self {
        assert!(
            (0.0..=1.0).contains(&alpha),
            "Alpha parameter must be a value from [0, 1] interval"
        );
        assert!(
            (5.0..=100.0).contains(&temp_0),
            "Starting temperature must be a value from [5, 100] interval"
        );

        Boltzmann {
            selection_size,
            alpha,
            max_gen_count,
            temp_0,
            elitism,
            rng,
        }
    }
}

impl<IndividualT, SizeValue, R> SelectionOperator<IndividualT> for Boltzmann<SizeValue, R>
where
    IndividualT: IndividualTrait<FitnessValueT = f64>,
    IndividualT::ChromosomeT: Index<usize, Output = IndividualT::FitnessValueT>,
    SizeValue: ValueProvider<usize>,
    R: Rng,
{
    fn apply<'a>(&mut self, metrics: &Metrics, population: &'a [IndividualT]) -> Vec<&'a IndividualT> {
        let count = self.selection_size.get(metrics);
        let mut selected: Vec<&IndividualT> = Vec::with_capacity(count);
        let mut weights: Vec<f64> = Vec::with_capacity(count);

        let k = 1.0 + 100.0 * (metrics.generation as f64) / (self.max_gen_count as f64);
        let temp = self.temp_0 * (1.0 - self.alpha).powf(k);

        for idv in population {
            weights.push((-idv.fitness() / temp).exp())
        }

        let Ok(indices) =
            rand::seq::index::sample_weighted(&mut self.rng, population.len(), |i| weights[i], count)
        else {
            panic!("Some error occured while generating indices. This is most likely an library implementation error. Please file an issue: https://github.com/kkafar/evolutionary-algorithms");
        };

        for i in indices {
            selected.push(&population[i]);
        }

        selected
    }
}

#[cfg(test)]
mod test {
    use super::{Boltzmann, RankR, Tournament};

    #[test]
    #[should_panic]
    fn boltzman_panics_on_too_big_alpha() {
        let _operator = Boltzmann::new(100, 5.0, 10.0, 300, false);
    }

    #[test]
    #[should_panic]
    fn boltzman_panics_on_too_small_alpha() {
        let _operator = Boltzmann::new(100, -0.1, 10.0, 300, false);
    }

    #[test]
    #[should_panic]
    fn boltzman_panics_on_too_low_temp() {
        let _operator = Boltzmann::new(100, 0.5, 4.0, 300, false);
    }

    #[test]
    #[should_panic]
    fn boltzman_panics_on_too_high_temp() {
        let _operator = Boltzmann::new(100, 0.5, 400.0, 300, false);
    }

    #[test]
    #[should_panic]
    fn tournament_panics_on_wrong_size_factor() {
        let _operator = Tournament::new(2.0, 100);
    }

    #[test]
    #[should_panic]
    fn rankr_panics_on_wrong_r() {
        let _operator = RankR::new(1.1, 100);
    }
}
