use ecrs::ga::{GAMetadata, Individual};
use ecrs::prelude::crossover::CrossoverOperator;
use ecrs::prelude::population::PopulationGenerator;
use ecrs::prelude::replacement::ReplacementOperator;
use ecrs::prelude::selection::SelectionOperator;

#[allow(unused_imports)]
use ecrs::prelude::*;

// ECRS nakłada zbyt duże ograniczenia.
//
// 1. Postać osobnika jest sprowadzona jedynie do chormosomu -- w przypadku rozwiązania
// które próbuję zaimplementować posiadanie stanu jest krytyczne!
//

fn decode_chromosome(chromosome: &Vec<f64>, rmc: &mut Vec<Vec<i32>>) {
    // We deduce the problem size from the chromosome
    let n: usize = chromosome.len() / 2;

    let mut priorities = vec![0; n];

    let mut active_schedule = std::collections::HashSet::new();
    let mut finish_times = std::collections::HashSet::new();
    let mut scheduled = std::collections::HashSet::new();
    let mut e_set = std::collections::HashSet::<i32>::new();

    active_schedule.insert(0);
    finish_times.insert(0);
    scheduled.insert(0);

    let mut g = 1;
    let mut t = 0;

    // while scheduled.len() < n + 1 {
    //    // Update e_set
    //    // e_set = ???
    //    while !e_set.is_empty() {
    //     // Select operation with highest priority
    //     let j = e_set.iter().enumerate().max_by_key(|(_, &val)| val).map(|(idx, _)| idx);
    //
    //     // Calculate earliset finish time (in terms of precedence only)
    //     earliset_finish_j =
    //    }
    // }
}

#[derive(Debug)]
struct Operation {
    id: usize,
    finish_time: usize,
    duration: usize,
    machine: usize,

    // Should I hold references to other operations or just their ids
    preds: Vec<usize>,
}

#[derive(Debug)]
struct Machine {
    id: usize,
    // dummy structure
    rmc: Vec<usize>,
}

impl Machine {
    fn is_idle(&self, range: std::ops::RangeInclusive<usize>) -> bool {
        assert!(*range.end() < 20);

        for i in range {
            if self.rmc[i] == 0 {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
struct JsspIndividual {
    chromosome: Vec<f64>,
    operations: Vec<Operation>,
    machines: Vec<Machine>,
    fitness: usize,
}

#[derive(Debug)]
struct JsspConfig {
    n_jobs: usize,
    n_machines: usize,
}

#[derive(Debug)]
struct JsspState {
    cfg: JsspConfig,
    population: Vec<JsspIndividual>,
}

impl JsspState {
    fn build_operations(&self) -> Vec<Operation> {
        // For now I'm implementing just for single case
        assert!(self.cfg.n_jobs == 2 && self.cfg.n_machines == 2);

        let mut operations = Vec::with_capacity(self.cfg.n_jobs + 2);
        operations.push(Operation {
            id: 0,
            finish_time: 0,
            duration: 0,
            machine: 0,
            preds: Vec::new(),
        });

        operations.push(Operation {
            id: 1,
            finish_time: 0,
            duration: 4,
            machine: 1,
            // preds: Vec::new(), // or maybe should I put Operation 0 here?
            preds: vec![0],
        });

        operations.push(Operation {
            id: 2,
            finish_time: 0,
            duration: 2,
            machine: 0,
            preds: vec![0, 1],
        });

        operations.push(Operation {
            id: 3,
            finish_time: 0,
            duration: 1,
            machine: 0,
            // preds: Vec::new(),
            preds: vec![0],
        });

        operations.push(Operation {
            id: 4,
            finish_time: 0,
            duration: 3,
            machine: 1,
            preds: vec![0, 3],
        });

        operations.push(Operation {
            id: 5,
            finish_time: 0,
            duration: 0,
            machine: 0,
            preds: vec![0, 1, 2, 3, 4],
        });

        operations
    }

    fn build_machines(&self) -> Vec<Machine> {
        // To represent machine we could think of some sparse structure, but for now let it be
        // vector

        let machines = vec![
            Machine {
                id: 0,
                rmc: vec![1, 20],
            },
            Machine {
                id: 1,
                rmc: vec![1, 20],
            },
        ];

        machines
    }

    fn init_pop(&mut self, size: usize) {
        self.population = ga::population::RandomPoints::with_constraints(2, vec![(0.0..1.0), (0.0..1.0)])
            .generate(size)
            .into_iter()
            .map(|idv| JsspIndividual {
                chromosome: idv.chromosome,
                operations: self.build_operations(),
                fitness: usize::MAX,
                machines: self.build_machines(),
            })
            .collect();
    }

    fn inject_ecrs_pop(&mut self, population: Vec<Individual<Vec<f64>>>) {
        self.population = population
            .into_iter()
            .map(|idv| JsspIndividual {
                chromosome: idv.chromosome,
                operations: self.build_operations(),
                fitness: usize::MAX,
                machines: self.build_machines(),
            })
            .collect();
    }

    fn eval_pop(&mut self) {
        for idv in self.population.iter_mut() {
            idv.eval();
        }
    }
}

impl JsspIndividual {
    fn eval(&mut self) -> usize {
        // We deduce the problem size from the chromosome
        let n: usize = self.chromosome.len() / 2;

        let mut active_schedule = std::collections::HashSet::new();
        let mut finish_times = vec![0, n + 2];
        let mut scheduled = std::collections::HashSet::new();
        let mut e_set = std::collections::HashSet::<usize>::new();

        active_schedule.insert(0);
        scheduled.insert(0);

        let mut g = 1;
        let mut t = 0;

        while scheduled.len() < n + 1 {
            // Update e_set
            // e_set = ???
            while !e_set.is_empty() {
                // Select operation with highest priority
                let j = e_set
                    .iter()
                    .enumerate()
                    .max_by(|(_, &a), (_, &b)| self.chromosome[a].partial_cmp(&self.chromosome[b]).unwrap())
                    // .max_by_key(|(_, &val)| self.chromosome[val])
                    .map(|(idx, _)| idx)
                    .unwrap();
                let op_j = &self.operations[j];

                // Calculate earliset finish time (in terms of precedence only)
                let mut earliset_finish_j = usize::MIN;

                for &pred in op_j.preds.iter() {
                    if self.operations[pred as usize].finish_time > earliset_finish_j {
                        earliset_finish_j = pred;
                    }
                }

                let pred_j_finish = earliset_finish_j;
                earliset_finish_j += self.operations[earliset_finish_j].duration;

                // Calculate the earliest finish time (in terms of precedence and capacity)
                
                let mut finish_time_j = usize::MAX;
                // let mut finish_times_g = scheduled.iter().map(|i_op| self.operations[i_op].)
                // for i_op in finish_times

            }
        }
        0
    }
}

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

    for _ in 0..100 {
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
