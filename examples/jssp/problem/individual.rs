use std::collections::HashSet;

use ecrs::ga::individual::IndividualTrait;

use super::{Machine, Operation};

#[derive(Debug, Clone)]
pub struct JsspIndividual {
    pub chromosome: Vec<f64>,
    pub operations: Vec<Operation>,
    pub machines: Vec<Machine>,
    pub fitness: usize,
    is_fitness_valid: bool,
}

impl JsspIndividual {
    pub fn new(chromosome: Vec<f64>, ops: Vec<Operation>, machines: Vec<Machine>, fitness: usize) -> Self {
        Self {
            chromosome,
            operations: ops,
            machines,
            fitness,
            is_fitness_valid: false,
        }
    }

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

    fn update_active_schedule(
        &self,
        active_schedule: &mut HashSet<usize>,
        finish_times: &[usize],
        time: usize,
    ) {
        let mut to_remove: Vec<usize> = vec![];
        for el in active_schedule.iter() {
            if finish_times[*el] <= time {
                to_remove.push(*el);
            }
        }
        for el in to_remove {
            active_schedule.remove(&el);
        }
    }

    pub fn eval(&mut self) -> usize {
        // We deduce the problem size from the chromosome size
        let n: usize = self.chromosome.len() / 2;

        let mut finish_times = vec![usize::MAX; n + 2];
        let mut scheduled = std::collections::HashSet::new();
        let mut e_set = std::collections::HashSet::<usize>::new();

        scheduled.insert(0);
        finish_times[0] = 0;
        self.operations[0].finish_time = 0;

        let mut g = 1;
        let mut t_g = 0;

        let max_dur = self.operations.iter().map(|op| op.duration).max().unwrap();

        let mut last_finish_time = 0;
        while scheduled.len() < n + 1 {
            // Update e_set
            let mut delay = self.chromosome[n + g - 1] * 1.5 * (max_dur as f64);
            self.update_delay_feasible_set(&mut e_set, &finish_times, delay, t_g);

            while !e_set.is_empty() {
                delay = self.chromosome[n + g - 1] * 1.5 * (max_dur as f64);

                // Select operation with highest priority
                let j = *e_set
                    .iter()
                    .max_by(|&&a, &&b| {
                        self.chromosome[a - 1]
                            .partial_cmp(&self.chromosome[b - 1])
                            .unwrap()
                    })
                    .unwrap();

                let op_j = &self.operations[j];

                // Calculate earliset finish time (in terms of precedence only)
                let pred_j_finish = op_j
                    .preds
                    .iter()
                    .filter(|&id| finish_times[*id] != usize::MAX)
                    .map(|&id| finish_times[id])
                    .max()
                    .unwrap_or(0);

                // Calculate the earliest finish time (in terms of precedence and capacity)
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
                g += 1;

                last_finish_time = usize::max(last_finish_time, finish_time_j);

                if g > n {
                    break;
                }

                delay = self.chromosome[n + g - 1] * 1.5 * (max_dur as f64);

                self.update_delay_feasible_set(&mut e_set, &finish_times, delay, t_g);

                self.machines[op_j.machine].reserve(finish_time_j - op_j.duration..finish_time_j);
            }
            // Update the scheduling time t_g associated with g
            t_g = *finish_times.iter().filter(|&&t| t > t_g).min().unwrap();
        }
        // self.local_search(last_finish_time);

        self.fitness = last_finish_time;
        last_finish_time
    }
}

impl PartialEq for JsspIndividual {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Eq for JsspIndividual {}

impl PartialOrd for JsspIndividual {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl Ord for JsspIndividual {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fitness.cmp(&other.fitness)
    }
}

impl IndividualTrait for JsspIndividual {
    type ChromosomeT = Vec<f64>;
    type FitnessValueT = usize;

    fn chromosome(&self) -> &Self::ChromosomeT {
        &self.chromosome
    }

    fn chromosome_mut(&mut self) -> &mut Self::ChromosomeT {
        &mut self.chromosome
    }

    fn fitness(&self) -> Self::FitnessValueT {
        self.fitness
    }

    fn fitness_mut(&mut self) -> &mut Self::FitnessValueT {
        &mut self.fitness
    }

    fn requires_evaluation(&self) -> bool {
        self.is_fitness_valid
    }
}

impl From<Vec<f64>> for JsspIndividual {
    fn from(value: Vec<f64>) -> Self {
        todo!()
    }
}
