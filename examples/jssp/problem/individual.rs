use std::collections::HashSet;

use ecrs::ga::individual::IndividualTrait;
use itertools::Itertools;
use log::{debug, info, trace, warn};

use super::{Edge, EdgeKind, Machine, Operation};

/// Models single solution to the JSSP problem instance
#[derive(Debug, Clone)]
pub struct JsspIndividual {
    /// Encoding of the solution. This can be decoded to the proper solution
    pub chromosome: Vec<f64>,
    /// Clone of all operations from the problem instance
    pub operations: Vec<Operation>,
    /// Clone of all machines from the problem instance
    pub machines: Vec<Machine>,
    /// If computed - fitness value of this solution. Check `is_fitness_valid`
    /// property to determine whether this value is up to date
    /// This is not an Option for some practical reasons
    /// TODO: But this should be an Option or some enum with additional information
    pub fitness: usize,
    /// If `true` the `fitness` field holds the value for the current `chromosome`
    /// and does need to be recomputed. This must be kept in sync!
    pub is_fitness_valid: bool,
    /// TODO: Determine what I've used it for
    is_dirty: bool,
}

impl JsspIndividual {
    pub fn new(chromosome: Vec<f64>, ops: Vec<Operation>, machines: Vec<Machine>, fitness: usize) -> Self {
        Self {
            chromosome,
            operations: ops,
            machines,
            fitness,
            is_fitness_valid: false,
            is_dirty: false,
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

    // fn update_active_schedule(
    //     &self,
    //     active_schedule: &mut HashSet<usize>,
    //     finish_times: &[usize],
    //     time: usize,
    // ) {
    //     let mut to_remove: Vec<usize> = vec![];
    //     for el in active_schedule.iter() {
    //         if finish_times[*el] <= time {
    //             to_remove.push(*el);
    //         }
    //     }
    //     for el in to_remove {
    //         active_schedule.remove(&el);
    //     }
    // }

    fn determine_critical_path(&mut self) {
        let mut visited = vec![false; self.operations.len()];
        self.calculate_critical_distance(0, &mut visited)
    }

    fn calculate_critical_distance(&mut self, op_id: usize, visited: &mut Vec<bool>) {
        let mut stack: Vec<usize> = Vec::with_capacity(visited.len() * 2);

        stack.push(op_id);
        while !stack.is_empty() {
            let crt_op_id = *stack.last().unwrap();
            // In current implementation it is possible (highly likely) that a vertex might be pushed
            // multiple times on the stack, before being processed, so we process the vertex iff it
            // has not been visited already.
            if !visited[crt_op_id] {
                let mut has_not_visited_neigh = false;
                for edge in self.operations[crt_op_id].edges_out.iter() {
                    if !visited[edge.neigh_id] {
                        stack.push(edge.neigh_id);
                        has_not_visited_neigh = true;
                    }
                }

                if !has_not_visited_neigh {
                    visited[crt_op_id] = true;
                    stack.pop();

                    if !self.operations[crt_op_id].edges_out.is_empty() {
                        let cp_edge = *self.operations[crt_op_id]
                            .edges_out
                            .iter()
                            .max_by_key(|edge| self.operations[edge.neigh_id].critical_distance)
                            .unwrap();

                        self.operations[crt_op_id].critical_distance = self.operations[crt_op_id].duration
                            + self.operations[cp_edge.neigh_id].critical_distance;
                        self.operations[crt_op_id].critical_path_edge = Some(cp_edge);
                    } else {
                        self.operations[crt_op_id].critical_distance = self.operations[crt_op_id].duration;
                    }
                }
            } else {
                stack.pop();
            }
        }
    }

    fn determine_critical_blocks(&mut self, blocks: &mut Vec<Vec<usize>>) {
        let mut crt_op = &self.operations[self.operations[0].critical_path_edge.unwrap().neigh_id];

        blocks.clear();
        blocks.push(Vec::new());
        while let Some(ref edge) = crt_op.critical_path_edge {
            blocks.last_mut().unwrap().push(crt_op.id);
            if edge.kind == EdgeKind::JobSucc {
                blocks.push(Vec::new());
            }
            crt_op = &self.operations[edge.neigh_id];
        }
        // there should be empty block at the end
        debug_assert!(blocks.last().unwrap().is_empty());
        blocks.pop();
    }

    fn determine_makespan(&mut self) -> usize {
        self.determine_critical_path();
        self.operations[0].critical_distance
    }

    fn swap_ops(&mut self, first_op_id: usize, sec_op_id: usize) {
        // We assume few things here:
        debug_assert!(first_op_id != 0 && sec_op_id != 0);

        // Check wheter there is follow up machine element
        let block_mach_succ_opt = if let Some(Edge {
            neigh_id: block_mach_succ,
            kind: _,
        }) = self.operations[sec_op_id].edges_out.get(1)
        {
            Some(*block_mach_succ)
        } else {
            None
        };

        if let Some(block_mach_succ) = block_mach_succ_opt {
            self.operations[first_op_id].edges_out[1].neigh_id = block_mach_succ;
            self.operations[block_mach_succ].machine_pred = Some(first_op_id);
            self.operations[sec_op_id].edges_out[1].neigh_id = first_op_id;
        } else {
            self.operations[first_op_id].edges_out.pop();
            self.operations[sec_op_id]
                .edges_out
                .push(Edge::new(first_op_id, EdgeKind::MachineSucc));
        }

        // Check whether there is predecessor machine element
        if let Some(block_mach_pred) = self.operations[first_op_id].machine_pred {
            self.operations[block_mach_pred].edges_out[1].neigh_id = sec_op_id;
            self.operations[sec_op_id].machine_pred = Some(block_mach_pred);
        } else {
            self.operations[sec_op_id].machine_pred = None;
        }

        self.operations[first_op_id].machine_pred = Some(sec_op_id);
    }

    fn local_search(&mut self) -> usize {
        // let mut vertices_in_topo_order: VecDeque<usize> = VecDeque::with_capacity(self.operations.len());
        let mut crt_sol_updated = true;
        let mut blocks: Vec<Vec<usize>> = Vec::new();
        let mut crt_makespan = usize::MAX;

        while crt_sol_updated {
            crt_sol_updated = false;
            crt_makespan = self.determine_makespan();
            self.determine_critical_blocks(&mut blocks);

            // Traverse along critical path
            let mut crt_block = 0;

            while crt_block < blocks.len() && !crt_sol_updated {
                let block = &blocks[crt_block];

                // Not first block
                if crt_block > 0 && block.len() >= 2 {
                    self.swap_ops(block[0], block[1]);

                    let new_makespan = self.determine_makespan();
                    if new_makespan < crt_makespan {
                        crt_makespan = new_makespan;
                        crt_sol_updated = true;
                    } else {
                        self.swap_ops(block[1], block[0]);
                    }
                }

                // Not last block
                if crt_block != blocks.len() - 1 && !crt_sol_updated && block.len() >= 2 {
                    let last_op_id = block[block.len() - 1];
                    let sec_last_op_id = block[block.len() - 2];
                    self.swap_ops(sec_last_op_id, last_op_id);

                    let new_makespan = self.determine_makespan();
                    if new_makespan < crt_makespan {
                        crt_makespan = new_makespan;
                        crt_sol_updated = true;
                    } else {
                        self.swap_ops(last_op_id, sec_last_op_id);
                    }
                }
                crt_block += 1;
            }
        }
        crt_makespan
    }

    pub fn eval(&mut self) -> usize {
        if self.is_dirty {
            self.reset();
        }

        // We deduce the problem size from the chromosome size
        let n: usize = self.chromosome.len() / 2;

        let mut finish_times = vec![usize::MAX; n + 2];
        let mut scheduled = HashSet::<usize>::new();
        let mut delay_feasibles = HashSet::<usize>::new();

        scheduled.insert(0);
        finish_times[0] = 0;
        self.operations[0].finish_time = Some(0);

        let mut g = 1;
        let mut t_g = 0;

        let max_dur = self.operations.iter().map(|op| op.duration).max().unwrap();

        let mut last_finish_time = 0;
        while scheduled.len() < n + 1 {
            // Update e_set
            let mut delay = self.chromosome[n + g - 1] * 1.5 * (max_dur as f64);
            self.update_delay_feasible_set(&mut delay_feasibles, &finish_times, delay, t_g);

            while !delay_feasibles.is_empty() {
                // delay = self.chromosome[n + g - 1] * 1.5 * (max_dur as f64);

                // Select operation with highest priority
                let j = *delay_feasibles
                    .iter()
                    .max_by(|&&a, &&b| {
                        self.chromosome[a - 1]
                            .partial_cmp(&self.chromosome[b - 1])
                            .unwrap()
                    })
                    .unwrap();

                let op_j_duration = self.operations[j].duration;
                let op_j_machine = self.operations[j].machine;
                let op_j = &self.operations[j];

                // Calculate the earliest finish time (in terms of precedence only)
                // TODO: We do not need to look on all predecessors. The direct one is enough, as
                // it could not be scheduled before all his preds were finished. The question is:
                // is the order of predecessors guaranteed? Look for places that manipulate this
                // field!
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
                    .filter(|&&t| self.machines[op_j.machine].is_idle(t..=t + op_j_duration))
                    .min()
                    .unwrap()
                    + op_j_duration;

                // Update state
                scheduled.insert(op_j.id);
                finish_times[op_j.id] = finish_time_j;
                g += 1;

                last_finish_time = usize::max(last_finish_time, finish_time_j);

                if let Some(last_sch_op) = self.machines[op_j_machine].last_scheduled_op {
                    self.operations[last_sch_op]
                        .edges_out
                        .push(Edge::new(j, EdgeKind::MachineSucc));
                    self.operations[j].machine_pred = Some(last_sch_op);
                }

                self.machines[op_j_machine].reserve(finish_time_j - op_j_duration..finish_time_j, j);

                if g > n {
                    break;
                }

                delay = self.chromosome[n + g - 1] * 1.5 * (max_dur as f64);

                self.update_delay_feasible_set(&mut delay_feasibles, &finish_times, delay, t_g);
            }
            // Update the scheduling time t_g associated with g
            t_g = *finish_times.iter().filter(|&&t| t > t_g).min().unwrap();
        }
        let makespan = usize::min(last_finish_time, self.local_search());

        self.fitness = makespan;
        self.is_fitness_valid = true;
        self.is_dirty = true;

        makespan
    }

    /// Resets all machines & operations associated with this individual
    fn reset(&mut self) {
        self.machines.iter_mut().for_each(|machine| machine.reset());
        self.operations.iter_mut().for_each(|op| op.reset());
    }
}

impl PartialEq for JsspIndividual {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Eq for JsspIndividual {}

impl PartialOrd for JsspIndividual {
    #[allow(clippy::incorrect_partial_ord_impl_on_ord_type)]
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

    #[inline]
    fn chromosome(&self) -> &Self::ChromosomeT {
        &self.chromosome
    }

    #[inline]
    fn chromosome_mut(&mut self) -> &mut Self::ChromosomeT {
        &mut self.chromosome
    }

    #[inline]
    fn fitness(&self) -> Self::FitnessValueT {
        self.fitness
    }

    #[inline]
    fn fitness_mut(&mut self) -> &mut Self::FitnessValueT {
        &mut self.fitness
    }

    #[inline]
    fn requires_evaluation(&self) -> bool {
        !self.is_fitness_valid
    }
}

impl From<Vec<f64>> for JsspIndividual {
    fn from(chromosome: Vec<f64>) -> Self {
        Self {
            chromosome,
            operations: Vec::new(),
            machines: Vec::new(),
            fitness: usize::MAX,
            is_fitness_valid: false,
            is_dirty: false,
        }
    }
}
