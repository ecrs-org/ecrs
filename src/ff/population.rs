use crate::ff::FireflyAlgorithmCfg;
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};
use std::ops::{Index, IndexMut};

pub struct Population {
  fireflies: Vec<Vec<f64>>,
}

impl Index<usize> for Population {
  type Output = Vec<f64>;

  fn index(&self, index: usize) -> &Self::Output {
    &self.fireflies[index]
  }
}

impl IndexMut<usize> for Population {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.fireflies[index]
  }
}

impl Population {
  pub fn new() -> Self {
    Population {
      fireflies: Vec::new(),
    }
  }

  pub fn from_config<T>(config: FireflyAlgorithmCfg<T>) -> Self
  where
    T: Distribution<f64>,
  {
    let mut res: Vec<Vec<f64>> = Vec::new();
    for _index in 0..config.population_size as usize {
      let mut temp: Vec<f64> = Vec::new();
      for _dim in 0..config.dimensions {
        temp.push(thread_rng().gen_range(config.lower_bound..config.upper_bound));
      }
      res.push(temp);
    }
    Population { fireflies: res }
  }

  pub fn from_vector(vector: &[Vec<f64>]) -> Self {
    Population {
      fireflies: vector.to_owned(),
    }
  }
}
