use crate::pso::probe::probe::Probe;
use crate::pso::swarm::Swarm;

pub struct MultiProbe{
    probes: Vec<Box<dyn Probe>>
}

impl MultiProbe {
    pub fn new(probes: Vec<Box<dyn Probe>>) -> MultiProbe {
        MultiProbe {
            probes
        }
    }
}

impl Probe for MultiProbe {
    fn on_begin(&mut self, swarm: &Swarm) {
        for probe in self.probes.iter_mut() {
            probe.on_begin(swarm);
        }
    }

    fn on_end(&mut self, swarm: &Swarm) {
        for probe in self.probes.iter_mut() {
            probe.on_end(swarm);
        }
    }

    fn on_new_generation(&mut self, swarm: &Swarm, generation: usize) {
        for probe in self.probes.iter_mut() {
            probe.on_new_generation(swarm, generation);
        }
    }
}
