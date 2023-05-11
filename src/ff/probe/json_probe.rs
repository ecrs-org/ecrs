use crate::ff::probe::Probe;
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Record {
    iteration: u32,
    best_value: f64,
    best_position: Vec<f64>,
}

pub struct JsonProbe {
    filename: &'static str,
    records: Vec<Record>,
    iteration: u32,
    best_solution: f64,
    best_solution_position: Vec<f64>,
}

impl JsonProbe {
    pub fn new(filename: &'static str) -> JsonProbe {
        JsonProbe {
            filename,
            records: vec![],
            iteration: 0,
            best_solution: 0.0,
            best_solution_position: vec![],
        }
    }
}

impl Probe for JsonProbe {
    fn on_start(&mut self) {
        /* noop */
    }

    fn on_iteration_start(&mut self, iteration: u32) {
        self.iteration = iteration;
    }

    fn on_iteration_end(&mut self, _iteration: u32) {
        self.records.push(Record {
            iteration: self.iteration,
            best_value: self.best_solution,
            best_position: self.best_solution_position.clone(),
        });
    }

    fn on_current_best(&mut self, solution: f64, position: &[f64]) {
        self.best_solution = solution;
        self.best_solution_position = position.to_vec();
    }

    fn on_end(&mut self) {
        let mut writer = &File::create(self.filename).unwrap();

        if serde_json::to_writer_pretty(writer, &self.records).is_err() {
            eprintln!("Failed to serialize records");
        }

        if writer.flush().is_err() {
            eprintln!("Failed to save algorithm results");
        }
    }
}
