#![cfg(feature = "ga")]

use ecrs::ga::individual::{IndividualTrait, RealValueIndividual};
use ecrs::ga::operators::crossover::Ppx;
use ecrs::ga::Metrics;
use ecrs::ga::{
    operators::crossover::{CrossoverOperator, MultiPoint, SinglePoint, TwoPoint, Uniform},
    population::{PopulationGenerator, RandomPoints},
    Individual,
};
use itertools::Itertools;

fn operator_takes_values_from_parents<T: CrossoverOperator<RealValueIndividual>>(mut operator: T) {
    let parents = RandomPoints::new(30).generate(2);
    assert_eq!(parents.len(), 2, "Expected population of size 2");

    let children = operator.apply(&Metrics::default(), &[&parents[0], &parents[1]]);
    let child_1 = &children[0];
    let child_2 = &children[1];
    for (i, (gene_1, gene_2)) in std::iter::zip(child_1.chromosome(), child_2.chromosome()).enumerate() {
        assert!(parents[0].chromosome()[i] == *gene_1 || parents[1].chromosome()[i] == *gene_1);
        assert!(parents[0].chromosome()[i] == *gene_2 || parents[1].chromosome()[i] == *gene_2);
    }
}

#[test]
fn single_point_takes_values_from_parents() {
    operator_takes_values_from_parents(SinglePoint::new());
}

#[test]
fn two_point_takes_values_from_parents() {
    operator_takes_values_from_parents(TwoPoint::new());
}

#[test]
fn multi_point_takes_values_from_parents() {
    for n in 4..=8 {
        operator_takes_values_from_parents(MultiPoint::new(n));
    }
}

#[test]
fn uniform_takes_values_from_parents() {
    operator_takes_values_from_parents(Uniform::new());
}

#[test]
// Can't think about a name for this test
fn ppx_test() {
    let mut op = Ppx::new();

    let p1 = Individual::from((0..10).collect_vec());
    let p2 = Individual::from((0..10).rev().collect_vec());
    let children = op.apply(&Metrics::default(), &[&p1, &p2]);
    let c1 = &children[0];
    let c2 = &children[1];

    c1.chromosome
        .iter()
        .zip(c2.chromosome.iter())
        .for_each(|(g1, g2)| assert_eq!(g1 + g2, 9));
}
