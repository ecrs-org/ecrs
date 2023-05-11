mod problem;
mod util;

#[allow(unused_imports)]
use ecrs::prelude::{crossover, ga, ops, replacement, selection};
use ecrs::{
    ga::{GAMetadata, Individual},
    prelude::{crossover::CrossoverOperator, replacement::ReplacementOperator, selection::SelectionOperator},
};

use crate::problem::{state::JsspState, JsspConfig};

fn run() -> () {
    const POPULATION_SIZE: usize = 4;
    const SELECTION_SIZE: usize = 2;

    let mut state = JsspState {
        cfg: JsspConfig {
            n_jobs: 4,
            n_machines: 2,
        },
        population: Vec::new(),
    };

    // Generate initial population
    state.init_pop(POPULATION_SIZE);

    // Evaluate population
    state.eval_pop();

    // For bounded number of iterations run evolution:
    // 1. Select with elitism
    // 2. Uniform crossover or chromosomes (not decoded solutions)
    // 3. Instead of mutation

    let mut selection_op = ga::operators::selection::Rank::new();
    let mut crossover_op = ga::operators::crossover::Uniform::new();
    let replacement_op = ga::operators::replacement::BothParents::new();
    // assert!(replacement_op.requires_children_fitness() == false);

    let stub_metadata = GAMetadata::new(None, None, 0);

    for _ in 0..1 {
        let mut ecrs_individuals: Vec<Individual<Vec<f64>>> = state
            .population
            .iter()
            .map(|jssp_idv| Individual {
                chromosome: jssp_idv.chromosome.clone(),
                fitness: jssp_idv.fitness as f64,
            })
            .collect();

        let selected_pop = selection_op.apply(&stub_metadata, &ecrs_individuals, SELECTION_SIZE);

        let mut children: Vec<Individual<Vec<f64>>> = Vec::with_capacity(SELECTION_SIZE);

        for i in (0..selected_pop.len()).step_by(2) {
            let crt_children = crossover_op.apply(selected_pop[i], selected_pop[i + 1]);
            children.push(crt_children.0);
            children.push(crt_children.1);
        }

        // TODO: Sample new individuals from the initial distribution

        ecrs_individuals = replacement_op.apply(ecrs_individuals, children);
        state.inject_ecrs_pop(ecrs_individuals);
        state.eval_pop();
    }
}

fn main() -> Result<(), ()> {
    run();
    Ok(())
}
