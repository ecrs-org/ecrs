use crate::aco::probe::Probe;
use crate::aco::FMatrix;
use crate::aco::Solution;

/// Wrapper probe. It holds a list of probes and calls them sequentially.
///
/// This structs implements [Probe] trait and can be used with ACO
pub struct AggregatedProbe {
  probes: Vec<Box<dyn Probe>>,
}

impl AggregatedProbe {
  /// Returns new instance of [AggregatedProbe]
  pub fn new() -> Self {
    Self { probes: Vec::new() }
  }

  /// Add probe to list
  ///
  /// **Note**: Probes will be called in order they were added
  pub fn add_probe<Pr: Probe + 'static>(mut self, probe: Pr) -> Self {
    self.probes.push(Box::new(probe));
    self
  }
}

impl Probe for AggregatedProbe {
  fn on_pheromone_update(&mut self, old_pheromone: &FMatrix, new_pheromone: &FMatrix) {
    self
      .probes
      .iter_mut()
      .for_each(|p| p.on_pheromone_update(old_pheromone, new_pheromone))
  }

  fn on_current_best(&mut self, best: &Solution) {
    self.probes.iter_mut().for_each(|p| p.on_current_best(best))
  }

  fn on_iteration_start(&mut self, iteration: usize) {
    self
      .probes
      .iter_mut()
      .for_each(|p| p.on_iteration_start(iteration))
  }

  fn on_iteration_end(&mut self, iteration: usize) {
    self.probes.iter_mut().for_each(|p| p.on_iteration_end(iteration))
  }

  fn on_end(&mut self) {
    self.probes.iter_mut().for_each(|p| p.on_end())
  }
}
