use ecrs::ga::{GAMetadata, Individual};
use ecrs::prelude::crossover::CrossoverOperator;
use ecrs::prelude::population::PopulationGenerator;
use ecrs::prelude::replacement::ReplacementOperator;
use ecrs::prelude::selection::SelectionOperator;

#[allow(unused_imports)]
use ecrs::prelude::*;

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
        assert!(*range.end() < 40);

        for i in range {
            if self.rmc[i] == 0 {
                return false;
            }
        }
        return true;
    }

    fn reserve(&mut self, range: std::ops::RangeInclusive<usize>) {
        for i in range {
            self.rmc[i] = 0;
        }
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
        assert!(self.cfg.n_jobs == 4 && self.cfg.n_machines == 2);

        let mut operations = Vec::with_capacity(self.cfg.n_jobs + 2);
        operations.push(Operation {
            id: 0,
            finish_time: usize::MAX,
            duration: 0,
            machine: 0,
            preds: Vec::new(),
        });

        operations.push(Operation {
            id: 1,
            finish_time: usize::MAX,
            duration: 4,
            machine: 1,
            // preds: Vec::new(), // or maybe should I put Operation 0 here?
            preds: vec![0],
        });

        operations.push(Operation {
            id: 2,
            finish_time: usize::MAX,
            duration: 2,
            machine: 0,
            preds: vec![0, 1],
        });

        operations.push(Operation {
            id: 3,
            finish_time: usize::MAX,
            duration: 1,
            machine: 0,
            // preds: Vec::new(),
            preds: vec![0],
        });

        operations.push(Operation {
            id: 4,
            finish_time: usize::MAX,
            duration: 3,
            machine: 1,
            preds: vec![0, 3],
        });

        operations.push(Operation {
            id: 5,
            finish_time: usize::MAX,
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
                rmc: vec![1; 40],
            },
            Machine {
                id: 1,
                rmc: vec![1; 40],
            },
        ];

        machines
    }

    fn init_pop(&mut self, size: usize) {
        self.population = ga::population::RandomPoints::with_constraints_single(8, (0.0..1.0))
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
        for op in self.operations.iter() {
            println!("Operation {}", op.id);
        }
        // We deduce the problem size from the chromosomno w sumie, niby ma działać
        let n: usize = self.chromosome.len() / 2;
        dbg!(n);

        let mut active_schedule = std::collections::HashSet::new();
        let mut finish_times = vec![usize::MAX; n + 2];
        debug_assert!(finish_times.len() == n + 2);
        let mut scheduled = std::collections::HashSet::new();
        let mut e_set = std::collections::HashSet::<usize>::new();

        active_schedule.insert(0);
        scheduled.insert(0);
        finish_times[0] = 0;
        self.operations[0].finish_time = 0;

        let mut g = 1;
        let mut t_g = 0;

        let max_dur = self.operations.iter().map(|op| op.duration).max().unwrap();

        println!("Entering main loop with g = 1, t_g = 0, max_dur = {}", max_dur);

        while scheduled.len() < n + 1 {
            dbg!(g);
            // Update e_set
            let delay = self.chromosome[n + g - 1] * 1.5 * (max_dur as f64);
            dbg!(delay);

            println!("finish_times: ");
            for item in finish_times.iter() {
                print!("{} ", item);
            }
            println!();

            self.operations
                .iter()
                .filter(|op| {
                    // it is asssumed here, that dependencies are in order
                    if let Some(pred) = op.preds.last() {
                        println!("Pred: {}", pred);
                        return finish_times[*pred] as f64 <= t_g as f64 + delay;
                    } else {
                        return false;
                    }
                })
                .for_each(|op| {
                    e_set.insert(op.id);
                });

            print!("e_set: ");
            for item in e_set.iter() {
                print!("{} ", item);
            }
            println!();

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
                let mut earliest_finish_j = op_j
                    .preds
                    .iter()
                    .filter(|&id| finish_times[*id] != usize::MAX)
                    .map(|&id| finish_times[id])
                    .max()
                    .unwrap_or(0);

                let pred_j_finish = earliest_finish_j;
                earliest_finish_j += self.operations[op_j.id].duration;

                // Calculate the earliest finish time (in terms of precedence and capacity)

                let max_finish_time_j = finish_times
                    .iter()
                    .filter(|&&i| i != usize::MAX)
                    .max()
                    .unwrap()
                    .clone();
                let mut min_finish_time_j = finish_times
                    .iter()
                    .filter(|&&i| i != usize::MAX)
                    .min()
                    .unwrap()
                    .clone();
                if min_finish_time_j > pred_j_finish {
                    min_finish_time_j = pred_j_finish;
                }

                debug_assert!(min_finish_time_j <= max_finish_time_j);

                let finish_time_j = finish_times
                    .iter()
                    .filter(|&&t| t != usize::MAX && t >= pred_j_finish)
                    .filter(|&&t| self.machines[self.operations[op_j.id].machine].is_idle(t..=t + op_j.duration))
                    .min()
                    .unwrap() + self.operations[op_j.id].duration;
                    

                // let finish_time_j = (min_finish_time_j..=max_finish_time_j)
                //     .filter(|&t| {
                //         self.machines[self.operations[op_j.id].machine].is_idle(t..=t + op_j.duration)
                //     })
                //     .min()
                //     .unwrap()
                //     + self.operations[op_j.id].duration;
                //
                scheduled.insert(op_j.id);
                finish_times[op_j.id] = finish_time_j;
                g += 1;

                // Update active schedule
                let mut to_remove: Vec<usize> = vec![];
                for el in active_schedule.iter() {
                    if finish_times[*el] <= t_g {
                        to_remove.push(*el);
                    }
                }

                for el in to_remove {
                    active_schedule.remove(&el);
                }

                // Update e_set
                println!("Index for delay: {}", n + g - 1);
                let delay = self.chromosome[n + g - 1] * 1.5 * (max_dur as f64);

                self.operations
                    .iter()
                    .filter(|op| {
                        // it is asssumed here, that dependencies are in order
                        if let Some(pred) = op.preds.last() {
                            return finish_times[*pred] as f64 <= t_g as f64 + delay;
                        } else {
                            return false;
                        }
                    })
                    .for_each(|op| {
                        e_set.insert(op.id);
                    });

                // Update RMC
                self.machines[self.operations[op_j.id].machine]
                    .reserve(finish_time_j - op_j.duration..=finish_time_j);
            }
            // Update the time t_g associated with g
            t_g = finish_times.iter().filter(|&&t| t > t_g).min().unwrap().clone();
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
