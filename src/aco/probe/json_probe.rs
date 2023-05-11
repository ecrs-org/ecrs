//! Probe implementation for writing algorithm output into a json file.
//!
//! Data is flushed only after algorithm ends.

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::fs::File;
use std::io::Write;

use crate::aco::probe::Probe;
use crate::aco::util::into_vec;
use crate::aco::FMatrix;
use crate::aco::Solution;

struct IterationData {
    pheromone: FMatrix,
    best_solution: Solution,
}

impl Default for IterationData {
    fn default() -> Self {
        IterationData {
            pheromone: FMatrix::zeros(0, 0),
            best_solution: Solution::default(),
        }
    }
}

impl Serialize for IterationData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut struct_ser = serializer.serialize_struct("Iteration", 2)?;
        let pheromone: Vec<Vec<f64>> = into_vec(&self.pheromone);
        struct_ser.serialize_field("pheromone", &pheromone)?;
        struct_ser.serialize_field("solution", &self.best_solution)?;

        struct_ser.end()
    }
}

/// Probe implementation for writing algorithm output into a json file.
///
/// Data is flushed only after algorithm ends.
pub struct JsonProbe {
    file_path: String,
    iterations: Vec<IterationData>,
}

impl JsonProbe {
    pub fn new(file_path: String) -> JsonProbe {
        JsonProbe {
            file_path,
            iterations: Vec::new(),
        }
    }

    #[doc(hidden)]
    fn flush(&mut self) {
        let mut writer = &File::create(&self.file_path).unwrap();
        if serde_json::to_writer_pretty(writer, &self.iterations).is_err() {
            eprintln!("Failed to serialize records");
        }
        if writer.flush().is_err() {
            eprintln!("Failed to save algorithm results");
        }
    }
}

impl Probe<FMatrix> for JsonProbe {
    fn on_pheromone_update(&mut self, old_pheromone: &FMatrix, _new_pheromone: &FMatrix) {
        self.iterations.last_mut().unwrap().pheromone = old_pheromone.clone()
    }

    fn on_current_best(&mut self, best: &Solution) {
        self.iterations.last_mut().unwrap().best_solution = best.clone()
    }

    fn on_iteration_start(&mut self) {
        self.iterations.push(IterationData::default())
    }

    fn on_iteration_end(&mut self) {}

    fn on_end(&mut self) {
        self.flush();
    }
}
