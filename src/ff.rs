use rand::{thread_rng, Rng};
use std::f64;

pub mod auxiliary;
pub mod probe;

use probe::Probe;

pub struct FireflyAlgorithmCfg {
  // Nr of dimensions
  dimensions: u8,
  // Lower search bound
  lower_bound: f64,
  // Upper search bound
  upper_bound: f64,
  // Maximum amount of generations
  max_generations: u32,
  // Population size
  population_size: u32,
  // Initial randomness coefficient
  alfa0: f64,
  // Attractiveness coefficient, in most cases leave as 1
  beta0: f64,
  // Light absorption coefficient
  gamma: f64,
  // Randomness decrease modifier, 0 < delta < 1
  delta: f64,
}

impl Default for FireflyAlgorithmCfg {
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
    }
  }
}

pub struct FireflyAlgorithm {
  pub config: FireflyAlgorithmCfg,
  pub brightness_function: fn(&Vec<f64>) -> f64,
  pub probe: Box<dyn Probe>,
}

impl FireflyAlgorithm {
  fn new(
    config: FireflyAlgorithmCfg,
    brightness_function: fn(&Vec<f64>) -> f64,
    probe: Box<dyn Probe>,
  ) -> Self {
    FireflyAlgorithm {
      config,
      brightness_function,
      probe,
    }
  }

  pub fn execute(&mut self) {
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
    let temp = population.clone();

    for point in temp {
      brightness.push(1_f64 / (self.brightness_function)(&point)); //TODO DELETE TEMP CLONEA
    }

    let scale = self.config.upper_bound - self.config.lower_bound;
    let mut alfa = self.config.alfa0;
    let mut rng = thread_rng();
    let mut currentbest: f64 = f64::MAX;

    for generation in 0..self.config.max_generations {
      self.probe.on_iteration_start(generation);

      for index in 0_usize..self.config.population_size as usize {
        for innerindex in 0_usize..self.config.population_size as usize {
          if brightness[index] < brightness[innerindex] {
            let const1 = self.config.beta0
              * f64::powf(
                f64::consts::E,
                -1_f64
                  * self.config.gamma
                  * f64::powi(distance(&population[index], &population[innerindex]), 2),
              );

            for dimension in 0_usize..self.config.dimensions as usize {
              population[index][dimension] += const1
                * (population[innerindex][dimension] - population[index][dimension])
                + self.config.alfa0 * alfa * (rng.gen_range(0.01..0.99)/*TODO ADD SETTING*/ - 0.5) * scale;
            }

            brightness[index] = 1_f64 / (self.brightness_function)(&population[index]);
          }
        }
      }

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

pub fn distance(a: &Vec<f64>, b: &[f64]) -> f64 {
  //Distance between two points
  let mut res: f64 = 0 as f64;
  for dimension in 0..a.len() {
    res += f64::powi(a[dimension] - b[dimension], 2)
  }
  f64::sqrt(res)
}
