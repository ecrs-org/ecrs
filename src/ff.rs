use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

pub mod auxiliary;
pub mod probe;

use probe::Probe;

use crate::ff::auxiliary::*;
use crate::ff::probe::stdout_probe::StdoutProbe;

pub struct FireflyAlgorithmCfg<T>
where
  T: Distribution<f64>,
{
  // Nr of dimensions
  pub dimensions: u8,
  // Lower search bound
  pub lower_bound: f64,
  // Upper search bound
  pub upper_bound: f64,
  // Maximum amount of generations
  pub max_generations: u32,
  // Population size
  pub population_size: u32,
  // Initial randomness coefficient
  pub alfa0: f64,
  // Attractiveness coefficient, in most cases leave as 1
  pub beta0: f64,
  // Light absorption coefficient
  pub gamma: f64,
  // Randomness decrease modifier, 0 < delta < 1
  pub delta: f64,
  //Number of threads in rayon worker pool, utilized to iterate the population
  pub threads: u8,
  //Probability distribution describing flight length
  pub distribution: T,
}

impl Default for FireflyAlgorithmCfg<Standard> {
  fn default() -> Self {
    FireflyAlgorithmCfg {
      dimensions: 2,
      lower_bound: -5.0,
      upper_bound: 5.0,
      max_generations: 1000,
      population_size: 25,
      alfa0: 1.0,
      beta0: 1.0,
      gamma: 0.01,
      delta: 0.97,
      threads: 2,
      distribution: Standard,
    }
  }
}

pub struct FireflyAlgorithm<T>
where
  T: Distribution<f64>,
{
  pub config: FireflyAlgorithmCfg<T>,
  pub brightness_function: fn(&Vec<f64>) -> f64,
  pub probe: Box<dyn Probe>,
  pub distance_function: fn(&Vec<f64>, &[f64]) -> f64,
}

impl<T> Default for FireflyAlgorithm<T>
where
  T: Distribution<f64>,
  FireflyAlgorithmCfg<T>: Default,
{
  fn default() -> Self {
    FireflyAlgorithm {
      config: Default::default(),
      brightness_function: rastrigin,
      probe: Box::new(StdoutProbe {}),
      distance_function: cartesian_distance,
    }
  }
}

impl<T> FireflyAlgorithm<T>
where
  T: Distribution<f64> + Sync,
{
  fn new(
    config: FireflyAlgorithmCfg<T>,
    brightness_function: fn(&Vec<f64>) -> f64,
    probe: Box<dyn Probe>,
    distance_function: fn(&Vec<f64>, &[f64]) -> f64,
  ) -> Self {
    FireflyAlgorithm {
      config,
      brightness_function,
      probe,
      distance_function,
    }
  }

  pub fn run(&mut self) {
    self.probe.on_start();
    let mut population: Vec<Vec<f64>> = Vec::new();

    for _index in 0..self.config.population_size as usize {
      //Generate initial population
      let mut temp: Vec<f64> = Vec::new();
      for _dim in 0..self.config.dimensions {
        temp.push(thread_rng().gen_range(self.config.lower_bound..self.config.upper_bound));
      }
      population.push(temp);
    }

    let mut brightness: Vec<f64> = Vec::new();

    for point in population.clone() {
      brightness.push(1_f64 / (self.brightness_function)(&point));
    }

    let update_brightness = |population: &Vec<Vec<f64>>| -> Vec<f64> {
      let mut res = vec![0 as f64; population.len()];
      for (dim, _ini) in population.iter().enumerate() {
        res[dim] = 1_f64 / (self.brightness_function)(&population[dim]);
      }
      res
    };

    let scale = self.config.upper_bound - self.config.lower_bound;
    let mut alfa = self.config.alfa0;
    let mut currentbest: f64 = f64::MAX;

    let pool = rayon::ThreadPoolBuilder::new()
      .num_threads(self.config.threads as usize)
      .build()
      .unwrap();

    let move_firefly = |index: usize,
                        local_brightness: Vec<f64>,
                        local_population: Vec<Vec<f64>>,
                        local_alfa: f64,
                        generation: f64|
     -> Vec<f64> {
      let mut res = local_population[index].clone();
      let mut did_i_move = false;
      for innerindex in 0_usize..self.config.population_size as usize {
        if local_brightness[index] < local_brightness[innerindex] {
          did_i_move = true;
          let const1 = self.config.beta0
            * f64::powf(
              std::f64::consts::E,
              -1_f64
                * self.config.gamma
                * f64::powi(
                  (self.distance_function)(&local_population[index], &local_population[innerindex]),
                  2,
                ),
            );
          let firefly = local_population[index].clone();
          for (dimension, _item) in firefly.iter().enumerate() {
            let step = const1
              * (local_population[innerindex][dimension] - local_population[index][dimension])
              + self.config.alfa0
                * local_alfa
                * (thread_rng().gen_range(0.01..0.99)
                  + self.config.distribution.sample(&mut thread_rng()) / generation
                  - 0.5)
                * scale;
            let _not_less_or_equal = matches!(
              (local_population[index][dimension] + step).partial_cmp(&self.config.lower_bound),
              None | Some(Ordering::Greater)
            );
            let _not_more_or_equal = matches!(
              (local_population[index][dimension] + step).partial_cmp(&self.config.upper_bound),
              None | Some(Ordering::Less)
            );
            if _not_more_or_equal && _not_less_or_equal {
              res[dimension] = local_population[index][dimension] + step;
            } else if local_population[index][dimension] + step > self.config.upper_bound {
              res[dimension] = self.config.upper_bound;
            } else {
              res[dimension] = self.config.lower_bound;
            }
          }
        }
      }
      if !did_i_move {
        let mut brownian = res.clone();
        for (dim, val) in res.clone().iter_mut().enumerate() {
          let step = thread_rng().gen_range(-1.0..1.0)
            * f64::powf(local_alfa, 2_f64)
            * f64::powf(
              std::f64::consts::E,
              -1_f64 * self.config.beta0 * self.config.gamma,
            );
          let _not_less_or_equal = matches!(
            (*val + step).partial_cmp(&self.config.lower_bound),
            None | Some(Ordering::Greater)
          );
          let _not_more_or_equal = matches!(
            (*val + step).partial_cmp(&self.config.upper_bound),
            None | Some(Ordering::Less)
          );
          if _not_more_or_equal && _not_less_or_equal {
            brownian[dim] = *val + step;
          } else if *val + step > self.config.upper_bound {
            brownian[dim] = self.config.upper_bound;
          } else {
            brownian[dim] = self.config.lower_bound;
          }
        }
        res = brownian;
      }
      res
    };

    for generation in 0..self.config.max_generations {
      self.probe.on_iteration_start(generation);
      let mut temp = population.clone();

      for (index, _item) in population.clone().iter_mut().enumerate() {
        temp[index] = pool.install(|| {
          move_firefly(
            index,
            brightness.clone(),
            population.clone(),
            alfa,
            generation as f64,
          )
        });
      }

      population = temp;

      brightness = update_brightness(&population);

      alfa *= self.config.delta;

      let mut maxpos = 0;
      let mut maxbright = 0 as f64;

      for (index, item) in brightness
        .iter()
        .enumerate()
        .take(self.config.population_size as usize)
      {
        if *item == f64::INFINITY {
          maxpos = index;
          break;
        }

        if *item > maxbright {
          maxbright = *item;
          maxpos = index;
        }
      }

      if (self.brightness_function)(&population[maxpos]) < currentbest {
        currentbest = (self.brightness_function)(&population[maxpos]);
      }

      self.probe.on_current_best(currentbest, &population[maxpos]);

      self.probe.on_iteration_end(generation);
    }

    self.probe.on_end();
  }
}
