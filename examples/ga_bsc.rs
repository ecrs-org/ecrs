use ecrs::ga;
mod util;

#[allow(clippy::ptr_arg)]
pub fn wordmax_fitness(chromosome: &Vec<bool>) -> f64 {
    chromosome.iter().filter(|gene| **gene).count() as f64
}

fn main() {
    let _ = util::init_logging();

    let best_individual = ga::Builder::with_bsc()
        .set_max_duration(std::time::Duration::from_secs(10))
        .fitness_fn(wordmax_fitness)
        .dim(10)
        .set_population_size(6)
        .build()
        .run()
        .unwrap();

    println!("Bitstring with most ones: {best_individual:#?}")
}

