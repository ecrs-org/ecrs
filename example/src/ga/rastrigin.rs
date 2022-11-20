use ecrs::ga;
use ga::Individual;
use itertools::Itertools;
use rand::Rng;

pub fn rastrigin_fitness(individual: &Individual<Vec<f64>>) -> f64 {
    5.0 * f64::exp(-rastrigin(individual.chromosome_ref()))
}

fn rastrigin(chromosome: &[f64]) -> f64 {
    10.0 * chromosome.len() as f64
        + chromosome.iter().fold(0.0, |sum, x| {
            sum + x.powi(2) - 10.0 * (2.0 * std::f64::consts::PI * x).cos()
        })
}

pub fn rastrigin_population_factory(population_size: usize) -> Vec<Individual<Vec<f64>>> {
    let mut population: Vec<Individual<Vec<f64>>> = Vec::with_capacity(population_size);

    let domain = rand::distributions::Uniform::from(-5.12..5.12);

    for _ in 0..population_size {
        let chromosome = rand::thread_rng()
            .sample_iter(domain)
            .take(3)
            .into_iter()
            .collect_vec();
        population.push(Individual {
            chromosome,
            fitness: f64::MAX,
        })
    }

    population
}
