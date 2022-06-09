use crate::ga::{Probe, Individual};

pub struct JsonProbe {

}

impl Probe for JsonProbe {
  fn on_start(&mut self) {

  }
  fn on_new_best(&mut self, individual: &Individual) {

  }
  fn on_mutation(&mut self, before: &Individual, after: &Individual) {

  }
  fn on_new_generation(&mut self) {

  }
  fn on_best_fit_in_generation(&mut self, individual: &Individual) {

  }
  fn on_iteration_start(&mut self, iteration: usize) {

  }
  fn on_iteration_end(&mut self, iteration: usize) {

  }
}