use crate::aco::pheromone::Pheromone;
use crate::aco::probe::Probe;
use crate::aco::Solution;

/// Wrapper probe. It holds a list of probes and calls them sequentially.
///
/// This structs implements [Probe] trait and can be used with ACO
pub struct AggregatedProbe<P: Pheromone> {
    probes: Vec<Box<dyn Probe<P>>>,
}

impl<P: Pheromone> AggregatedProbe<P> {
    /// Returns new instance of [AggregatedProbe]
    pub fn new() -> Self {
        Self { probes: Vec::new() }
    }

    /// Add probe to list
    ///
    /// **Note**: Probes will be called in order they were added
    pub fn add_probe<Pr: Probe<P> + 'static>(mut self, probe: Pr) -> Self {
        self.probes.push(Box::new(probe));
        self
    }
}

impl<P: Pheromone> Probe<P> for AggregatedProbe<P> {
    fn on_pheromone_update(&mut self, new_pheromone: &P) {
        self.probes
            .iter_mut()
            .for_each(|p| p.on_pheromone_update(new_pheromone))
    }

    fn on_current_best(&mut self, best: &Solution) {
        self.probes.iter_mut().for_each(|p| p.on_current_best(best))
    }

    fn on_iteration_start(&mut self) {
        self.probes.iter_mut().for_each(|p| p.on_iteration_start())
    }

    fn on_iteration_end(&mut self) {
        self.probes.iter_mut().for_each(|p| p.on_iteration_end())
    }

    fn on_end(&mut self) {
        self.probes.iter_mut().for_each(|p| p.on_end())
    }
}
