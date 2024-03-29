//! Implementation of PSO algorithm

pub mod builder;
pub mod particle;
pub mod probe;
pub mod swarm;
pub mod termination_condition;
pub mod util;

use crate::pso::probe::stdout_probe::StdoutProbe;
use crate::pso::probe::Probe;
use crate::pso::swarm::Swarm;
use crate::pso::termination_condition::{GenerationLimit, TerminationCondition};

use crate::test_functions::rosenbrock;

/// Struct wrapping all configuration parameters of PSO algorithm.
/// # Parameters:
///  - dimensions - number of dimension of optimized function's domain
///  - lower_bound - lower bound of search area in every dimension of the domain
///  - upper_bound - upper bound of search area in every dimension of the domain
///  - particle_count - number of particles to use in optimization (number of particles will be maintained throughout the algorithm's run)
///  - inertia_weight - specifies how much particles retain their speed from previous iteration (0 - no speed retention, 1 - no slowdown)
///  - cognitive_coefficient - specifies how much particles are attracted their own best positions
///  - social_coefficient - specifies how much particles are attracted to entire swarm's best position
///  - function - function to be optimized
///  - termination_condition - used for determining stopping point of the algorithm
///  - probe - used for displaying results / progress of the algorithm
/// # Example coefficient values:
///  - inertia_weight: 0.5
///  - cognitive_coefficient: 1.0
///  - social_coefficient: 3.0
pub struct PSOAlgorithmCfg {
    dimensions: usize,
    lower_bound: f64,
    upper_bound: f64,
    particle_count: usize,
    inertia_weight: f64,
    cognitive_coefficient: f64,
    social_coefficient: f64,
    function: fn(&Vec<f64>) -> f64,
    termination_condition: Box<dyn TerminationCondition>,
    probe: Box<dyn Probe>,
}

impl Default for PSOAlgorithmCfg {
    fn default() -> Self {
        PSOAlgorithmCfg {
            dimensions: 2,
            lower_bound: -10.0,
            upper_bound: 10.0,
            particle_count: 30,
            inertia_weight: 0.5,
            cognitive_coefficient: 1.0,
            social_coefficient: 3.0,
            function: rosenbrock,
            termination_condition: Box::new(GenerationLimit::new(500)),
            probe: Box::new(StdoutProbe::new()),
        }
    }
}

/// Struct used for running PSO algorithm
/// # Usage example:
/// ```rust
/// # use ecrs::pso::{builder::PSOAlgorithmBuilder, self};
/// let iterations = 50; // use more reasonable number here
/// let stdout_probe = Box::new(pso::probe::stdout_probe::StdoutProbe::new());
/// let csv_probe = Box::new(pso::probe::csv_probe::CsvProbe::new("pso_example.csv"));
/// let json_probe = Box::new(pso::probe::json_probe::JsonProbe::new("pso_example.json"));
/// let probes: Vec<Box<dyn pso::probe::Probe>> = vec![stdout_probe, csv_probe, json_probe];
/// let aggregated_probe = Box::new(pso::probe::aggregated_probe::AggregatedProbe::from_probes(probes));
/// let probing_policy = Box::new(pso::probe::probing_policy::GenerationInterval::new(50));
/// let policy_driven_probe = Box::new(pso::probe::policy_driven_probe::PolicyDrivenProbe::new(aggregated_probe, probing_policy));
/// let mut algorithm = PSOAlgorithmBuilder::new()
///     .set_dimensions(3)
///     .set_generation_limit(iterations)
///     .set_probe(policy_driven_probe)
///     .build();
/// algorithm.run();
/// ```
pub struct PSOAlgorithm {
    config: PSOAlgorithmCfg,
    swarm: Swarm,
}

impl PSOAlgorithm {
    pub fn new(config: PSOAlgorithmCfg) -> Self {
        let swarm = Swarm::generate(
            config.particle_count,
            config.dimensions,
            config.lower_bound,
            config.upper_bound,
            config.function,
        );
        PSOAlgorithm { config, swarm }
    }

    pub fn run(&mut self) {
        self.config.probe.on_begin(&self.swarm);
        let mut iteration = 0;
        while !self.config.termination_condition.is_met(iteration, &self.swarm) {
            iteration += 1;
            self.swarm.update_velocities(
                &self.config.inertia_weight,
                &self.config.cognitive_coefficient,
                &self.config.social_coefficient,
            );
            self.swarm.update_positions(self.config.function);
            self.swarm.update_best_position(self.config.function);
            self.config.probe.on_new_generation(&self.swarm, iteration);
        }
        self.config.probe.on_end(&self.swarm, iteration);
    }
}
