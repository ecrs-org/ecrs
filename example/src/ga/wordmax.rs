use ecrs::ga::{individual, Individual};

pub fn wordmax_fitness(individual: &Individual<Vec<bool>>) -> f64 {
  individual.chromosome_ref().iter().filter(|gene| **gene).count() as f64
}
