use std::collections::HashSet;

use ecrs::ga::individual::IndividualTrait;
use itertools::Itertools;

use super::{Edge, EdgeKind, Machine, Operation};

#[derive(Debug, Clone)]
pub struct JsspIndividual {
    pub chromosome: Vec<f64>,
    pub operations: Vec<Operation>,
    pub machines: Vec<Machine>,
    pub fitness: usize,
    pub is_fitness_valid: bool,
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
            }
        }
    }

    fn determine_critical_blocks(&mut self, blocks: &mut Vec<Vec<usize>>) {
        let mut crt_op = &self.operations[self.operations[0].critical_path_edge.unwrap().neigh_id];

        blocks.clear();
        blocks.push(Vec::new());
        while let Some(edge) = crt_op.critical_path_edge {
            blocks.last_mut().unwrap().push(crt_op.id);
            if edge.kind == EdgeKind::JobSucc {
                blocks.push(Vec::new());
            }
            crt_op = &self.operations[edge.neigh_id];
        }
        // there should be empty block at the end
        assert!(blocks.last().unwrap().is_empty());
        blocks.pop();
    }

    fn determine_makespan(&mut self) -> usize {
        self.determine_critical_path();
        self.operations[0].critical_distance
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
            let block = &blocks[crt_block];

            while crt_block < blocks.len() && !crt_sol_updated {
                // // Not first block
                if crt_block > 0 && block.len() >= 2 {
                    // Swap first two operations of current block in the current solution
                    // Move to the block beginning

                    let index_1 = self.operations[block[0]]
                        .edges_out
                        .iter()
                        .find_position(|edge| edge.kind == EdgeKind::MachineSucc)
                        .unwrap()
                        .0;

                    let edge_rm_1 = self.operations[block[0]].edges_out.swap_remove(index_1);
                    self.operations[block[1]].edges_out.push(Edge {
                        neigh_id: block[0],
                        kind: EdgeKind::MachineSucc,
                    });

                    if block.len() >= 3 {
                        let index_rm_2 = self.operations[block[1]]
                            .edges_out
                            .iter()
                            .find_position(|edge| edge.kind == EdgeKind::MachineSucc)
                            .unwrap()
                            .0;
                        let edge = self.operations[block[1]].edges_out.swap_remove(index_rm_2);
                        self.operations[block[0]].edges_out.push(edge);
                    }

                    let new_makespan = self.determine_makespan();
                    if new_makespan < crt_makespan {
                        // I should remeber here what was the solution, but it is done in the
                        // background by modyfing the edges of the solution
                        crt_sol_updated = true;
                        crt_makespan = new_makespan;
                    } else {
                        // Restore solution
                        // New edges are always last (push implementation)
                        if block.len() >= 3 {
                            self.operations[block[0]].edges_out.pop();
                        }
                        self.operations[block[1]].edges_out.pop();
                        self.operations[block[0]].edges_out.push(edge_rm_1);
                    }
                }

                // Not last block
                if crt_block != blocks.len() - 1 && !crt_sol_updated && block.len() >= 2 {
                    let sec_last_op_i = block.len() - 2;
                    let last_op_i = block.len() - 1;

                    let sec_last_rm_i = self.operations[sec_last_op_i]
                        .edges_out
                        .iter()
                        .find_position(|edge| edge.kind == EdgeKind::MachineSucc)
                        .unwrap()
                        .0;

                    let sec_last_rm_edge = self.operations[sec_last_op_i]
                        .edges_out
                        .swap_remove(sec_last_rm_i);
                    self.operations[last_op_i].edges_out.push(Edge {
                        neigh_id: sec_last_op_i,
                        kind: EdgeKind::MachineSucc,
                    });

                    if let Some(&third_last_i) = block.get(block.len() - 3) {
                        self.operations[third_last_i]
                            .edges_out
                            .iter_mut()
                            .find(|edge| edge.kind == EdgeKind::MachineSucc)
                            .unwrap()
                            .neigh_id = last_op_i;
                    }

                    let new_makespan = self.determine_makespan();
                    if new_makespan < crt_makespan {
                        crt_sol_updated = true;
                        crt_makespan = new_makespan;
                    } else {
                        self.operations[last_op_i].edges_out.pop();
                        self.operations[sec_last_op_i].edges_out.push(sec_last_rm_edge);

                        if let Some(&third_last_i) = block.get(block.len() - 3) {
                            self.operations[third_last_i]
                                .edges_out
                                .iter_mut()
                                .find(|edge| edge.kind == EdgeKind::MachineSucc)
                                .unwrap()
                                .neigh_id = sec_last_op_i;
                        }
                    }
                }
                crt_block += 1;
            }
        }
        crt_makespan
    }

    pub fn eval(&mut self) -> usize {
        // We deduce the problem size from the chromosome size
        let n: usize = self.chromosome.len() / 2;

        let mut finish_times = vec![usize::MAX; n + 2];
        let mut scheduled = std::collections::HashSet::new();
        let mut delay_feasibles = std::collections::HashSet::<usize>::new();

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

                self.update_delay_feasible_set(&mut delay_feasibles, &finish_times, delay, t_g);

                self.machines[op_j.machine].reserve(finish_time_j - op_j.duration..finish_time_j);
            }
            // Update the scheduling time t_g associated with g
            t_g = *finish_times.iter().filter(|&&t| t > t_g).min().unwrap();
        }
        let makespan = usize::min(last_finish_time, self.local_search());
        self.fitness = makespan;
        self.is_fitness_valid = true;

        self.machines.iter_mut().for_each(|machine| machine.reset());
        makespan
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
        }
    }
}
