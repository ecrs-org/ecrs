use std::collections::HashSet;

use crate::util::{print_hash_set, print_slice};

use super::{Machine, Operation};

#[derive(Debug)]
pub struct JsspIndividual {
    pub chromosome: Vec<f64>,
    pub operations: Vec<Operation>,
    pub machines: Vec<Machine>,
    pub fitness: usize,
}

impl JsspIndividual {
    fn update_delay_feasible_set(
        &self,
        feasibles: &mut HashSet<usize>,
        finish_times: &[usize],
        delay: f64,
        time: usize,
    ) {
        // As we are iterating over all operations, we want to make sure that the feasibles set is
        // empty before inserting anything.
        feasibles.clear();

        println!("Updating e_set");
        self.operations
            .iter()
            .filter(|op| finish_times[op.id] == usize::MAX)
            .filter(|op| {
                // It is assumed here, that dependencies are in order

                // If there is a predecessor operation -- its finish time is our earliest start
                // time ==> we want to check whether all `op` dependencies can be finished before
                // current schedule time + delay window.
                for &pred in op.preds.iter() {
                    if finish_times[pred] as f64 > time as f64 + delay {
                        return false;
                    }
                }
                true
            })
            .for_each(|op| {
                feasibles.insert(op.id);
            })
    }

    pub fn eval(&mut self) -> usize {
        println!("++++++++++++++++++++++++++++++++++");
        // We deduce the problem size from the chromosome size
        let n: usize = self.chromosome.len() / 2;
        println!("Deduced problem size n = {n}");

        let mut active_schedule = std::collections::HashSet::new();
        let mut finish_times = vec![usize::MAX; n + 2];
        let mut scheduled = std::collections::HashSet::new();
        let mut e_set = std::collections::HashSet::<usize>::new();

        active_schedule.insert(0);
        scheduled.insert(0);
        finish_times[0] = 0;
        self.operations[0].finish_time = 0;

        let mut g = 1;
        let mut t_g = 0;

        let max_dur = self.operations.iter().map(|op| op.duration).max().unwrap();

        println!("Entering main loop with g = 1, t_g = 0, max_dur = {max_dur}");

        let mut last_finish_time = 0;
        while scheduled.len() < n + 1 {
            println!("==================================");
            println!("g = {g}");

            // Update e_set
            let delay = self.chromosome[n + g - 1] * 1.5 * (max_dur as f64);
            println!("delay = {delay}");

            print!("finish_times: ");
            print_slice(&finish_times);

            self.update_delay_feasible_set(&mut e_set, &finish_times, delay, t_g);

            print!("e_set: ");
            print_hash_set(&e_set);

            while !e_set.is_empty() {
                println!("---------------------------------");
                println!("Inner loop for g = {g}");
                print!("e_set: ");
                print_hash_set(&e_set);

                // Select operation with highest priority
                let j = *e_set
                    .iter()
                    .max_by(|&&a, &&b| self.chromosome[a].partial_cmp(&self.chromosome[b]).unwrap())
                    .unwrap();

                let op_j = &self.operations[j];

                println!("Operation with highest priority: {j}");

                // Calculate earliset finish time (in terms of precedence only)
                let pred_j_finish = op_j
                    .preds
                    .iter()
                    .filter(|&id| finish_times[*id] != usize::MAX)
                    .map(|&id| finish_times[id])
                    .max()
                    .unwrap_or(0);

                // Calculate the earliest finish time (in terms of precedence and capacity)
                println!("pred finish_time = {pred_j_finish}");

                let finish_time_j = finish_times
                    .iter()
                    .filter(|&&t| t != usize::MAX && t >= pred_j_finish)
                    .filter(|&&t| self.machines[op_j.machine].is_idle(t..=t + op_j.duration))
                    .min()
                    .unwrap()
                    + op_j.duration;

                // Update state
                scheduled.insert(op_j.id);
                finish_times[op_j.id] = finish_time_j;
                println!(
                    "Scheduled op {} with for time = {}..{}, machine = {}",
                    j,
                    finish_time_j - op_j.duration,
                    finish_time_j,
                    op_j.machine
                );
                g += 1;

                last_finish_time = usize::max(last_finish_time, finish_time_j);

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
                // println!("Index for delay = {}", n + g - 1);
                // let delay = self.chromosome[n + g - 1] * 1.5 * (max_dur as f64);

                e_set.remove(&j);

                println!("Removed op {j} from e_set");
                print!("e_set: ");
                print_hash_set(&e_set);

                // Update RMC
                self.machines[op_j.machine].reserve(finish_time_j - op_j.duration..finish_time_j);
                println!("---------------------------------");
            }
            // Update the scheduling time t_g associated with g
            t_g = *finish_times.iter().filter(|&&t| t > t_g).min().unwrap();
            println!("==================================");
        }
        println!("++++++++++++++++++++++++++++++++++");
        last_finish_time
    }
}
