use std::ops::Index;

use rand::Rng;

use crate::ga::{
  individual::{Chromosome, Individual},
  GAMetadata,
};

pub trait SelectionOperator<T: Chromosome> {
  fn apply<'a>(
    &mut self,
    metadata: &GAMetadata,
    population: &'a [Individual<T>],
    count: usize,
  ) -> Vec<&'a Individual<T>>;
}

pub struct RouletteWheel;

impl RouletteWheel {
  pub fn new() -> Self {
    RouletteWheel {}
  }
}

impl<T: Chromosome> SelectionOperator<T> for RouletteWheel {
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

        if crt_sum > threshold {
          selected.push(indiv);
          break;
        }
      }
    }
    selected
  }
}

pub struct Random;

impl Random {
  pub fn new() -> Self {
    Random {}
  }
}

impl<T: Chromosome> SelectionOperator<T> for Random {
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

pub struct Rank;

impl Rank {
  pub fn new() -> Self {
    Rank {}
  }
}

impl<T: Chromosome> SelectionOperator<T> for Rank {
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

pub struct RankR {
  r: f64,
}

impl RankR {
  pub fn new(r: f64) -> Self {
    assert!((0.0..=1.0).contains(&r));
    RankR { r }
  }
}

impl<T: Chromosome> SelectionOperator<T> for RankR {
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

pub struct Tournament {
  size_factor: f64,
}

impl Tournament {
  pub fn new(size_factor: f64) -> Self {
    assert!((0.0..=1.0).contains(&size_factor));
    Tournament { size_factor }
  }
}

impl<T: Chromosome> SelectionOperator<T> for Tournament {
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

pub struct StochasticUniversalSampling;

impl StochasticUniversalSampling {
  pub fn new() -> Self {
    StochasticUniversalSampling {}
  }
}

impl<T: Chromosome> SelectionOperator<T> for StochasticUniversalSampling {
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

pub struct Boltzmann {
  alpha: f64,
  max_gen_count: usize, // FIXME: This should be removed after operators are passed whole algorithm state & config
  temp_0: f64,
  elitism: bool, // FIXME: Make use of elitism strategy
}

impl Boltzmann {
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
    use super::{Boltzmann, Tournament, RankR};

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
		let _operator = Boltzmann::new(0.5,  4.0, 300, false);
	}

	#[test]
	#[should_panic]
	fn boltzman_panics_on_too_high_temp() {
		let _operator = Boltzmann::new(0.5,  400.0, 300, false);
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
