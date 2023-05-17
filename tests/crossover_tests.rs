#![cfg(feature = "ga")]

use ecrs::ga::operators::crossover::Ppx;
use ecrs::ga::{
    operators::crossover::{CrossoverOperator, MultiPoint, SinglePoint, TwoPoint, Uniform},
    population::{PopulationGenerator, RandomPoints},
    ConcreteIndividual,
};
use itertools::Itertools;

fn operator_takes_values_from_parents<T: CrossoverOperator<Vec<f64>>>(mut operator: T) {
    let parents = RandomPoints::new(30).generate(2);
    assert_eq!(parents.len(), 2, "Expected population of size 2");

    let (child_1, child_2) = operator.apply(&parents[0], &parents[1]);
    for (i, (gene_1, gene_2)) in
        std::iter::zip(child_1.chromosome_ref(), child_2.chromosome_ref()).enumerate()
    {
        assert!(parents[0].chromosome_ref()[i] == *gene_1 || parents[1].chromosome_ref()[i] == *gene_1);
        assert!(parents[0].chromosome_ref()[i] == *gene_2 || parents[1].chromosome_ref()[i] == *gene_2);
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

    let p1 = ConcreteIndividual::from((0..10).collect_vec());
    let p2 = ConcreteIndividual::from((0..10).rev().collect_vec());
    let (c1, c2) = op.apply(&p1, &p2);

    c1.chromosome
        .iter()
        .zip(c2.chromosome.iter())
        .for_each(|(g1, g2)| assert_eq!(g1 + g2, 9));
}
