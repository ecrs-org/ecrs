use crate::pso::probe::Probe;
use crate::pso::swarm::Swarm;

pub struct AggregatedProbe {
    probes: Vec<Box<dyn Probe>>,
}

impl AggregatedProbe {
    pub fn new() -> AggregatedProbe {
        AggregatedProbe { probes: vec![] }
    }

    pub fn from_probes(probes: Vec<Box<dyn Probe>>) -> AggregatedProbe {
        AggregatedProbe { probes }
    }

    pub fn add_probe<Pr: Probe + 'static>(mut self, probe: Pr) -> AggregatedProbe {
        self.probes.push(Box::new(probe));
        self
    }
}

impl Probe for AggregatedProbe {
    fn on_begin(&mut self, swarm: &Swarm) {
        for probe in self.probes.iter_mut() {
            probe.on_begin(swarm);
        }
    }

    fn on_end(&mut self, swarm: &Swarm, generation: usize) {
        for probe in self.probes.iter_mut() {
            probe.on_end(swarm, generation);
        }
    }

    fn on_new_generation(&mut self, swarm: &Swarm, generation: usize) {
        for probe in self.probes.iter_mut() {
            probe.on_new_generation(swarm, generation);
        }
    }
}
