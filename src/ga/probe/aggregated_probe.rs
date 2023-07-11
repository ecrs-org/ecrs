use super::Probe;
use crate::ga::{individual::IndividualTrait, GAMetadata};

/// Wrapper probe. It holds a list of probes and calls them sequentially.
///
/// This structs implements [Probe] trait and can be used with GA
pub struct AggregatedProbe<IndividualT: IndividualTrait> {
    probes: Vec<Box<dyn Probe<IndividualT>>>,
}

impl<IndividualT: IndividualTrait> AggregatedProbe<IndividualT> {
    /// Returns new instance of [AggregatedProbe]
    pub fn new() -> Self {
        Self { probes: Vec::new() }
    }

    /// Add probe to list
    ///
    /// **Note**: Probes will be called in order they were added
    pub fn add_probe<Pr: Probe<IndividualT> + 'static>(mut self, probe: Pr) -> Self {
        self.probes.push(Box::new(probe));
        self
    }
}

impl<IndividualT: IndividualTrait> Probe<IndividualT> for AggregatedProbe<IndividualT> {
    /// This method is called in the very beginning of genetic algorithm, even before
    /// initial population is generated.
    ///
    /// Calls all the probes sequentially
    ///
    /// ### Arguments
    ///
    /// * `metadata` - Structure containing metadata information on genetic algorithm.
    /// See [GAMetadata] for reference. When running this method only `start_time`
    /// field has meaningful value.
    fn on_start(&mut self, metadata: &GAMetadata) {
        for probe in &mut self.probes {
            probe.on_start(metadata);
        }
    }

    /// This method is called directly after initial populationn is created and fitness
    /// of the individuals is evaluated
    ///
    /// Calls all the probes sequentially
    ///
    /// ### Arguments
    ///
    /// * `population` - Freshly generated population
    fn on_initial_population_created(&mut self, metadata: &GAMetadata, population: &[IndividualT]) {
        for probe in &mut self.probes {
            probe.on_initial_population_created(metadata, population);
        }
    }

    /// This method is called every time new best individual is found (irrespectively of generation)
    ///
    /// Calls all the probes sequentially
    ///
    /// ### Arguments
    ///
    /// * `metadata` - Structure containing metadata information on genetic algorithm.
    /// See [GAMetadata] for reference.
    /// * `individual` - New best individual
    fn on_new_best(&mut self, metadata: &GAMetadata, individual: &IndividualT) {
        for probe in &mut self.probes {
            probe.on_new_best(metadata, individual);
        }
    }

    /// This method is called every time a new generation is created (but not for initial population)
    ///
    /// Calls all the probes sequentially
    ///
    /// ### Arguments
    ///
    /// * `generation` - Newly created generation
    fn on_new_generation(&mut self, metadata: &GAMetadata, generation: &[IndividualT]) {
        /* defaults to noop */
        for probe in &mut self.probes {
            probe.on_new_generation(metadata, generation);
        }
    }

    /// This method is called once per generation with best individual in it
    ///
    /// Calls all the probes sequentially
    ///
    /// ### Arguments
    ///
    /// * `metadata` - Structure containing metadata information on genetic algorithm.
    /// See [GAMetadata] for reference.
    /// * `individual` - Best individual in current generation
    fn on_best_fit_in_generation(&mut self, metadata: &GAMetadata, individual: &IndividualT) {
        for probe in &mut self.probes {
            probe.on_best_fit_in_generation(metadata, individual);
        }
    }

    /// This method is called in the very begining of algorithm's main loop
    ///
    /// Calls all the probes sequentially
    ///
    /// ### Arguments
    ///
    /// * `metadata` - Structure containing metadata information on genetic algorithm.
    /// See [GAMetadata] for reference.
    fn on_iteration_start(&mut self, metadata: &GAMetadata) {
        /* defaults to noop */
        for probe in &mut self.probes {
            probe.on_iteration_start(metadata);
        }
    }

    /// This method is called in the very end of algorithm's main loop, just before
    /// termination conditions are evaluated
    ///
    /// Calls all the probes sequentially
    ///
    /// ### Arguments
    ///
    /// * `metadata` - Structure containing metadata information on genetic algorithm.
    /// See [GAMetadata] for reference.
    fn on_iteration_end(&mut self, metadata: &GAMetadata) {
        /* defaults to noop */
        for probe in &mut self.probes {
            probe.on_iteration_end(metadata);
        }
    }

    /// This method is called after algorithm 's main loop is exited, just before the `run`
    /// method returns
    ///
    /// Calls all the probes sequentially
    ///
    /// ### Arguments
    ///
    /// * `metadata` - Structure containing metadata information on genetic algorithm.
    /// See [GAMetadata] for reference.
    /// * `population` - Final population
    /// * `best_individual` - Best individual found by algorithm
    fn on_end(&mut self, metadata: &GAMetadata, population: &[IndividualT], best_individual: &IndividualT) {
        for probe in &mut self.probes {
            probe.on_end(metadata, population, best_individual);
        }
    }
}
