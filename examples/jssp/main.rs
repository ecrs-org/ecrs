#![allow(unused_imports)]
mod cli;
mod logging;
mod parse;
mod problem;
mod util;

use std::path::{Path, PathBuf};
use std::time::Duration;

use cli::Args;
use ecrs::ga::probe::{AggregatedProbe, ElapsedTime, PolicyDrivenProbe, ProbingPolicy};
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
use problem::probe::JsspProbe;
use problem::replacement::JsspReplacement;

use crate::problem::{JsspConfig, JsspInstance};

fn run_with_ecrs(instance: JsspInstance, _args: Args) {
    let pop_size = instance.cfg.n_ops * 2;

    let probe = AggregatedProbe::new()
        .add_probe(JsspProbe::new())
        .add_probe(PolicyDrivenProbe::new(
            ElapsedTime::new(Duration::from_millis(1000), Duration::from_millis(0)),
            StdoutProbe::new(),
        ));

    // Only for debugging purposes. TODO: Remove it
    let population_provider = JsspPopProvider::new(instance.clone());
    for op in population_provider.operations.iter() {
        info!("{op:?}");
    }

    let mut solver = ga::Builder::new()
        .set_selection_operator(selection::Rank::new())
        .set_crossover_operator(JsspCrossover::new())
        .set_mutation_operator(mutation::Identity::new())
        .set_population_generator(JsspPopProvider::new(instance.clone()))
        .set_replacement_operator(JsspReplacement::new(JsspPopProvider::new(instance), 0.1, 0.2))
        .set_fitness(JsspFitness::new())
        .set_probe(probe)
        // .set_max_duration(std::time::Duration::from_secs(30))
        .set_max_generation_count(400)
        .set_population_size(pop_size)
        .build();

    solver.run();
}

fn run() {
    let args = cli::parse_args();

    util::assert_dir_exists(args.output_dir.as_ref());
    let event_map = util::create_event_map(args.output_dir.as_ref());
    if let Err(err) = logging::init_logging(&event_map, &args.output_dir.join("run_metadata.json")) {
        panic!("Logger initialization failed with error: {err}");
    }

    // Existance of input file is asserted during cli args parsing
    let instance = JsspInstance::try_from(&args.input_file).unwrap();
    // for job in instance.jobs.iter() {
    //     for op in job {
    //         info!("{op:?}");
    //     }
    //     info!("\n")
    // }
    run_with_ecrs(instance, args)
}

fn main() -> Result<(), ()> {
    run();
    Ok(())
}
