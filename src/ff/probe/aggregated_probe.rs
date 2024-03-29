use crate::ff::probe::Probe;

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
    fn on_start(&mut self) {
        for probe in self.probes.iter_mut() {
            probe.on_start();
        }
    }

    fn on_iteration_start(&mut self, iteration: u32) {
        for probe in self.probes.iter_mut() {
            probe.on_iteration_start(iteration);
        }
    }

    fn on_iteration_end(&mut self, iteration: u32) {
        for probe in self.probes.iter_mut() {
            probe.on_iteration_end(iteration);
        }
    }

    fn on_current_best(&mut self, solution: f64, position: &[f64]) {
        for probe in self.probes.iter_mut() {
            probe.on_current_best(solution, position);
        }
    }

    fn on_end(&mut self) {
        for probe in self.probes.iter_mut() {
            probe.on_end();
        }
    }
}
