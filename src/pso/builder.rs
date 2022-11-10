use crate::pso::probe::probe::Probe;
use crate::pso::{PSOAlgorithm, PSOAlgorithmCfg};

pub struct PSOAlgorithmBuilder {
    config: PSOAlgorithmCfg
}

impl PSOAlgorithmBuilder {
    pub fn new() -> Self {
        PSOAlgorithmBuilder {
            config: PSOAlgorithmCfg::default()
        }
    }

    pub fn set_dimensions(mut self, dimensions: usize) -> Self {
        self.config.dimensions = dimensions;
        return self;
    }

    pub fn set_lower_bound(mut self, lower_bound: f64) -> Self {
        self.config.lower_bound = lower_bound;
        return self;
    }

    pub fn set_upper_bound(mut self, upper_bound: f64) -> Self {
        self.config.upper_bound = upper_bound;
        return self;
    }

    pub fn set_particle_count(mut self, particle_count: usize) -> Self {
        self.config.particle_count = particle_count;
        return self;
    }

    pub fn set_inertia_weight(mut self, inertia_weight: f64) -> Self {
        self.config.inertia_weight = inertia_weight;
        return self;
    }

    pub fn set_cognitive_coefficient(mut self, cognitive_coefficient: f64) -> Self {
        self.config.cognitive_coefficient = cognitive_coefficient;
        return self;
    }

    pub fn set_social_coefficient(mut self, social_coefficient: f64) -> Self {
        self.config.social_coefficient = social_coefficient;
        return self;
    }

    pub fn set_function(mut self, function: fn(&Vec<f64>) -> f64) -> Self {
        self.config.function = function;
        return self;
    }

    pub fn set_iterations(mut self, iterations: usize) -> Self {
        self.config.iterations = iterations;
        return self;
    }

    pub fn set_probe(mut self, probe: Box<dyn Probe>) -> Self {
        self.config.probe = probe;
        return self;
    }

    pub fn build(self) -> PSOAlgorithm {
        return PSOAlgorithm::new(self.config);
    }
}