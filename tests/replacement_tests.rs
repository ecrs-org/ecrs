#![cfg(feature = "ga")]

use ecrs::ga::{
    operators::replacement::{BothParents, Noop, ReplacementOperator},
    population::{PopulationGenerator, RandomPoints},
    Individual, Metrics,
};

#[test]
fn noop_does_nothing() {
    let point_count = 40;
    let mut pop_gen = RandomPoints::new(4);

    let population: Vec<Individual<Vec<f64>>> = pop_gen.generate(point_count);
    let children = pop_gen.generate(point_count);

    let population_clone = population.clone();

    let mut noop = Noop::new();

    let result = noop.apply(&Metrics::default(), population, children);

    assert_eq!(result, population_clone);
}

#[test]
fn both_parents_returns_children() {
    let point_count = 40;

    let mut pop_gen = RandomPoints::new(4);

    let population: Vec<Individual<Vec<f64>>> = pop_gen.generate(point_count);
    let children = pop_gen.generate(point_count);

    let children_clone = children.clone();

    let mut both_parents = BothParents::new();

    let result = both_parents.apply(&Metrics::default(), population, children);

    assert_eq!(result, children_clone);
}
