#![allow(unused_imports)]
mod cli;
mod config;
mod logging;
mod parse;
mod problem;
mod util;

use std::path::{Path, PathBuf};
use std::time::Duration;

use cli::Args;
use config::Config;
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

fn run_randomsearch(instance: JsspInstance, config: Config) {
    info!("Running jssp solver with random search");

    let pop_size = if let Some(ps) = config.pop_size {
        // Overrided by user
        ps
    } else {
        // Defined in paper
        instance.cfg.n_ops * 2
    };

    let n_gen = if let Some(ng) = config.n_gen {
        // Overrided by user
        ng
    } else {
        // Defined in paper
        400
    };

    // let probe = AggregatedProbe::new()
    //     .add_probe(JsspProbe::new())
    //     .add_probe(PolicyDrivenProbe::new(
    //         ElapsedTime::new(Duration::from_millis(1000), Duration::from_millis(0)),
    //         StdoutProbe::new(),
    //     ));

    // Only for debugging purposes. TODO: Remove it
    // let population_provider = JsspPopProvider::new(instance.clone());
    // for op in population_provider.operations.iter() {
    //     info!("{op:?}");
    // }

    ga::Builder::new()
        .set_population_generator(JsspPopProvider::new(instance.clone()))
        .set_fitness(JsspFitness::new(1.5))
        .set_selection_operator(problem::selection::EmptySelection::new())
        .set_crossover_operator(problem::crossover::NoopCrossover::new())
        .set_mutation_operator(mutation::Identity::new())
        .set_replacement_operator(problem::replacement::ReplaceWithRandomPopulation::new(
            JsspPopProvider::new(instance),
        ))
        .set_probe(JsspProbe::new())
        .set_max_generation_count(n_gen)
        .set_population_size(pop_size)
        .build()
        .run();
}

fn run_jssp_solver(instance: JsspInstance, config: Config) {
    info!("Running JSSP solver");

    let pop_size = if let Some(ps) = config.pop_size {
        // Overrided by user
        ps
    } else {
        // Defined in paper
        instance.cfg.n_ops * 2
    };

    let n_gen = if let Some(ng) = config.n_gen {
        // Overrided by user
        ng
    } else {
        // Defined in paper
        400
    };

    // let probe = AggregatedProbe::new()
    //     .add_probe(JsspProbe::new())
    //     .add_probe(PolicyDrivenProbe::new(
    //         ElapsedTime::new(Duration::from_millis(1000), Duration::from_millis(0)),
    //         StdoutProbe::new(),
    //     ));

    // Only for debugging purposes. TODO: Remove it
    // let population_provider = JsspPopProvider::new(instance.clone());
    // for op in population_provider.operations.iter() {
    //     info!("{op:?}");
    // }

    ga::Builder::new()
        .set_selection_operator(selection::Rank::new())
        .set_crossover_operator(JsspCrossover::new())
        .set_mutation_operator(mutation::Identity::new())
        .set_population_generator(JsspPopProvider::new(instance.clone()))
        .set_replacement_operator(JsspReplacement::new(JsspPopProvider::new(instance), 0.1, 0.2))
        .set_fitness(JsspFitness::new(1.5))
        .set_probe(JsspProbe::new())
        // .set_max_duration(std::time::Duration::from_secs(30))
        .set_max_generation_count(n_gen)
        .set_population_size(pop_size)
        .build()
        .run();
}

fn run() {
    let args = cli::parse_args();
    let config = match Config::try_from(args) {
        Ok(config) => config,
        Err(err) => panic!("Failed to create config from args: {err}"),
    };

    util::assert_dir_exists(config.output_dir.as_ref());
    let event_map = util::create_event_map(config.output_dir.as_ref());

    if let Err(err) = logging::init_logging(&event_map, &config.output_dir.join("run_metadata.json")) {
        panic!("Logger initialization failed with error: {err}");
    }

    // Existance of input file is asserted during cli args parsing
    let instance = JsspInstance::try_from(&config.input_file).unwrap();

    match config.perform_randomsearch {
        true => run_randomsearch(instance, config),
        false => run_jssp_solver(instance, config),
    }
}

fn main() -> Result<(), ()> {
    run();
    Ok(())
}
