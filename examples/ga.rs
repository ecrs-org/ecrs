use ecrs::{
    ga::{self, individual::RealValueIndividual, probe::AggregatedProbe},
    prelude::{
        crossover::SinglePoint, fitness::FnBasedFitness, mutation::Identity, population::RandomPoints,
        replacement::WeakParent, selection::Boltzmann,
    },
};

mod util;

#[allow(clippy::ptr_arg)]
fn rastrigin_fitness(chromosome: &Vec<f64>) -> f64 {
    1000.0 * f64::exp(-ecrs::test_functions::rastrigin(chromosome))
}

fn main() {
    let _ = util::init_logging();

    let res = ga::Builder::new::<
        RealValueIndividual,
        Identity,
        SinglePoint,
        Boltzmann<usize>,
        WeakParent,
        RandomPoints,
        FnBasedFitness<RealValueIndividual>,
        AggregatedProbe<RealValueIndividual>,
    >()
    .set_max_generation_count(50_000)
    .set_population_size(100)
    .set_fitness_fn(rastrigin_fitness)
    .set_crossover_operator(ga::operators::crossover::SinglePoint::new())
    .set_replacement_operator(ga::operators::replacement::WeakParent::new())
    .set_mutation_operator(ga::operators::mutation::Identity::new())
    .set_population_generator(ga::population::RandomPoints::with_constraints(
        3,
        vec![-5.12..5.12, -5.12..5.12, -5.12..5.12],
    ))
    .set_selection_operator(ga::operators::selection::Boltzmann::new(100, 0.05, 80.0, 500, false))
    .set_probe(
        ga::probe::AggregatedProbe::new()
            .add_probe(ga::probe::PolicyDrivenProbe::new(
                ga::probe::ElapsedTime::new(std::time::Duration::from_millis(200), std::time::Duration::ZERO),
                ga::probe::StdoutProbe,
            ))
            .add_probe(ga::probe::PolicyDrivenProbe::new(
                ga::probe::GenerationInterval::new(500, 100),
                ga::probe::StdoutProbe,
            )),
    )
    .build()
    .run()
    .unwrap();

    println!("{res:?}");
}
