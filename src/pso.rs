use std::borrow::Borrow;

pub mod particle;
pub mod probe;
pub mod swarm;
pub mod util;
pub mod builder;

use crate::pso::probe::console_probe::ConsoleProbe;
use crate::pso::probe::Probe;
use crate::pso::swarm::Swarm;
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
///  - iterations - number of iterations, the algorithm should run for
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
    iterations: usize,
    probe: Box<dyn Probe>
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
            iterations: 500,
            probe:Box::new(ConsoleProbe::new())
        }
    }
}

/// Struct used for running PSO algorithm
/// # Usage example:
/// ```rust
/// # use ecrs::pso::{PSOAlgorithmCfg, PSOAlgorithm};
/// # use ecrs::pso::probe::console_probe::ConsoleProbe;
///
/// let config = PSOAlgorithmCfg {
///     dimensions: 3,
///     iterations: 1000,
///     probe: Box::new(ConsoleProbe::new()),
///     ..PSOAlgorithmCfg::default()
/// };
///
/// let mut algorithm = PSOAlgorithm::new(config);
///
/// algorithm.execute();
/// ```
pub struct PSOAlgorithm {
    config: PSOAlgorithmCfg,
    swarm: Swarm
}

impl PSOAlgorithm {
    fn new(config: PSOAlgorithmCfg) -> Self {
        let swarm = Swarm::generate(config.particle_count, config.dimensions, config.lower_bound, config.upper_bound, config.function.borrow());
        PSOAlgorithm {
            config,
            swarm
        }
    }

    pub fn run(&mut self) {
        self.config.probe.on_begin(&self.swarm);
        for iteration in 0..self.config.iterations {
            self.swarm.update_velocities(&self.config.inertia_weight, &self.config.cognitive_coefficient, &self.config.social_coefficient);
            self.swarm.update_positions(self.config.function);
            self.swarm.update_best_position(self.config.function);
            self.config.probe.on_new_generation(&self.swarm, iteration + 1);
        }
        self.config.probe.on_end(&self.swarm);
    }
}
