use std::fs::{copy, File};
use crate::ga::{Probe, Individual};
use serde::{Serialize};
use serde_json;

#[derive(Serialize)]
struct JsonRecord {
  generation_number: usize,
  best_fitness: f64,
  best_individual: Vec<f64>
}

pub struct JsonProbe {
  writer: File,
  best_fitness: f64,
  generation_number: usize,
  best_chromosome: Vec<f64>,
  records: Vec<JsonRecord>,
}

impl JsonProbe {
  pub fn new(file: String) -> JsonProbe {
    JsonProbe {
      writer: File::create(file).unwrap(),
      best_fitness: 0f64,
      generation_number: 0,
      best_chromosome: vec![],
      records: vec![]
    }
  }
}

impl Probe for JsonProbe {
  fn on_start(&mut self) {
    /* defaults to noop */
  }
  fn on_new_best(&mut self, individual: &Individual) {
    /* defaults to noop */
  }
  fn on_mutation(&mut self, before: &Individual, after: &Individual) {
    /* defaults to noop */
  }
  fn on_new_generation(&mut self, generation: &Vec<Individual>) {
    println!("Calling on new generation");
    self.records.push(JsonRecord {
      generation_number: self.generation_number,
      best_fitness: self.best_fitness,
      best_individual: self.best_chromosome.clone()
    });
  }
  fn on_best_fit_in_generation(&mut self, individual: &Individual) {
    self.best_fitness = individual.fitness;
    self.best_chromosome = individual.chromosome.clone();
  }
  fn on_iteration_start(&mut self, iteration: usize) {
    self.generation_number = iteration;
  }
  fn on_iteration_end(&mut self, iteration: usize) {
    /* defaults to noop */
  }
  fn on_end(&mut self) {
    serde_json::to_writer_pretty(&self.writer, &self.records);
  }
}