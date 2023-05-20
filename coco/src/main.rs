#![allow(dead_code)]

mod adapter;

use coco_rs::{LogLevel, Observer, ObserverName, Problem, RandomState, Suite, SuiteName};
use ecrs::{
    ga::{individual::RealValueIndividual, probe::EmptyProbe},
    prelude::{
        crossover::Uniform, mutation::Reversing, population::RandomPoints,
        replacement::WeakParent, selection::Tournament, *,
    },
};

const BUDGET_MULTIPLIER: usize = 10;
const INDEPENDENT_RESTARTS_100K: u64 = 1e5 as u64;
const INDEPENDENT_RESTARTS_100: u64 = 1e2 as u64;
const RANDOM_SEED: u32 = 0xdeadbeef;

type SearchFn = fn(&mut Problem, usize, &mut RandomState) -> ();

fn main() {
    let random_generator = &mut RandomState::new(RANDOM_SEED);
    println!("Running the example experiment... (might take time, be patient)");

    coco_rs::set_log_level(LogLevel::Info);

    run_experiment(
        ecrs_ga_search,
        SuiteName::Bbob,
        "",
        ObserverName::Bbob,
        "result_folder: ecrs_on_bbob-2",
        random_generator,
    );

    run_experiment(
        random_search,
        SuiteName::Bbob,
        "",
        ObserverName::Bbob,
        "result_folder: random_on_bbob",
        random_generator,
    );

    println!("Done!");
}

fn run_experiment(
    search_fn: SearchFn,
    suite_name: SuiteName,
    suite_options: &str,
    observer_name: ObserverName,
    observer_options: &str,
    random_generator: &mut RandomState,
) {
    let suite = &mut Suite::new(suite_name, "", suite_options).unwrap();
    let observer = &mut Observer::new(observer_name, observer_options).unwrap();

    while let Some(problem) = &mut suite.next_problem(Some(observer)) {
        let dimension = problem.dimension();
        let no_constraints = problem.number_of_constraints();
        let no_objectives = problem.number_of_objectives();
        let no_int_objectives = problem.number_of_integer_variables();

        if no_constraints != 0 {
            panic!("PROBLEM OF DIMENSION {dimension} WITH {no_constraints} CONSTRAINTS");
        }

        if no_objectives > 1 {
            panic!("PROBLEM OF DIMENSION {dimension} WITH {no_objectives} OBJECTIVES");
        }

        if no_int_objectives > 0 {
            panic!("PROBLEM OF DIMENSION {dimension} WITH {no_int_objectives} INT OBJECTIVES");
        }

        for _ in 1..=INDEPENDENT_RESTARTS_100K {
            let evaluations_done = problem.evaluations() + problem.evaluations_constraints();
            let evaluations_remaining =
                (dimension * BUDGET_MULTIPLIER).saturating_sub(evaluations_done as usize);

            if problem.final_target_hit() || evaluations_remaining == 0 {
                break;
            }

            search_fn(problem, evaluations_remaining, random_generator);
        }
    }
}

fn ecrs_ga_search(problem: &mut Problem, _max_budget: usize, _random_generator: &mut RandomState) {
    let dimension = problem.dimension();
    let population_size = 100;

    let constraints = problem.get_ranges_of_interest();

    let fitness = adapter::CocoFitness::new(problem);

    // TODO: Find a way to take advantage of feasible solution
    // let mut init_solution_box = [0.0];

    let mut solver = ecrs::ga::Builder::new::<
        RealValueIndividual,
        Reversing,
        Uniform,
        Tournament,
        WeakParent,
        RandomPoints,
        adapter::CocoFitness,
        EmptyProbe,
    >()
    .set_population_size(population_size)
    .set_max_generation_count(10_000)
    .set_max_duration(std::time::Duration::from_millis(200))
    .set_fitness(fitness)
    .set_probe(ecrs::ga::probe::EmptyProbe)
    .set_population_generator(population::RandomPoints::with_constraints_inclusive(
        dimension,
        constraints,
    ))
    .set_selection_operator(selection::Tournament::new(0.2))
    .set_crossover_operator(crossover::Uniform::new())
    .set_mutation_operator(mutation::Reversing::new())
    .set_replacement_operator(replacement::WeakParent::new())
    .build();

    solver.run();
}

/// Code taken from docs of COCO platform
/// See their GitHub for more information and links
/// https://github.com/numbbo/coco
fn random_search(problem: &mut Problem, max_budget: usize, random_generator: &mut RandomState) {
    let dimension = problem.dimension();
    let number_of_objectives = problem.number_of_objectives();
    let numver_of_constraints = problem.number_of_constraints();
    let number_of_integer_variables = problem.number_of_integer_variables();
    let bounds = problem.get_ranges_of_interest();

    let x = &mut vec![0.0; dimension];
    let y = &mut vec![0.0; number_of_objectives];
    let c = &mut vec![0.0; numver_of_constraints];

    problem.initial_solution(x);
    problem.evaluate_function(x, y);

    for _ in 0..max_budget {
        for (i, xi) in x.iter_mut().enumerate() {
            let (lower, upper) = bounds[i].clone().into_inner();
            *xi = lower + random_generator.uniform() * (upper - lower);

            if i < number_of_integer_variables {
                *xi = xi.round();
            }
        }

        if numver_of_constraints > 0 {
            problem.evaluate_constraint(x, c);
        }

        problem.evaluate_function(x, y);
    }
}
