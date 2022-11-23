use log::info;

use crate::ga::{
  individual::{Chromosome, Individual},
  GAMetadata, Probe,
};

pub struct StdoutProbe;

impl StdoutProbe {
  pub fn new() -> Self {
    StdoutProbe
  }
}

impl<T: Chromosome> Probe<T> for StdoutProbe {
  fn on_start(&mut self, _metadata: &GAMetadata) {
    info!("[START] GA");
  }

  fn on_new_best(&mut self, metadata: &GAMetadata, individual: &Individual<T>) {
    info!(
      "[NEW_BEST] {},{},{:?},{}",
      metadata.duration.unwrap().as_millis(),
      metadata.generation.unwrap(),
      individual.chromosome_ref(),
      individual.fitness
    );
  }

  fn on_new_generation(&mut self, _generation: &[Individual<T>]) {
    // TODO: Take reference to whole generation as a parameter and display it here!
    // We don't want to print anything on new generation right now
  }

  fn on_best_fit_in_generation(&mut self, metadata: &GAMetadata, individual: &Individual<T>) {
    // TODO: Take reference to the best chromosome & display it here!
    info!(
      "[BEST_IN_GEN] {},{},{:?},{}",
      metadata.duration.unwrap().as_millis(),
      metadata.generation.unwrap(),
      individual.chromosome_ref(),
      individual.fitness
    );
  }

  // fn on_iteration_start(&mut self, iteration: usize) {
  //   // TODO: Take iteration count & maybe some more info here (best so far, etc.)
  //   info!("Start of iteration: {}", iteration);
  // }

  // fn on_iteration_end(&mut self, iteration: usize) {
  //   // TODO: Take iteration count & maybe some more info here (best so far, etc.)
  //   info!("End of iteration: {}", iteration);
  // }
}
