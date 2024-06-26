#![cfg(feature = "ga")]

use ecrs::ga::{
    individual::{BitStringIndividual, RealValueIndividual},
    operators::selection::{
        Boltzmann, Random, Rank, RankR, RouletteWheel, SelectionOperator, StochasticUniversalSampling,
        Tournament,
    },
    population::{BitStrings, PopulationGenerator, RandomPoints},
    Metrics,
};

#[test]
fn random_selection_returns_demanded_size() {
    let expected_population_size: usize = 42;
    let population: Vec<BitStringIndividual> = BitStrings::new(21).generate(expected_population_size);

    assert_eq!(
        expected_population_size,
        population.len(),
        "Population generator must return population of expected size"
    );

    // FIXME: We must add mocking!
    let metrics = Metrics::default();

    let expected_selection_size = expected_population_size / 2;

    let selected = Random::new(expected_selection_size).apply(&metrics, &population);

    assert_eq!(
        expected_selection_size,
        selected.len(),
        "Selection operator must return population of expected size"
    );
}

#[test]
fn roulette_whell_returns_demanded_size() {
    let expected_population_size: usize = 42;
    let population: Vec<BitStringIndividual> = BitStrings::new(21).generate(expected_population_size);

    assert_eq!(
        expected_population_size,
        population.len(),
        "Population generator must return population of expected size"
    );

    // FIXME: We must add mocking!
    let metrics = Metrics::default();

    let expected_selection_size = expected_population_size / 2;

    let selected = RouletteWheel::new(expected_selection_size).apply(&metrics, &population);

    assert_eq!(
        expected_selection_size,
        selected.len(),
        "Selection operator must return population of expected size"
    );
}

#[test]
fn rank_returns_demanded_size() {
    let expected_population_size: usize = 42;
    let population: Vec<BitStringIndividual> = BitStrings::new(21).generate(expected_population_size);

    assert_eq!(
        expected_population_size,
        population.len(),
        "Population generator must return population of expected size"
    );

    // FIXME: We must add mocking!
    let metrics = Metrics::default();

    let expected_selection_size = expected_population_size / 2;

    let selected = Rank::new(expected_selection_size).apply(&metrics, &population);

    assert_eq!(
        expected_selection_size,
        selected.len(),
        "Selection operator must return population of expected size"
    );
}

#[test]
fn rankr_returns_demanded_size() {
    let expected_population_size: usize = 42;
    let population: Vec<BitStringIndividual> = BitStrings::new(21).generate(expected_population_size);

    assert_eq!(
        expected_population_size,
        population.len(),
        "Population generator must return population of expected size"
    );

    // FIXME: We must add mocking!
    let metrics = Metrics::default();

    let expected_selection_size = expected_population_size / 2;

    let selected = RankR::new(0.5, expected_selection_size).apply(&metrics, &population);

    assert_eq!(
        expected_selection_size,
        selected.len(),
        "Selection operator must return population of expected size"
    );
}

#[test]
fn tournament_returns_demanded_size() {
    let expected_population_size: usize = 42;
    let population: Vec<BitStringIndividual> = BitStrings::new(21).generate(expected_population_size);

    assert_eq!(
        expected_population_size,
        population.len(),
        "Population generator must return population of expected size"
    );

    // FIXME: We must add mocking!
    let metrics = Metrics::default();

    let expected_selection_size = expected_population_size / 2;

    let selected = Tournament::new(0.2, expected_selection_size).apply(&metrics, &population);

    assert_eq!(
        expected_selection_size,
        selected.len(),
        "Selection operator must return population of expected size"
    );
}

#[test]
fn sus_returns_demanded_size_when_fitness_positive() {
    let expected_population_size: usize = 42;
    let mut population: Vec<BitStringIndividual> = BitStrings::new(21).generate(expected_population_size);

    // SUS requires positive fitness
    for individual in &mut population {
        individual.fitness = 1.0;
    }

    assert_eq!(
        expected_population_size,
        population.len(),
        "Population generator must return population of expected size"
    );

    // FIXME: We must add mocking!
    let metrics = Metrics::default();

    let expected_selection_size = expected_population_size / 2;

    let selected = StochasticUniversalSampling::new(expected_selection_size).apply(&metrics, &population);

    assert_eq!(
        expected_selection_size,
        selected.len(),
        "Selection operator must return population of expected size"
    );
}

#[test]
fn boltzmann_returns_demanded_size() {
    let expected_population_size: usize = 42;
    let expected_selection_size = expected_population_size / 2;
    let dim = 21;

    let mut constraints: Vec<std::ops::Range<f64>> = Vec::with_capacity(dim);
    for _ in 0..dim {
        constraints.push(-1.0..1.0);
    }

    let population: Vec<RealValueIndividual> =
        RandomPoints::with_constraints(dim, constraints).generate(expected_population_size);

    assert_eq!(
        expected_population_size,
        population.len(),
        "Population generator must return population of expected size"
    );

    // FIXME: We must add mocking!
    let metrics = Metrics::new(
        Some(std::time::Instant::now()),
        None,
        40,
        expected_population_size,
    );

    let selected = Boltzmann::new(expected_selection_size, 0.2, 6.0, 300, true).apply(&metrics, &population);

    assert_eq!(
        expected_selection_size,
        selected.len(),
        "Selection operator must return population of expected size"
    );
}

#[test]
fn random_returns_whole_population_in_order() {
    let population_size = 42;
    let dim = 21;

    let population: Vec<RealValueIndividual> = RandomPoints::new(dim).generate(population_size);
    let mut operator = Random::with_rng(population_size, rand::rngs::mock::StepRng::new(0, 1));

    let selected = operator.apply(&Metrics::default(), &population);

    for (expected, actual) in std::iter::zip(&population, selected) {
        assert_eq!(expected, actual);
    }
}
