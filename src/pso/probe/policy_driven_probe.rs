use crate::pso::probe::{Probe, ProbingPolicy};
use crate::pso::swarm::Swarm;

pub struct PolicyDrivenProbe {
    probe: Box<dyn Probe>,
    policy: Box<dyn ProbingPolicy>,
}

impl PolicyDrivenProbe {
    pub fn new(probe: Box<dyn Probe>, policy: Box<dyn ProbingPolicy>) -> PolicyDrivenProbe {
        PolicyDrivenProbe { probe, policy }
    }
}

impl Probe for PolicyDrivenProbe {
    fn on_begin(&mut self, swarm: &Swarm) {
        if self.policy.on_begin() {
            self.probe.on_begin(swarm);
        }
    }

    fn on_end(&mut self, swarm: &Swarm, generation: usize) {
        if self.policy.on_end(generation) {
            self.probe.on_new_generation(swarm, generation);
        }
        self.probe.on_end(swarm, generation);
    }

    fn on_new_generation(&mut self, swarm: &Swarm, generation: usize) {
        if self.policy.on_new_generation(generation) {
            self.probe.on_new_generation(swarm, generation);
        }
    }
}
