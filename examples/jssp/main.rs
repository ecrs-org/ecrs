#![allow(unused_imports)]
mod cli;
mod logging;
mod parse;
mod problem;
mod util;

use std::path::PathBuf;

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
use log::info;
use problem::crossover::JsspCrossover;
use problem::fitness::JsspFitness;
use problem::individual::JsspIndividual;
use problem::population::JsspPopProvider;

use crate::problem::{JsspConfig, JsspInstance};

fn run_with_ecrs(instance: JsspInstance) {
    let pop_size = instance.cfg.n_ops * 2;

    let mut solver = ga::Builder::new()
        .set_selection_operator(selection::Rank::new())
        .set_crossover_operator(JsspCrossover::new())
        .set_mutation_operator(mutation::Identity::new())
        .set_replacement_operator(replacement::BothParents::new())
        .set_population_generator(JsspPopProvider::new(instance))
        .set_fitness(JsspFitness::new())
        .set_probe(ga::probe::StdoutProbe::new())
        .set_max_duration(std::time::Duration::from_secs(30))
        .set_max_generation_count(400)
        .set_population_size(pop_size)
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
            info!("{op:?}");
        }
        run_with_ecrs(instance);
    }
}

fn main() -> Result<(), ()> {
    run();
    Ok(())
}
