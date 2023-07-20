use log::info;

use crate::ga::{individual::IndividualTrait, GAMetadata, Probe};

pub struct StdoutProbe;

impl StdoutProbe {
    pub fn new() -> Self {
        StdoutProbe
    }
}

impl<IndividualT: IndividualTrait> Probe<IndividualT> for StdoutProbe {
    fn on_start(&mut self, _metadata: &GAMetadata) {
        info!("[START] time,generation,chromosome,fitness");
    }

    fn on_new_best(&mut self, metadata: &GAMetadata, individual: &IndividualT) {
        info!(
            "[NEW_BEST] {},{},{:?},{}",
            metadata
                .total_dur
                .unwrap_or(std::time::Duration::from_millis(0))
                .as_millis(),
            metadata.generation,
            individual.chromosome(),
            individual.fitness()
        );
    }

    fn on_new_generation(&mut self, _metadata: &GAMetadata, _generation: &[IndividualT]) {
        // TODO: Take reference to whole generation as a parameter and display it here!
        // We don't want to print anything on new generation right now
    }

    fn on_best_fit_in_generation(&mut self, metadata: &GAMetadata, individual: &IndividualT) {
        // TODO: Take reference to the best chromosome & display it here!
        info!(
            "[BEST_IN_GEN] {},{},{:?},{}",
            metadata.total_dur.unwrap().as_millis(),
            metadata.generation,
            individual.chromosome(),
            individual.fitness()
        );
    }

    fn on_end(&mut self, metadata: &GAMetadata, _population: &[IndividualT], best_individual: &IndividualT) {
        info!(
            "[END] {},{},{:?},{}",
            metadata.total_dur.unwrap().as_millis(),
            metadata.generation,
            best_individual.chromosome(),
            best_individual.fitness()
        );
    }

    // fn on_iteration_start(&mut self, iteration: usize) {
    //   // TODO: Take iteration count & maybe some more info here (best so far, etc.)
    //   info!("Start of iteration: {}", iteration);
    // }

    // fn on_iteration_end(&mut self, iteration: usize) {
    //   // TODO: Take iteration count & maybe some more info here (best so far, etc.)
    //   info!("End of iteration: {}", iteration);
    // }
}
