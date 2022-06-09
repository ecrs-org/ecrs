use std::fs::File;
use crate::ga::{Probe, Individual};

use serde::{Serialize};

#[derive(Serialize)]
struct CsvRecord {
  generation_number: usize,
  best_fitness: f64,
}

pub struct CsvProbe {
  writer: csv::Writer<File>,
  best_fitness: f64,
  generation_number: usize,
}

impl CsvProbe {
  pub fn new(file: String) -> CsvProbe {
    CsvProbe {
      writer: csv::WriterBuilder::new().from_path(file).unwrap(),
      best_fitness: 0f64,
      generation_number: 0
    }
  }
}

impl Probe for CsvProbe {
  fn on_start(&mut self) {
    /* noop */
  }
  fn on_new_best(&mut self, individual: &Individual) {
    /* noop */
  }
  fn on_mutation(&mut self, before: &Individual, after: &Individual) {
    /* noop */
  }
  fn on_new_generation(&mut self, generation: &Vec<Individual>) {
    let record = CsvRecord {
      generation_number: self.generation_number, // TODO: Take it as arg or save context in other methods
      best_fitness: self.best_fitness, // TODO: take it as arg or save context in other methods
    };

    self.writer.serialize(record);
    self.writer.flush();
  }
  fn on_best_fit_in_generation(&mut self, individual: &Individual) {
    self.best_fitness = individual.fitness;
  }
  fn on_iteration_start(&mut self, iteration: usize) {
    self.generation_number = iteration;
  }
  fn on_iteration_end(&mut self, iteration: usize) {
  }
}
