use crate::pso::probe::Probe;
use crate::pso::termination_condition::{GenerationLimit, TerminationCondition, TimeLimit};
use crate::pso::{PSOAlgorithm, PSOAlgorithmCfg};

pub struct PSOAlgorithmBuilder {
    config: PSOAlgorithmCfg,
}

impl PSOAlgorithmBuilder {
    pub fn new() -> Self {
        PSOAlgorithmBuilder {
            config: PSOAlgorithmCfg::default(),
        }
    }

    pub fn set_dimensions(mut self, dimensions: usize) -> Self {
        self.config.dimensions = dimensions;
        self
    }

    pub fn set_lower_bound(mut self, lower_bound: f64) -> Self {
        self.config.lower_bound = lower_bound;
        self
    }

    pub fn set_upper_bound(mut self, upper_bound: f64) -> Self {
        self.config.upper_bound = upper_bound;
        self
    }

    pub fn set_particle_count(mut self, particle_count: usize) -> Self {
        self.config.particle_count = particle_count;
        self
    }

    pub fn set_inertia_weight(mut self, inertia_weight: f64) -> Self {
        self.config.inertia_weight = inertia_weight;
        self
    }

    pub fn set_cognitive_coefficient(mut self, cognitive_coefficient: f64) -> Self {
        self.config.cognitive_coefficient = cognitive_coefficient;
        self
    }

    pub fn set_social_coefficient(mut self, social_coefficient: f64) -> Self {
        self.config.social_coefficient = social_coefficient;
        self
    }

    pub fn set_function(mut self, function: fn(&Vec<f64>) -> f64) -> Self {
        self.config.function = function;
        self
    }

    pub fn set_termination_condition(
        mut self,
        termination_condtition: Box<dyn TerminationCondition>,
    ) -> Self {
        self.config.termination_condition = termination_condtition;
        self
    }

    pub fn set_generation_limit(mut self, generation_limit: usize) -> Self {
        self.config.termination_condition = Box::new(GenerationLimit::new(generation_limit));
        self
    }

    pub fn set_time_limit(mut self, time_limit: usize) -> Self {
        self.config.termination_condition = Box::new(TimeLimit::new(time_limit));
        self
    }

    pub fn set_probe(mut self, probe: Box<dyn Probe>) -> Self {
        self.config.probe = probe;
        self
    }

    pub fn build(self) -> PSOAlgorithm {
        PSOAlgorithm::new(self.config)
    }
}
