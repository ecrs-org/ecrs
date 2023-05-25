#![allow(unused_imports)]
mod cli;
mod logging;
mod parse;
mod problem;
mod util;

use ecrs::prelude::{crossover, ga, ops, replacement, selection};
use ecrs::{
    ga::{GAMetadata, Individual, StdoutProbe},
    prelude::{
        crossover::{CrossoverOperator, UniformParameterized},
        mutation::{self, Identity},
        replacement::{BothParents, ReplacementOperator},
        selection::{Rank, SelectionOperator},
    },
};
use problem::fitness::JsspFitness;
use problem::individual::JsspIndividual;
use problem::population::JsspPopProvider;

use crate::problem::{state::JsspState, JsspConfig, JsspInstance};

fn run_with_ecrs() {
    let mut solver = ga::Builder::new::<
        JsspIndividual,
        Identity,
        UniformParameterized,
        Rank,
        BothParents,
        JsspPopProvider,
        JsspFitness,
        StdoutProbe,
    >()
    .set_selection_operator(selection::Rank::new())
    .set_crossover_operator(crossover::UniformParameterized::new(0.7))
    .set_mutation_operator(mutation::Identity::new())
    .set_replacement_operator(replacement::BothParents::new())
    .set_probe(ga::probe::StdoutProbe::new())
    .build();

    solver.run();
}

fn run() {
    if let Err(err) = logging::init_logging() {
        println!("Logger initialization returned following error");
        println!("{err}");
        return;
    }

    let args = cli::parse_args();

    if let Some(file) = args.file {
        let instance = JsspInstance::try_from(file).unwrap();
        for op in instance.jobs.iter() {
            println!("{op:?}");
        }
    }

    const POPULATION_SIZE: usize = 4;
    const SELECTION_SIZE: usize = 2;
    const GENERATION_COUNT: usize = 15;
    const ELITE_SIZE: usize = POPULATION_SIZE - SELECTION_SIZE;

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
    let mut best_fitness = state.eval_pop();
    println!("Best fitness: {best_fitness}");

    // For bounded number of iterations run evolution:
    // 1. Select with elitism
    // 2. Uniform crossover or chromosomes (not decoded solutions)
    // 3. Instead of mutation

    let mut selection_op = selection::Rank::new();
    // let mut crossover_op = crossover::Uniform::new();
    let mut crossover_op = crossover::UniformParameterized::new(0.7);
    let replacement_op = replacement::BothParents::new();

    let stub_metadata = GAMetadata::new(None, None, 0);

    for _ in 0..GENERATION_COUNT {
        let mut ecrs_individuals: Vec<Individual<Vec<f64>>> = state
            .population
            .iter()
            .map(|jssp_idv| Individual {
                chromosome: jssp_idv.chromosome.clone(),
                fitness: jssp_idv.fitness as f64,
            })
            .collect();

        let selected_pop = selection_op.apply(&stub_metadata, &ecrs_individuals, SELECTION_SIZE);

        let mut children: Vec<Individual<Vec<f64>>> = Vec::with_capacity(POPULATION_SIZE);

        for parents in selected_pop.chunks(2) {
            let crt_children = crossover_op.apply(parents[0], parents[1]);
            children.push(crt_children.0);
            children.push(crt_children.1);
        }

        if ELITE_SIZE > 0 {
            // ecrs_individuals.sort_by(|a , b| b.cmp(a));
            ecrs_individuals.sort();
            ecrs_individuals
                .iter()
                .take(ELITE_SIZE)
                .for_each(|idv| children.push(idv.clone()));
        }

        // TODO: Sample new individuals from the initial distribution
        assert!(children.len() == ecrs_individuals.len());

        ecrs_individuals = replacement_op.apply(ecrs_individuals, children);
        state.inject_ecrs_pop(ecrs_individuals);
        best_fitness = state.eval_pop();
        println!("Best fitness: {best_fitness}");
    }
}

fn main() -> Result<(), ()> {
    run();
    Ok(())
}
