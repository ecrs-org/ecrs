use ecrs::ga::{individual::IndividualTrait, Probe};
use itertools::Itertools;
use log::info;

use super::individual::JsspIndividual;

pub(crate) struct JsspProbe {}

impl JsspProbe {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn estimate_pop_diversity(population: &[JsspIndividual]) -> f64 {
        population
            .iter()
            .map(|idv| (idv.chromosome().iter().product::<f64>() * 100_000f64) as usize)
            .unique()
            .count() as f64
            / population.len() as f64
    }
}

impl Probe<JsspIndividual> for JsspProbe {
    // CSV OUTLINE:
    // diversity,<generation>,<total_duration>,<population_size>,<diversity>
    // newbest,<generation>,<total_duration>,<fitness>
    // bestingen,<generation>,<total_duration>,<fitness>
    // popgentime,<time>
    // iterinfo,<generation>,<eval_time>,<sel_time>,<cross_time>,<mut_time>,<repl_time>,<iter_time>

    #[inline]
    fn on_start(&mut self, _metadata: &ecrs::ga::GAMetadata) {
        // This is a marker record for ECDataKit. Since it looks like
        // polars.DataFrame.read_csv deduces number of columns from the first encoutered
        // record it leads to crashes when longer records are encountered deeper in the file.
        info!(target: "csv", "event,,,,,,,");
    }

    fn on_initial_population_created(
        &mut self,
        metadata: &ecrs::ga::GAMetadata,
        population: &[JsspIndividual],
    ) {
        let diversity = JsspProbe::estimate_pop_diversity(population);
        info!(target: "csv", "diversity,0,0,{},{diversity}\npopgentime,{}", population.len(), metadata.pop_gen_dur.unwrap().as_millis());
    }

    fn on_new_best(&mut self, metadata: &ecrs::ga::GAMetadata, individual: &JsspIndividual) {
        info!(
            target: "csv",
            "newbest,{},{},{}",
            metadata.generation,
            metadata.total_dur.unwrap().as_millis(),
            individual.fitness
        );
    }

    fn on_new_generation(&mut self, metadata: &ecrs::ga::GAMetadata, generation: &[JsspIndividual]) {
        let diversity = JsspProbe::estimate_pop_diversity(generation);
        info!(
            target: "csv",
            "diversity,{},{},{},{diversity}",
            metadata.generation,
            metadata.total_dur.unwrap().as_millis(),
            generation.len()
        );
    }

    fn on_best_fit_in_generation(&mut self, metadata: &ecrs::ga::GAMetadata, individual: &JsspIndividual) {
        info!(
            target: "csv",
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
        info!(target: "csv", "iterinfo,{},{},{},{},{},{},{}",
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
        _metadata: &ecrs::ga::GAMetadata,
        _population: &[JsspIndividual],
        _best_individual: &JsspIndividual,
    ) { /* defaults to noop */
    }
}
