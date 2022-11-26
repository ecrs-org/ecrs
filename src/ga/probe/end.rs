use log::info;

use crate::ga::individual::Chromosome;

use super::Probe;

pub struct EndProbe;

impl EndProbe {
  pub fn new() -> Self {
    Self
  }
}

impl<T: Chromosome> Probe<T> for EndProbe {
  fn on_end(
    &mut self,
    metadata: &crate::ga::GAMetadata,
    _population: &[crate::ga::Individual<T>],
    best_individual: &crate::ga::Individual<T>,
  ) {
    /* defaults to noop */
    info!(
      "[END] {},{},{:?},{}",
      metadata.duration.unwrap().as_millis(),
      metadata.generation.unwrap(),
      best_individual.chromosome_ref(),
      best_individual.fitness
    );
  }
}
