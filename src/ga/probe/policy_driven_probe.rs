use std::marker::PhantomData;

use crate::ga::{individual::IndividualTrait, Metrics};

use super::{Probe, ProbingPolicy};

/// ## PolicyDrivenProbe
///
/// Checks whether policy allows for logging and if so, delegates actual logging to wrapped probe
pub struct PolicyDrivenProbe<
    IndividualT: IndividualTrait,
    Pc: ProbingPolicy<IndividualT>,
    Pr: Probe<IndividualT>,
> {
    policy: Pc,
    probe: Pr,
    _phantom: PhantomData<IndividualT>, // FIXME: Is there a way to avoid it?
}

impl<IndividualT: IndividualTrait, Pc: ProbingPolicy<IndividualT>, Pr: Probe<IndividualT>>
    PolicyDrivenProbe<IndividualT, Pc, Pr>
{
    /// Returns new instance of [PolicyDrivenProbe]
    ///
    /// ### Arguments
    ///
    /// * `policy` - logging policy to apply
    /// * `probe` - probe used to logging
    pub fn new(policy: Pc, probe: Pr) -> Self {
        Self {
            policy,
            probe,
            _phantom: PhantomData,
        }
    }
}

impl<IndividualT: IndividualTrait, Pc: ProbingPolicy<IndividualT>, Pr: Probe<IndividualT>> Probe<IndividualT>
    for PolicyDrivenProbe<IndividualT, Pc, Pr>
{
    /// This method is called in the very beginning of genetic algorithm, even before
    /// initial population is generated.
    ///
    /// Delegates actual logging to wrapped `probe` only if `policy` returns `true`
    ///
    /// ### Arguments
    ///
    /// * `metrics` - Structure containing metrics information on genetic algorithm.
    /// See [Metrics] for reference. When running this method only `start_time`
    /// field has meaningful value.
    fn on_start(&mut self, metrics: &Metrics) {
        if self.policy.on_start(metrics) {
            self.probe.on_start(metrics);
        }
    }

    /// This method is called directly after initial populationn is created and fitness
    /// of the individuals is evaluated
    ///
    /// Delegates actual logging to wrapped `probe` only if `policy` returns `true`
    ///
    /// ### Arguments
    ///
    /// * `population` - Freshly generated population
    fn on_initial_population_created(&mut self, metrics: &Metrics, population: &[IndividualT]) {
        if self.policy.on_initial_population_created(metrics, population) {
            self.probe.on_initial_population_created(metrics, population);
        }
    }

    /// This method is called every time new best individual is found (irrespectively of generation)
    ///
    /// Delegates actual logging to wrapped `probe` only if `policy` returns `true`
    ///
    /// ### Arguments
    ///
    /// * `metrics` - Structure containing metrics information on genetic algorithm.
    /// See [Metrics] for reference.
    /// * `individual` - New best individual
    fn on_new_best(&mut self, metrics: &Metrics, individual: &IndividualT) {
        if self.policy.on_new_best(metrics, individual) {
            self.probe.on_new_best(metrics, individual);
        }
    }

    /// This method is called every time a new generation is created (but not for initial population)
    ///
    /// Delegates actual logging to wrapped `probe` only if `policy` returns `true`
    ///
    /// ### Arguments
    ///
    /// * `generation` - Newly created generation
    fn on_new_generation(&mut self, metrics: &Metrics, generation: &[IndividualT]) {
        if self.policy.on_new_generation(metrics, generation) {
            self.probe.on_new_generation(metrics, generation);
        }
    }

    /// This method is called once per generation with best individual in it
    ///
    /// Delegates actual logging to wrapped `probe` only if `policy` returns `true`
    ///
    /// ### Arguments
    ///
    /// * `metrics` - Structure containing metrics information on genetic algorithm.
    /// See [Metrics] for reference.
    /// * `individual` - Best individual in current generation
    fn on_best_fit_in_generation(&mut self, metrics: &Metrics, individual: &IndividualT) {
        if self.policy.on_best_fit_in_generation(metrics, individual) {
            self.probe.on_best_fit_in_generation(metrics, individual);
        }
    }

    /// This method is called in the very begining of algorithm's main loop
    ///
    /// Delegates actual logging to wrapped `probe` only if `policy` returns `true`
    ///
    /// ### Arguments
    ///
    /// * `metrics` - Structure containing metrics information on genetic algorithm.
    /// See [Metrics] for reference.
    fn on_iteration_start(&mut self, metrics: &Metrics) {
        if self.policy.on_iteration_start(metrics) {
            self.probe.on_iteration_start(metrics);
        }
    }

    /// This method is called in the very end of algorithm's main loop, just before
    /// termination conditions are evaluated
    ///
    /// Delegates actual logging to wrapped `probe` only if `policy` returns `true`
    ///
    /// ### Arguments
    ///
    /// * `metrics` - Structure containing metrics information on genetic algorithm.
    /// See [Metrics] for reference.
    fn on_iteration_end(&mut self, metrics: &Metrics) {
        if self.policy.on_iteration_end(metrics) {
            self.probe.on_iteration_end(metrics);
        }
    }

    /// This method is called after algorithm 's main loop is exited, just before the `run`
    /// method returns
    ///
    /// Delegates actual logging to wrapped `probe` only if `policy` returns `true`
    ///
    /// ### Arguments
    ///
    /// * `metrics` - Structure containing metrics information on genetic algorithm.
    /// See [Metrics] for reference.
    /// * `population` - Final population
    /// * `best_individual` - Best individual found by algorithm
    fn on_end(&mut self, metrics: &Metrics, population: &[IndividualT], best_individual: &IndividualT) {
        if self.policy.on_end(metrics, population, best_individual) {
            self.probe.on_end(metrics, population, best_individual);
        }
    }
}
