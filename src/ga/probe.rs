use super::{individual::IndividualTrait, GAMetadata};

mod aggregated_probe;
mod csv_probe;
mod empty;
mod json_probe;
mod policy_driven_probe;
mod probing_policy;
mod stdout_probe;

pub use aggregated_probe::AggregatedProbe;
pub use csv_probe::CsvProbe;
pub use empty::EmptyProbe;
pub use json_probe::JsonProbe;
pub use policy_driven_probe::PolicyDrivenProbe;
pub use probing_policy::{ElapsedTime, GenerationInterval};
pub use stdout_probe::StdoutProbe;

/// Thit trait specifies common behaviour for probes that can be used with GA
///
/// You can implement this trait to define your custom probe.
pub trait Probe<IndividualT: IndividualTrait> {
    /// This method is called in the very beginning of genetic algorithm, even before
    /// initial population is generated.
    ///
    /// Default implementation does nothing
    ///
    /// ### Arguments
    ///
    /// * `metadata` - Structure containing metadata information on genetic algorithm.
    /// See [GAMetadata] for reference. When running this method only `start_time`
    /// field has meaningful value.
    fn on_start(&mut self, _metadata: &GAMetadata) { /* defaults to noop */
    }

    /// This method is called directly after initial populationn is created and fitness
    /// of the individuals is evaluated
    ///
    /// Default implementation does nothing
    ///
    /// ### Arguments
    ///
    /// * `population` - Freshly generated population
    fn on_initial_population_created(&mut self, _metadata: &GAMetadata, _population: &[IndividualT]) {
        /* defaults to noop */
    }

    /// This method is called every time new best individual is found (irrespectively of generation)
    ///
    /// Default implementation does nothing
    ///
    /// ### Arguments
    ///
    /// * `metadata` - Structure containing metadata information on genetic algorithm.
    /// See [GAMetadata] for reference.
    /// * `individual` - New best individual
    fn on_new_best(&mut self, _metadata: &GAMetadata, _individual: &IndividualT) {
        /* defaults to noop */
    }

    /// This method is called every time a new generation is created (but not for initial population)
    ///
    /// Default implementation does nothing
    ///
    /// ### Arguments
    ///
    /// * `metadata` - Structure containing metadata information on genetic algorithm.
    /// See [GAMetadata] for reference.
    /// * `generation` - Newly created generation
    fn on_new_generation(&mut self, _metadata: &GAMetadata, _generation: &[IndividualT]) {
        /* defaults to noop */
    }

    /// This method is called once per generation with best individual in it
    ///
    /// Default implementation does nothing
    ///
    /// ### Arguments
    ///
    /// * `metadata` - Structure containing metadata information on genetic algorithm.
    /// See [GAMetadata] for reference.
    /// * `individual` - Best individual in current generation
    fn on_best_fit_in_generation(&mut self, _metadata: &GAMetadata, _individual: &IndividualT) {
        /* defaults to noop */
    }

    /// This method is called in the very begining of algorithm's main loop
    ///
    /// Default implementation does nothing
    ///
    /// ### Arguments
    ///
    /// * `metadata` - Structure containing metadata information on genetic algorithm.
    /// See [GAMetadata] for reference.
    fn on_iteration_start(&mut self, _metadata: &GAMetadata) { /* defaults to noop */
    }

    /// This method is called in the very end of algorithm's main loop, just before
    /// termination conditions are evaluated
    ///
    /// ### Arguments
    ///
    /// * `metadata` - Structure containing metadata information on genetic algorithm.
    /// See [GAMetadata] for reference.
    fn on_iteration_end(&mut self, _metadata: &GAMetadata) { /* defaults to noop */
    }

    /// This method is called after algorithm 's main loop is exited, just before the `run`
    /// method returns
    ///
    /// Default implementation does nothing
    ///
    /// ### Arguments
    ///
    /// * `metadata` - Structure containing metadata information on genetic algorithm.
    /// See [GAMetadata] for reference.
    /// * `population` - Final population
    /// * `best_individual` - Best individual found by algorithm
    fn on_end(
        &mut self,
        _metadata: &GAMetadata,
        _population: &[IndividualT],
        _best_individual: &IndividualT,
    ) { /* defaults to noop */
    }
}

/// Probing policy can be used with [PolicyDrivenProbe] to control when actual logging probe
/// is notified of given event
///
/// This trait defines a series of method mirroring `Probe` trait, except the fact that all
/// the methods return a boolean indicating whether to log or not
///
/// You can implement this trait to define your custom logggin policy.
///
/// ### Example
///
/// Probing policy to print only on even iterations
///
/// ```
/// # use ecrs::ga::individual::IndividualTrait;
/// # use ecrs::ga::GAMetadata;
/// # use ecrs::ga::probe::ProbingPolicy;
///
/// struct EvenIteration;
///
/// impl<IndividualT: IndividualTrait> ProbingPolicy<IndividualT> for EvenIteration {
///   fn on_start(&mut self, _metadata: &GAMetadata) -> bool {
///     // We want to always log on start
///     true
///   }
///
///   fn on_initial_population_created(&mut self, _population: &[IndividualT]) -> bool {
///     // We want to log initial population
///     true
///   }
///
///   fn on_new_best(&mut self, _metadata: &GAMetadata, _individual: &IndividualT) -> bool {
///     // We want to see when algorithm improves
///     true
///   }
///
///   fn on_new_generation(&mut self, metadata: &GAMetadata, _generation: &[IndividualT]) -> bool {
///     // Only on even iterations
///     metadata.generation % 2 == 0
///   }
///
///   fn on_best_fit_in_generation(&mut self, metadata: &GAMetadata, _individual: &IndividualT) -> bool {
///     // Only on even iterations
///     metadata.generation % 2 == 0
///   }
///
///   fn on_iteration_start(&mut self, metadata: &GAMetadata) -> bool {
///     metadata.generation % 2 == 0
///   }
///
///   fn on_iteration_end(&mut self, metadata: &GAMetadata) -> bool {
///     metadata.generation % 2 == 0
///   }
///
///   fn on_end(
///     &mut self,
///     _metadata: &GAMetadata,
///     _population: &[IndividualT],
///     _best_individual: &IndividualT,
///   ) -> bool {
///     // We want to see the end result
///     true
///   }
/// }
/// ```
///
/// Later you can use it with [PolicyDrivenProbe]
pub trait ProbingPolicy<IndividualT: IndividualTrait> {
    fn on_start(&mut self, _metadata: &GAMetadata) -> bool;
    fn on_initial_population_created(&mut self, _metadata: &GAMetadata, _population: &[IndividualT]) -> bool;
    fn on_new_best(&mut self, _metadata: &GAMetadata, _individual: &IndividualT) -> bool;
    fn on_new_generation(&mut self, _metadata: &GAMetadata, _generation: &[IndividualT]) -> bool;
    fn on_best_fit_in_generation(&mut self, _metadata: &GAMetadata, _individual: &IndividualT) -> bool;
    fn on_iteration_start(&mut self, _metadata: &GAMetadata) -> bool;
    fn on_iteration_end(&mut self, _metadata: &GAMetadata) -> bool;
    fn on_end(
        &mut self,
        _metadata: &GAMetadata,
        _population: &[IndividualT],
        _best_individual: &IndividualT,
    ) -> bool;
}
