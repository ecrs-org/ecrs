#![cfg(feature = "ga")]

use std::f64;

pub fn ackley2(x: &Vec<f64>) -> f64 {
    assert_eq!(
        2,
        x.len(),
        "Ackley 2nd function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powf(-200_f64 * f64::consts::E, -0.02) * f64::sqrt(f64::powi(x1, 2) + f64::powi(x2, 2))
}

#[allow(clippy::ptr_arg)]
pub fn wordmax(chromosome: &Vec<bool>) -> f64 {
    chromosome.iter().filter(|gene| **gene).count() as f64
}

use ecrs::{
    ga::{individual::RealValueIndividual, StdoutProbe},
    prelude::{
        crossover::SinglePoint, fitness::FnBasedFitness, mutation::Identity, population::RandomPoints,
        replacement::BothParents, selection::Boltzmann,
    },
};

#[test]
fn generic_does_not_panic_with_some_params_unspecified() {
    // selection_rate, mutation_rate omitted
    let _ = ecrs::ga::Builder::new::<
        RealValueIndividual,
        Identity,
        SinglePoint,
        Boltzmann,
        BothParents,
        RandomPoints,
        FnBasedFitness<RealValueIndividual>,
        StdoutProbe,
    >()
    .set_max_generation_count(500)
    .set_population_size(100)
    .set_fitness_fn(ackley2)
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
    let _ = ecrs::ga::Builder::new::<
        RealValueIndividual,
        Identity,
        SinglePoint,
        Boltzmann,
        BothParents,
        RandomPoints,
        FnBasedFitness<RealValueIndividual>,
        StdoutProbe,
    >()
    .set_fitness_fn(ackley2)
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
    let _ = ecrs::ga::Builder::with_rvc().dim(10).fitness_fn(ackley2).build();
}

#[test]
fn bsc_does_not_panic_with_some_operators_unsepcified() {
    let _ = ecrs::ga::Builder::with_bsc().dim(10).fitness_fn(wordmax).build();
}
