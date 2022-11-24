use std::ops::Index;

use rand::Rng;

use crate::ga::{
  individual::{Chromosome, Individual},
  GAMetadata,
};

/// ### Selection operator
///
/// This trait defines common behaviour for selection operators.
/// You can implement this trait to provide your custom crossover opertator to the GA.
///
/// Following operators are implemented in the library:
///
/// * [RouletteWheel]
/// * [Random]
/// * [Rank]
/// * [RankR]
/// * [Tournament]
/// * [StochasticUniversalSampling]
/// * [Boltzmann]
///
/// See their respecitve docs for details.
pub trait SelectionOperator<T: Chromosome> {
  /// Returns a vector of references to individuals selected to mating pool
  ///
  /// ### Arguments
  ///
  /// * `metadata` - [crate::ga::GAMetadata] information on current stage of the algorithm (iteration, elapsed time, etc.)
  /// * `population` - individuals to choose mating pool from
  /// * `count` - target number of individuals in mating pool
  fn apply<'a>(
    &mut self,
    metadata: &GAMetadata,
    population: &'a [Individual<T>],
    count: usize,
  ) -> Vec<&'a Individual<T>>;
}

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
pub struct RouletteWheel;

impl RouletteWheel {
  /// Returns new instance of [RouletteWheel] selection operator
  pub fn new() -> Self {
    RouletteWheel {}
  }
}

// FIXME: It will return empty vector if total_fitness == 0
// WORKING CHANGE: crt >= threshold instead of crt_sum > threshold
// But this should be resolved some other way
impl<T: Chromosome> SelectionOperator<T> for RouletteWheel {
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
  /// * `metadata` - [crate::ga::GAMetadata] information on current stage of the algorithm (iteration, elapsed time, etc.)
  /// * `population` - individuals to choose mating pool from
  /// * `count` - target number of individuals in mating pool
  fn apply<'a>(
    &mut self,
    _metadata: &GAMetadata,
    population: &'a [Individual<T>],
    count: usize,
  ) -> Vec<&'a Individual<T>> {
    let total_fitness: f64 = population.iter().map(|indiv| indiv.fitness).sum();

    let mut selected: Vec<&Individual<T>> = Vec::with_capacity(count);

    for _ in 0..count {
      let threshold = total_fitness * rand::random::<f64>();

      let mut crt_sum = 0.0;
      for indiv in population {
        crt_sum += indiv.fitness;

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
/// **Note**: The same individual *can not* be selected mutiple times.
pub struct Random;

impl Random {
  /// Returns new instance of [Random] selection operator
  pub fn new() -> Self {
    Random {}
  }
}

impl<T: Chromosome> SelectionOperator<T> for Random {
  /// Returns a vector of references to individuals selected to mating pool.
  ///
  /// Individuals are selected with uniform probability.
  ///
  /// **Note**: The same individual *can not* be selected multiple times.
  ///
  /// ### Arguments
  ///
  /// * `metadata` - [crate::ga::GAMetadata] information on current stage of the algorithm (iteration, elapsed time, etc.)
  /// * `population` - individuals to choose mating pool from
  /// * `count` - target number of individuals in mating pool
  fn apply<'a>(
    &mut self,
    _metadata: &GAMetadata,
    population: &'a [Individual<T>],
    count: usize,
  ) -> Vec<&'a Individual<T>> {
    // We must use index API, as we want to return vector of references, not vector of actual items
    let indices = rand::seq::index::sample(&mut rand::thread_rng(), population.len(), count);
    let mut selected: Vec<&Individual<T>> = Vec::with_capacity(count);

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
pub struct Rank;

impl Rank {
  /// Returns new instance of [Rank] selection operator
  pub fn new() -> Self {
    Rank {}
  }
}

impl<T: Chromosome> SelectionOperator<T> for Rank {
  /// Returns a vector of references to individuals selected to mating pool.
  ///
  /// Individuals are selected by randomly (uniform distribution) choosing pairs of individuals - better
  /// rated individual from selected pair goes to mating pool. In case of equal fitness - only one goes to mating pool.
  ///
  /// **Note**: The same individual *can* be selected multiple times.
  ///
  /// ### Arguments
  ///
  /// * `metadata` - [crate::ga::GAMetadata] information on current stage of the algorithm (iteration, elapsed time, etc.)
  /// * `population` - individuals to choose mating pool from
  /// * `count` - target number of individuals in mating pool
  fn apply<'a>(
    &mut self,
    _metadata: &GAMetadata,
    population: &'a [Individual<T>],
    count: usize,
  ) -> Vec<&'a Individual<T>> {
    let mut selected: Vec<&Individual<T>> = Vec::with_capacity(count);

    let population_len = population.len();
    for _ in 0..count {
      // TODO: Consider creating two random index permutations and then iterating over them
      // instead of N times using random.
      let p1 = &population[rand::thread_rng().gen_range(0..population_len)];
      let p2 = &population[rand::thread_rng().gen_range(0..population_len)];

      selected.push(if p1.fitness >= p2.fitness { p1 } else { p2 })
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
pub struct RankR {
  r: f64,
}

impl RankR {
  /// Returns new instance of [RankR] selection operator
  ///
  /// ### Arguments
  ///
  /// * `r` - threshold in range [0, 1]; see [RankR] description for explaination
  pub fn new(r: f64) -> Self {
    assert!((0.0..=1.0).contains(&r));
    RankR { r }
  }
}

impl<T: Chromosome> SelectionOperator<T> for RankR {
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
  /// * `metadata` - [crate::ga::GAMetadata] information on current stage of the algorithm (iteration, elapsed time, etc.)
  /// * `population` - individuals to choose mating pool from
  /// * `count` - target number of individuals in mating pool
  fn apply<'a>(
    &mut self,
    _metadata: &GAMetadata,
    population: &'a [Individual<T>],
    count: usize,
  ) -> Vec<&'a Individual<T>> {
    let mut selected: Vec<&Individual<T>> = Vec::with_capacity(count);
    let population_len = population.len();
    let distribution_for_ind = rand::distributions::Uniform::from(0..population_len);
    let distribution_for_rand = rand::distributions::Uniform::from(0.0..1.0);

    for _ in 0..count {
      // TODO: Consider creating two random index permutations and then iterating over them
      // instead of N times using random.
      let p1 = &population[rand::thread_rng().sample(distribution_for_ind)];
      let p2 = &population[rand::thread_rng().sample(distribution_for_ind)];

      selected.push(if rand::thread_rng().sample(distribution_for_rand) < self.r {
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
pub struct Tournament {
  size_factor: f64,
}

impl Tournament {
  /// Returns new instance of [Tournament] selection operator
  ///
  /// ### Arguments
  ///
  /// * `size_factor` - part of population to take part in tournament for choosing single individual; must be in range [0, 1]
  pub fn new(size_factor: f64) -> Self {
    assert!((0.0..=1.0).contains(&size_factor));
    Tournament { size_factor }
  }
}

impl<T: Chromosome> SelectionOperator<T> for Tournament {
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
  /// * `metadata` - [crate::ga::GAMetadata] information on current stage of the algorithm (iteration, elapsed time, etc.)
  /// * `population` - individuals to choose mating pool from
  /// * `count` - target number of individuals in mating pool
  fn apply<'a>(
    &mut self,
    _metadata: &GAMetadata,
    population: &'a [Individual<T>],
    count: usize,
  ) -> Vec<&'a Individual<T>> {
    let tournament_size = (population.len() as f64 * self.size_factor) as usize;

    assert!(tournament_size > 0);

    let mut selected: Vec<&Individual<T>> = Vec::with_capacity(count);

    for _ in 0..count {
      let tournament_indices =
        rand::seq::index::sample(&mut rand::thread_rng(), population.len(), tournament_size);
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
pub struct StochasticUniversalSampling;

impl StochasticUniversalSampling {
	/// Returns new instance of [StochasticUniversalSampling] selection operator
  pub fn new() -> Self {
    StochasticUniversalSampling {}
  }
}

// FIXME: Panics then total_fitness == 0
// Should this be expected or do we want to handle this?
impl<T: Chromosome> SelectionOperator<T> for StochasticUniversalSampling {
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
  /// * `metadata` - [crate::ga::GAMetadata] information on current stage of the algorithm (iteration, elapsed time, etc.)
  /// * `population` - individuals to choose mating pool from
  /// * `count` - target number of individuals in mating pool
  fn apply<'a>(
    &mut self,
    _metadata: &GAMetadata,
    population: &'a [Individual<T>],
    count: usize,
  ) -> Vec<&'a Individual<T>> {
    let total_fitness: f64 = population.iter().map(|indiv| indiv.fitness).sum();

    let mut selected: Vec<&Individual<T>> = Vec::with_capacity(count);

    let distance_between_pointers = total_fitness / (count as f64);

    assert!(distance_between_pointers > 0.0);

    let mut pointer_pos = rand::thread_rng().gen_range(0.0..=distance_between_pointers);

    let mut curr_sum = 0.0;
    for idv in population {
      curr_sum += idv.fitness;

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
///
pub struct Boltzmann {
  alpha: f64,
  max_gen_count: usize, // FIXME: This should be removed after operators are passed whole algorithm state & config
  temp_0: f64,
  elitism: bool, // FIXME: Make use of elitism strategy
}

impl Boltzmann {
	/// Returns new instance of [Boltzmann] selection operator
	///
	/// ### Arguments
	///
	/// * `alpha` - prameter that controlls temperature scaling; must be in [0, 1] range
	/// * `temp_0` - initial temperature for the operator
	/// * `max_gen_count` - maximum number of generations GA can run; this param will be removed in future version of the library
	/// * `elitism` - set to true to ensure that best individuals end in mating pool no matter operator results
  pub fn new(alpha: f64, temp_0: f64, max_gen_count: usize, elitism: bool) -> Self {
    assert!(
      (0.0..=1.0).contains(&alpha),
      "Alpha parameter must be a value from [0, 1] interval"
    );
    assert!(
      (5.0..=100.0).contains(&temp_0),
      "Starting temperature must be a value from [5, 100] interval"
    );

    Boltzmann {
      alpha,
      max_gen_count,
      temp_0,
      elitism,
    }
  }
}

impl<T> SelectionOperator<T> for Boltzmann
where
  T: Chromosome + Index<usize, Output = f64>,
{
  fn apply<'a>(
    &mut self,
    metadata: &GAMetadata,
    population: &'a [Individual<T>],
    count: usize,
  ) -> Vec<&'a Individual<T>> {
    let mut selected: Vec<&Individual<T>> = Vec::with_capacity(count);
    let mut weights: Vec<f64> = Vec::with_capacity(count);

    let k = 1.0 + 100.0 * (metadata.generation.unwrap() as f64) / (self.max_gen_count as f64);
    let temp = self.temp_0 * (1.0 - self.alpha).powf(k);

    for idv in population {
      weights.push((-idv.fitness / temp).exp())
    }

    let Ok(indices) = rand::seq::index::sample_weighted(&mut rand::thread_rng(), population.len(), |i| weights[i], count) else {
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
    let _operator = Boltzmann::new(5.0, 10.0, 300, false);
  }

  #[test]
  #[should_panic]
  fn boltzman_panics_on_too_small_alpha() {
    let _operator = Boltzmann::new(-0.1, 10.0, 300, false);
  }

  #[test]
  #[should_panic]
  fn boltzman_panics_on_too_low_temp() {
    let _operator = Boltzmann::new(0.5, 4.0, 300, false);
  }

  #[test]
  #[should_panic]
  fn boltzman_panics_on_too_high_temp() {
    let _operator = Boltzmann::new(0.5, 400.0, 300, false);
  }

  #[test]
  #[should_panic]
  fn tournament_panics_on_wrong_size_factor() {
    let _operator = Tournament::new(2.0);
  }

  #[test]
  #[should_panic]
  fn rankr_panics_on_wrong_r() {
    let _operator = RankR::new(1.1);
  }
}
