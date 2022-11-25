use ecrs::ga::{individual, Individual};

pub fn wordmax_fitness(individual: &Individual<Vec<bool>>) -> f64 {
	individual.chromosome_ref().into_iter().filter(|gene| **gene == true).count() as f64
}
