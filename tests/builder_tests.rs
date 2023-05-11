#[test]
fn generic_does_not_panic_with_some_params_unspecified() {
    // selection_rate, mutation_rate omitted
    let _ = ecrs::ga::Builder::new()
        .set_max_generation_count(500)
        .set_population_size(100)
        .set_fitness_fn(ecrs::test_functions::rastrigin)
        .set_crossover_operator(ecrs::ga::operators::crossover::SinglePoint::new())
        .set_replacement_operator(ecrs::ga::operators::replacement::BothParents)
        .set_mutation_operator(ecrs::ga::operators::mutation::Identity::new())
        .set_population_generator(ecrs::ga::population::RandomPoints::with_constraints(
            3,
            vec![-5.12..5.12, -5.12..5.12, -5.12..5.12],
        ))
        .set_selection_operator(ecrs::ga::operators::selection::Boltzmann::new(
            0.05, 80.0, 500, false,
        ))
        .set_probe(ecrs::ga::probe::StdoutProbe)
        .build();

    // all params omitted
    let _ = ecrs::ga::Builder::new()
        .set_fitness_fn(ecrs::test_functions::rastrigin)
        .set_crossover_operator(ecrs::ga::operators::crossover::SinglePoint::new())
        .set_mutation_operator(ecrs::ga::operators::mutation::Identity::new())
        .set_replacement_operator(ecrs::ga::operators::replacement::BothParents)
        .set_population_generator(ecrs::ga::population::RandomPoints::with_constraints(
            3,
            vec![-5.12..5.12, -5.12..5.12, -5.12..5.12],
        ))
        .set_selection_operator(ecrs::ga::operators::selection::Boltzmann::new(
            0.05, 80.0, 500, false,
        ))
        .set_probe(ecrs::ga::probe::StdoutProbe)
        .build();
}

#[test]
fn rvc_does_not_panic_with_some_operators_unspecified() {
    let _ = ecrs::ga::Builder::with_rvc()
        .dim(10)
        .fitness_fn(ecrs::test_functions::ackley)
        .build();
}

#[test]
fn bsc_does_not_panic_with_some_operators_unsepcified() {
    let _ = ecrs::ga::Builder::with_bsc()
        .dim(10)
        .fitness_fn(ecrs::test_functions::wordmax)
        .build();
}
