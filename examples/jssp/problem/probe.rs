use std::cmp::Ordering;

use crate::util::euclidean_distance;
use ecrs::ga::{individual::IndividualTrait, Probe};
use itertools::Itertools;
use log::info;
use md5;

use crate::logging::OutputData;

use super::individual::JsspIndividual;

pub(crate) struct JsspProbe {
    repeated: Vec<bool>,
}

impl JsspProbe {
    pub(crate) fn new() -> Self {
        // Deferring creation of vector as we do not know the required capacity
        Self { repeated: Vec::new() }
    }

    #[allow(dead_code)]
    // TODO: This has either been not working as expected or the solver runs so bad.
    // TODO: Verify whether the diversity is better on other problems
    // fn estimate_pop_diversity(&mut self, population: &[JsspIndividual]) -> f64 {
    //     population
    //         .iter()
    //         .map(|idv| (idv.chromosome().iter().product::<f64>() * 100_000f64) as usize)
    //         .unique()
    //         .count() as f64
    //         / population.len() as f64
    // }

    /// This is slow. O(N^2 * M) complexity. N - number of individuals, M - length of chromosome
    /// Consider using HyperLogLog++ algorithm
    /// See: https://en.wikipedia.org/wiki/HyperLogLog
    /// I can not really use hash set here, as f64 does not implement neither Eq nor Hash...
    /// (and it is used in chromosome...)
    // #[allow(dead_code)]
    // fn estimate_pop_diversity(&mut self, population: &[JsspIndividual]) -> f64 {
    //     self.repeated.fill(false);
    //     let mut n_unique = population.len();
    //     for i in 0..population.len() - 1 {
    //         if self.repeated[i] {
    //             continue;
    //         }
    //         for j in i + 1..population.len() {
    //             if !self.repeated[j] && population[i].chromosome.eq(population[j].chromosome()) {
    //                 n_unique -= 1;
    //                 self.repeated[j] = true;
    //             }
    //         }
    //     }
    //
    //     (n_unique as f64) / (population.len() as f64)
    // }

    #[allow(dead_code)]
    fn estimate_avg_distance(&mut self, population: &[JsspIndividual]) -> f64 {
        let mut distance_sum = 0.0;
        for i in 0..population.len() - 1 {
            for j in i + 1..population.len() {
                distance_sum += euclidean_distance(population[i].chromosome(), population[j].chromosome())
            }
        }
        // No. of hand shakes: n * (n - 1) / 2
        // This **should** work
        distance_sum / (population.len() * (population.len() - 1) / 2) as f64
    }

    #[inline]
    fn estimate_pop_diversity(&mut self, _population: &[JsspIndividual]) -> f64 {
        0.0
    }
}

impl Probe<JsspIndividual> for JsspProbe {
    // CSV OUTLINE:
    // popmetrics,<generation>,<total_duration>,<population_size>,<diversity>,<distance_avg>
    // newbest,<generation>,<total_duration>,<fitness>
    // bestingen,<generation>,<total_duration>,<fitness>
    // popgentime,<time>
    // iterinfo,<generation>,<eval_time>,<sel_time>,<cross_time>,<mut_time>,<repl_time>,<iter_time>

    #[inline]
    fn on_start(&mut self, _metadata: &ecrs::ga::GAMetadata) {
        // Writing csv header to each file
        info!(target: "popmetrics", "event_name,generation,total_duration,population_size,diversity,distance_avg");
        info!(target: "popgentime", "event_name,time");
        info!(target: "newbest", "event_name,generation,total_duration,fitness");
        info!(target: "bestingen", "event_name,generation,total_duration,fitness");
        info!(target: "iterinfo", "event_name,generation,eval_time,sel_time,cross_time,mut_time,repl_time,iter_time");
    }

    fn on_initial_population_created(
        &mut self,
        metadata: &ecrs::ga::GAMetadata,
        population: &[JsspIndividual],
    ) {
        debug_assert_eq!(self.repeated.len(), 0);
        self.repeated.resize(population.len(), false);

        // TODO: As this metric is useless right now I'm disabling it temporarily
        // let diversity = self.estimate_pop_diversity(population);
        let diversity = self.estimate_pop_diversity(population);
        let distance_avg = self.estimate_avg_distance(population);
        info!(target: "popmetrics", "diversity,0,0,{},{diversity},{distance_avg}", population.len());
        info!(target: "popgentime", "popgentime,{}", metadata.pop_gen_dur.unwrap().as_millis());
    }

    fn on_new_best(&mut self, metadata: &ecrs::ga::GAMetadata, individual: &JsspIndividual) {
        info!(
            target: "newbest",
            "newbest,{},{},{}",
            metadata.generation,
            metadata.total_dur.unwrap().as_millis(),
            individual.fitness
        );
    }

    fn on_new_generation(&mut self, metadata: &ecrs::ga::GAMetadata, generation: &[JsspIndividual]) {
        // TODO: As this metric is useless right now I'm disabling it temporarily
        // let diversity = self.estimate_pop_diversity(generation);
        let diversity = self.estimate_pop_diversity(generation);
        let distance_avg = self.estimate_avg_distance(generation);
        info!(
            target: "popmetrics",
            "diversity,{},{},{},{diversity},{distance_avg}",
            metadata.generation,
            metadata.total_dur.unwrap().as_millis(),
            generation.len()
        );
    }

    fn on_best_fit_in_generation(&mut self, metadata: &ecrs::ga::GAMetadata, individual: &JsspIndividual) {
        info!(
            target: "bestingen",
            "bestingen,{},{},{}",
            metadata.generation,
            metadata.total_dur.unwrap().as_millis(),
            individual.fitness
        );
    }

    #[inline]
    fn on_iteration_start(&mut self, _metadata: &ecrs::ga::GAMetadata) { /* defaults to noop */
    }

    #[inline]
    fn on_iteration_end(&mut self, metadata: &ecrs::ga::GAMetadata) {
        info!(target: "iterinfo", "iterinfo,{},{},{},{},{},{},{}",
            metadata.generation,
            metadata.pop_eval_dur.unwrap().as_millis(),
            metadata.selection_dur.unwrap().as_millis(),
            metadata.crossover_dur.unwrap().as_millis(),
            metadata.mutation_dur.unwrap().as_millis(),
            metadata.replacement_dur.unwrap().as_millis(),
            metadata.iteration_dur.unwrap().as_millis()
        );
    }

    #[inline]
    fn on_end(
        &mut self,
        metadata: &ecrs::ga::GAMetadata,
        _population: &[JsspIndividual],
        best_individual: &JsspIndividual,
    ) {
        let mut ops = best_individual.operations.clone();

        for op in ops.iter() {
            println!("{op:?}");
        }

        // This includes zero & sink operations
        let n = ops.len() - 2;

        #[allow(clippy::if_same_then_else)]
        ops.sort_unstable_by(|a, b| {
            if a.id == n + 1 {
                Ordering::Greater
            } else if b.id == n + 1 {
                Ordering::Less
            } else if a.finish_time.unwrap() < b.finish_time.unwrap() {
                Ordering::Less
            } else if a.finish_time.unwrap() > b.finish_time.unwrap() {
                Ordering::Greater
            } else if a.duration != 0 && b.duration != 0 {
                a.machine.cmp(&b.machine)
            } else if a.duration != 0 && b.duration == 0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        let solution_string = ops
            .into_iter()
            .filter(|op| op.id != 0 && op.id != n + 1)
            .map(|op| op.id.to_string())
            .join("_");

        let hash = md5::compute(solution_string.clone());
        let outdata = OutputData {
            solution_string,
            hash: format!("{:x}", hash),
            fitness: best_individual.fitness,
            generation_count: metadata.generation,
            total_time: metadata.total_dur.unwrap().as_millis(),
            chromosome: best_individual.chromosome(),
        };
        let serialized_object = serde_json::to_string_pretty(&outdata).unwrap();
        info!(target: "metadata", "{serialized_object}");
    }
}
