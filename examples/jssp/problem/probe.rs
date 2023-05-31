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
    #[inline]
    fn on_start(&mut self, _metadata: &ecrs::ga::GAMetadata) {}

    fn on_initial_population_created(&mut self, population: &[JsspIndividual]) {
        let diversity = JsspProbe::estimate_pop_diversity(population);
        info!(target: "csv", "diversity,0,0,{},{diversity}", population.len());
    }

    fn on_new_best(&mut self, metadata: &ecrs::ga::GAMetadata, individual: &JsspIndividual) {
        info!(
            target: "csv",
            "newbest,{},{},{}",
            metadata.generation,
            metadata.duration.unwrap().as_millis(),
            individual.fitness
        );
    }

    fn on_new_generation(&mut self, metadata: &ecrs::ga::GAMetadata, generation: &[JsspIndividual]) {
        let diversity = JsspProbe::estimate_pop_diversity(generation);
        info!(
            target: "csv",
            "diversity,{},{},{},{diversity}",
            metadata.generation,
            metadata.duration.unwrap().as_millis(),
            generation.len()
        );
    }

    fn on_best_fit_in_generation(&mut self, metadata: &ecrs::ga::GAMetadata, individual: &JsspIndividual) {
        info!(
            target: "csv",
            "bestingen,{},{},{}",
            metadata.generation,
            metadata.duration.unwrap().as_millis(),
            individual.fitness
        );
    }

    #[inline]
    fn on_iteration_start(&mut self, _metadata: &ecrs::ga::GAMetadata) { /* defaults to noop */
    }

    #[inline]
    fn on_iteration_end(&mut self, _metadata: &ecrs::ga::GAMetadata) { /* defaults to noop */
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
