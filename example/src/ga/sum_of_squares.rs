use super::point_generator;
use ecrs::ga;
use ga::Individual;

pub fn sum_of_squares_fitness(individual: &Individual<Vec<f64>>) -> f64 {
    10.0 * f64::exp(-sum_of_squares(individual.chromosome_ref()))
}

fn sum_of_squares(chromosome: &[f64]) -> f64 {
    chromosome.iter().map(|v| v * v).sum()
}

pub fn quadratic_population_factory(population_size: usize) -> Vec<Individual<Vec<f64>>> {
    let mut population: Vec<Individual<Vec<f64>>> = Vec::with_capacity(population_size);
    let restrictions = vec![(-2.0, 2.0), (-2.0, 2.0)];

    for _ in 0..population_size {
        let chromosome = point_generator(&restrictions);

        population.push(Individual {
            chromosome,
            fitness: f64::MAX,
        });
    }
    population
}
