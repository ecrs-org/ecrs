use crate::ff::FireflyAlgorithmCfg;
use rand::distributions::Distribution;
use rand::{thread_rng, Rng};
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone)]
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

impl IntoIterator for Population {
    type Item = Vec<f64>;
    type IntoIter = <Vec<Vec<f64>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.fireflies.into_iter()
    }
}

impl Deref for Population {
    type Target = [Vec<f64>];

    fn deref(&self) -> &Self::Target {
        &self.fireflies[..]
    }
}

impl DerefMut for Population {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fireflies[..]
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
