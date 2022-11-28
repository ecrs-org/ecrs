use super::Probe;
use crate::ga::{individual::Chromosome, GAMetadata, Individual};

pub struct AggregatedProbe<T: Chromosome> {
  probes: Vec<Box<dyn Probe<T>>>,
}

impl<T: Chromosome> AggregatedProbe<T> {
  pub fn new() -> Self {
    Self { probes: Vec::new() }
  }

  pub fn add_probe<Pr: Probe<T> + 'static>(mut self, probe: Pr) -> Self {
    self.probes.push(Box::new(probe));
    self
  }
}

impl<T: Chromosome> Probe<T> for AggregatedProbe<T> {
  fn on_start(&mut self, metadata: &GAMetadata) {
    /* defaults to noop */
    for probe in &mut self.probes {
      probe.on_start(metadata);
    }
  }

  fn on_initial_population_created(&mut self, population: &[Individual<T>]) {
    for probe in &mut self.probes {
      probe.on_initial_population_created(population);
    }
  }

  fn on_new_best(&mut self, metadata: &GAMetadata, individual: &Individual<T>) {
    for probe in &mut self.probes {
      probe.on_new_best(metadata, individual);
    }
  }

  fn on_new_generation(&mut self, metadata: &GAMetadata, generation: &[Individual<T>]) {
    /* defaults to noop */
    for probe in &mut self.probes {
      probe.on_new_generation(metadata, generation);
    }
  }

  fn on_best_fit_in_generation(&mut self, metadata: &GAMetadata, individual: &Individual<T>) {
    for probe in &mut self.probes {
      probe.on_best_fit_in_generation(metadata, individual);
    }
  }

  fn on_iteration_start(&mut self, metadata: &GAMetadata) {
    /* defaults to noop */
    for probe in &mut self.probes {
      probe.on_iteration_start(metadata);
    }
  }

  fn on_iteration_end(&mut self, metadata: &GAMetadata) {
    /* defaults to noop */
    for probe in &mut self.probes {
      probe.on_iteration_end(metadata);
    }
  }

  fn on_end(&mut self, metadata: &GAMetadata, population: &[Individual<T>], best_individual: &Individual<T>) {
    for probe in &mut self.probes {
      probe.on_end(metadata, population, best_individual);
    }
  }
}
