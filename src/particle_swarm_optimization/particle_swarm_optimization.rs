use std::borrow::Borrow;
use itertools::iterate;
use num::{NumCast, One};

use crate::particle_swarm_optimization::probe::console_probe::ConsoleProbe;
use crate::particle_swarm_optimization::probe::csv_probe::CsvProbe;
use crate::particle_swarm_optimization::probe::json_probe::JsonProbe;
use crate::particle_swarm_optimization::probe::probe::Probe;
use crate::particle_swarm_optimization::swarm::Swarm;


struct PSOAlgorithmCfg {
    dimensions: usize,
    lower_bound: f64,
    upper_bound: f64,
    particle_count: usize,
    inertia_weight: f64,
    cognitive_coefficient: f64,
    social_coefficient: f64,
    // function: Box<dyn Fn(&Vec<f64>) -> f64>,
    function: fn(&Vec<f64>) -> f64,
    iterations: usize,
    log_interval: usize
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
            log_interval: 10
        }
    }
}

struct PSOAlgorithm {
    config: PSOAlgorithmCfg,
    swarm: Swarm,
    probes: Vec<Box<dyn Probe>>
}

impl PSOAlgorithm {
    fn new(config: PSOAlgorithmCfg, probes: Vec<Box<dyn Probe>>) -> Self {
        let swarm = Swarm::generate(config.particle_count.clone(), config.dimensions.clone(), config.lower_bound.clone(), config.upper_bound.clone(), config.function.borrow());
        PSOAlgorithm {
            config,
            swarm,
            probes
        }
    }

    fn execute(&mut self) {
        // self.probes.on_begin(&self.swarm);
        for probe in self.probes.iter_mut() {
            probe.on_begin(&self.swarm);
        }
        for iteration in 0..self.config.iterations {
            self.swarm.update_velocities(&self.config.inertia_weight, &self.config.cognitive_coefficient, &self.config.social_coefficient);
            self.swarm.update_positions(&self.config.function);
            self.swarm.update_best_position(&self.config.function);
            if (iteration + 1) % self.config.log_interval == 0 {
                // self.probes.on_new_generation(&self.swarm, iteration + 1);
                for probe in self.probes.iter_mut() {
                    probe.on_new_generation(&self.swarm, iteration + 1);
                }
            }
        }
        // self.probes.on_end(&self.swarm);
        for probe in self.probes.iter_mut() {
            probe.on_end(&self.swarm);
        }
    }
}

fn rosenbrock(x: &Vec<f64>) -> f64 {
    let _100: f64 = NumCast::from(100).unwrap();
    let mut value: f64 = f64::default();
    for i in 0..x.len() {
        if i == x.len() - 1 {
            break;
        }
        let x_curr: f64 = x[i];
        let x_next: f64 = x[i+1];
        value += _100 * (x_next - (x_curr * x_curr)) * (x_next - (x_curr * x_curr)) + (f64::one() - x_curr)*(f64::one() - x_curr);
    }
    return value;
}

pub fn pso_demo() {
    let config = PSOAlgorithmCfg{
        dimensions: 3,
        iterations: 1000,
        log_interval: 50,
        ..PSOAlgorithmCfg::default()
    };
    let console_probe = Box::new(ConsoleProbe::new());
    let csv_probe = Box::new(CsvProbe::new(config.iterations));
    let json_probe = Box::new(JsonProbe::new(config.iterations));

    let probes : Vec<Box<dyn Probe>> = vec![console_probe, csv_probe, json_probe];

    let mut algorithm = PSOAlgorithm::new(config, probes);

    algorithm.execute();

    println!("\n\n\nRosenbrock function value at [1.0, 1.0, 1.0]: {}", rosenbrock(&vec![1.0, 1.0, 1.0]));
}