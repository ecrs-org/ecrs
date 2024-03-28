#![allow(dead_code)]
use ecrs::prelude::fitness::Fitness;
use std::collections::HashSet;

use crate::problem::{Edge, EdgeKind};

use super::individual::JsspIndividual;

pub struct JsspFitness {
    // Delay feasible operations are those operations that:
    // 1. have not yet been scheduled up to iteration g (counter defined below),
    // 2. all their predecesors have finished / will have been finished in time window t_g +
    //    delay_g (also defined below)
    // To put this in other way: all jobs that can be scheduled in time window considered in
    // given iteration g.
    delay_feasibles: Vec<usize>,

    /// The constant used to compute delay for given iteration g. The default value used in paper
    /// is 1.5.
    delay_const: f64,
}

impl JsspFitness {
    pub fn new(delay_constant: f64) -> Self {
        Self {
            delay_feasibles: Vec::new(),
            delay_const: delay_constant,
        }
    }

    pub fn evaluate_individual(&mut self, indv: &mut JsspIndividual) -> usize {
        // State is shered between indviduals & calls to this method, thus it must be resetted
        self.reset();

        if indv.is_dirty {
            indv.reset();
        }

        // Resolving problem size. -2 because zero & sink dummy operations
        let n: usize = indv.operations.len() - 2;

        // TODO: This state is not hoisted, as we do not know the number of operations
        // before first call to this function. Think of some better solution though.
        let mut finish_times = vec![usize::MAX; n + 2];

        // Schedule the dummy zero operation
        let mut scheduled_count = 1;
        finish_times[0] = 0;
        indv.operations[0].finish_time = Some(0);

        // TODO: consider starting from 0 here to make arithemtics more gracefully
        // Iteration number. Notation borrowed from the paper.
        let mut g = 1;

        // Scheduling time associated with current iteration g. This is usually equal to largest
        // schedule time form g-1 iteration + 1, so that if we do not have any operations feasible
        // to schedule with current time restriction (see the definition of delay_feasibles) we
        // relax the condition.
        let mut t_g = 0;

        // Longest duration of a single opration
        // TODO(perf): We could precompute this for each individual on population loading /
        // creation?
        let maxdur = indv.operations.iter().map(|op| op.duration).max().unwrap();

        // Id of operation with highest priority in step g. This is updated alongside computing
        // delay feasibles set.
        let mut j: usize;

        let mut last_finish_time = 0;
        while scheduled_count < n + 1 {
            // Calculate the delay. The formula is taken straight from the paper.
            // TODO: Parameterize this & conduct experiments
            let mut delay = self.delay_for_g(indv, n, g, maxdur);

            // Updating delay feasibles & finding highest priority operation from this set are
            // merged to avoid multiple iterations over whole set of operations.
            j = self.update_delay_feasibles_and_highest_prior_op(indv, &finish_times, delay, t_g);

            while !self.delay_feasibles.is_empty() {
                let op_j_duration = indv.operations[j].duration;
                let op_j_machine = indv.operations[j].machine;
                let op_j = &mut indv.operations[j];

                // Calculate the earliest finish time (in terms of precedence only)
                // We do not need to look on all predecessors. The direct one is enough, as
                // it could not be scheduled before all his preds were finished. The question is:
                // is the order of predecessors guaranteed? Look for places that manipulate this
                // field! Answer: yes it is after #444 was merged.
                // https://github.com/ecrs-org/ecrs/pull/444
                let pred_j_finish = finish_times[*op_j.preds.last().unwrap()];

                // Calculate the earliest finish time (in terms of precedence and capacity)
                let finish_time_j = finish_times
                    .iter()
                    .filter(|&&t| t != usize::MAX && t >= pred_j_finish)
                    .filter(|&&t| indv.machines[op_j_machine].is_idle(t..t + op_j_duration))
                    .min()
                    .unwrap()
                    + op_j_duration;

                // Update state
                scheduled_count += 1;
                finish_times[op_j.id] = finish_time_j;
                op_j.finish_time = Some(finish_time_j);
                g += 1;

                last_finish_time = usize::max(last_finish_time, finish_time_j);

                // ATTENTION!!!
                // There is a possibility we have a bug here.
                // It is possible that the most recently scheduled job is **not** the actual last on the
                // machine. I believe there migtht be a situation where the job is scheduled
                // before the last one. See notes on "Machine model" attached to paper.
                if let Some(last_sch_op) = indv.machines[op_j_machine].last_scheduled_op {
                    indv.operations[last_sch_op]
                        .edges_out
                        .push(Edge::new(j, EdgeKind::MachineSucc));
                    indv.operations[j].machine_pred = Some(last_sch_op);
                }

                indv.machines[op_j_machine].reserve(finish_time_j - op_j_duration..finish_time_j, j);

                if g > n {
                    break;
                }

                delay = self.delay_for_g(indv, n, g, maxdur);
                j = self.update_delay_feasibles_and_highest_prior_op(indv, &finish_times, delay, t_g);
            }
            // Update the scheduling time t_g associated with g
            t_g = *finish_times.iter().filter(|&&t| t > t_g).min().unwrap();
        }
        let makespan = usize::min(last_finish_time, self.local_search(indv));

        indv.fitness = makespan;
        indv.is_fitness_valid = true;
        indv.is_dirty = true;

        makespan
    }

    #[inline(always)]
    fn delay_for_g(&self, indv: &JsspIndividual, n: usize, g: usize, maxdur: usize) -> f64 {
        indv.chromosome[n + g - 1] * self.delay_const * (maxdur as f64)
    }

    #[inline(always)]
    pub fn op_priority(indv: &JsspIndividual, op_id: usize) -> f64 {
        // We subtract 1 as operation 0 (and sink, but it is not important here) are not taken into account in the chromosome
        indv.chromosome[op_id - 1]
    }

    fn update_delay_feasibles_and_highest_prior_op(
        &mut self,
        indv: &JsspIndividual,
        finish_times: &[usize],
        delay: f64,
        time: usize,
    ) -> usize {
        // As we are iterating over all operations, we want to make sure that the feasibles set is
        // empty before inserting anything.
        self.delay_feasibles.clear();
        let mut op_id_with_highest_priority = 0;
        let mut highest_priority = f64::MIN;

        indv.operations
            .iter()
            .filter(|op| finish_times[op.id] == usize::MAX)
            .filter(|op| {
                // It is assumed here, that dependencies are in order

                // If there is a predecessor operation -- its finish time is our earliest start
                // time ==> we want to check whether all `op` dependencies can be finished before
                // current schedule time + delay window.
                // for &pred in op.preds.iter() {
                //     if finish_times[pred] as f64 > time as f64 + delay {
                //         return false;
                //     }
                // }
                // return true;

                // We do not need to iterate over all predecessors. It is sufficient to
                // check only the direct one, because it could have been scheduled only in case its
                // own direct predecessor had finished (and so on...). However we need to handle
                // special case of sink operation as it has every every operation in it's
                // predecessor list. We do not need to handle operation zero, as it is always
                // scheduled up front, before first call to this method ==> it is filtered out
                // by previous predicate.
                // TODO(perf): Find way to get rid of this distinction. Maybe use some odditional
                // space to store only the direct predecessor (list of direct predecessors?).
                if op.id != indv.operations.len() - 1 {
                    if let Some(direct_pred_id) = op.preds.last() {
                        if finish_times[*direct_pred_id] as f64 > time as f64 + delay {
                            return false;
                        }
                    }
                } else {
                    for &pred in op.preds.iter() {
                        if finish_times[pred] as f64 > time as f64 + delay {
                            return false;
                        }
                    }
                }
                true
            })
            .for_each(|op| {
                self.delay_feasibles.push(op.id);
                if JsspFitness::op_priority(indv, op.id) > highest_priority {
                    op_id_with_highest_priority = op.id;
                    highest_priority = JsspFitness::op_priority(indv, op.id);
                }
            });
        op_id_with_highest_priority
    }

    fn local_search(&mut self, indv: &mut JsspIndividual) -> usize {
        let mut crt_sol_updated = true;
        let mut blocks: Vec<Vec<usize>> = Vec::new();
        let mut crt_makespan = usize::MAX;

        while crt_sol_updated {
            crt_sol_updated = false;
            crt_makespan = self.determine_makespan(indv);
            self.determine_critical_blocks(indv, &mut blocks);

            // Traverse along critical path
            let mut crt_block = 0;

            while crt_block < blocks.len() && !crt_sol_updated {
                let block = &blocks[crt_block];

                // Not first block
                if crt_block > 0 && block.len() >= 2 {
                    self.swap_ops(indv, block[0], block[1]);

                    let new_makespan = self.determine_makespan(indv);
                    if new_makespan < crt_makespan {
                        crt_makespan = new_makespan;
                        crt_sol_updated = true;
                    } else {
                        self.swap_ops(indv, block[1], block[0]);
                    }
                }

                // Not last block
                if crt_block != blocks.len() - 1 && !crt_sol_updated && block.len() >= 2 {
                    let last_op_id = block[block.len() - 1];
                    let sec_last_op_id = block[block.len() - 2];
                    self.swap_ops(indv, sec_last_op_id, last_op_id);

                    let new_makespan = self.determine_makespan(indv);
                    if new_makespan < crt_makespan {
                        crt_makespan = new_makespan;
                        crt_sol_updated = true;
                    } else {
                        self.swap_ops(indv, last_op_id, sec_last_op_id);
                    }
                }
                crt_block += 1;
            }
        }
        crt_makespan
    }

    fn determine_critical_path(&mut self, indv: &mut JsspIndividual) {
        let mut visited = vec![false; indv.operations.len()];
        self.calculate_critical_distance(indv, 0, &mut visited)
    }

    fn calculate_critical_distance(
        &mut self,
        indv: &mut JsspIndividual,
        op_id: usize,
        visited: &mut Vec<bool>,
    ) {
        let mut stack: Vec<usize> = Vec::with_capacity(visited.len() * 2);

        stack.push(op_id);
        while !stack.is_empty() {
            let crt_op_id = *stack.last().unwrap();
            // In current implementation it is possible (highly likely) that a vertex might be pushed
            // multiple times on the stack, before being processed, so we process the vertex iff it
            // has not been visited already.
            if !visited[crt_op_id] {
                let mut has_not_visited_neigh = false;
                for edge in indv.operations[crt_op_id].edges_out.iter() {
                    if !visited[edge.neigh_id] {
                        stack.push(edge.neigh_id);
                        has_not_visited_neigh = true;
                    }
                }

                if !has_not_visited_neigh {
                    visited[crt_op_id] = true;
                    stack.pop();

                    if !indv.operations[crt_op_id].edges_out.is_empty() {
                        let cp_edge = *indv.operations[crt_op_id]
                            .edges_out
                            .iter()
                            .max_by_key(|edge| indv.operations[edge.neigh_id].critical_distance)
                            .unwrap();

                        indv.operations[crt_op_id].critical_distance = indv.operations[crt_op_id].duration
                            + indv.operations[cp_edge.neigh_id].critical_distance;
                        indv.operations[crt_op_id].critical_path_edge = Some(cp_edge);
                    } else {
                        indv.operations[crt_op_id].critical_distance = indv.operations[crt_op_id].duration;
                    }
                }
            } else {
                stack.pop();
            }
        }
    }

    fn determine_critical_blocks(&mut self, indv: &mut JsspIndividual, blocks: &mut Vec<Vec<usize>>) {
        let mut crt_op = &indv.operations[indv.operations[0].critical_path_edge.unwrap().neigh_id];

        blocks.clear();
        blocks.push(Vec::new());
        while let Some(ref edge) = crt_op.critical_path_edge {
            blocks.last_mut().unwrap().push(crt_op.id);
            if edge.kind == EdgeKind::JobSucc {
                blocks.push(Vec::new());
            }
            crt_op = &indv.operations[edge.neigh_id];
        }
        // there should be empty block at the end
        debug_assert!(blocks.last().unwrap().is_empty());
        blocks.pop();
    }

    fn determine_makespan(&mut self, indv: &mut JsspIndividual) -> usize {
        self.determine_critical_path(indv);
        indv.operations[0].critical_distance
    }

    fn swap_ops(&mut self, indv: &mut JsspIndividual, first_op_id: usize, sec_op_id: usize) {
        // We assume few things here:
        debug_assert!(first_op_id != 0 && sec_op_id != 0);

        // Check wheter there is follow up machine element
        let block_mach_succ_opt = if let Some(Edge {
            neigh_id: block_mach_succ,
            kind: _,
        }) = indv.operations[sec_op_id].edges_out.get(1)
        {
            Some(*block_mach_succ)
        } else {
            None
        };

        if let Some(block_mach_succ) = block_mach_succ_opt {
            indv.operations[first_op_id].edges_out[1].neigh_id = block_mach_succ;
            indv.operations[block_mach_succ].machine_pred = Some(first_op_id);
            indv.operations[sec_op_id].edges_out[1].neigh_id = first_op_id;
        } else {
            indv.operations[first_op_id].edges_out.pop();
            indv.operations[sec_op_id]
                .edges_out
                .push(Edge::new(first_op_id, EdgeKind::MachineSucc));
        }

        // Check whether there is predecessor machine element
        if let Some(block_mach_pred) = indv.operations[first_op_id].machine_pred {
            indv.operations[block_mach_pred].edges_out[1].neigh_id = sec_op_id;
            indv.operations[sec_op_id].machine_pred = Some(block_mach_pred);
        } else {
            indv.operations[sec_op_id].machine_pred = None;
        }

        indv.operations[first_op_id].machine_pred = Some(sec_op_id);
    }

    #[inline]
    fn reset(&mut self) {
        self.delay_feasibles.clear();
    }
}

impl Fitness<JsspIndividual> for JsspFitness {
    #[inline]
    fn apply(&mut self, individual: &mut JsspIndividual) -> usize {
        self.evaluate_individual(individual)
    }
}
