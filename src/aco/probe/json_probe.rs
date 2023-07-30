//! Probe implementation for writing algorithm output into a json file.
//!
//! Data is flushed only after algorithm ends.

use itertools::Itertools;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::fs::File;
use std::io::Write;

use crate::aco::probe::Probe;
use crate::aco::Solution;
use crate::aco::{AdditionalArgs, FMatrix};

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

impl<Args: AdditionalArgs> Probe<FMatrix, Args> for JsonProbe {
    fn on_pheromone_update(&mut self, new_pheromone: &FMatrix, _: &Args) {
        self.iterations.last_mut().unwrap().pheromone = new_pheromone.clone()
    }

    fn on_current_best(&mut self, best: &Solution, _: &Args) {
        self.iterations.last_mut().unwrap().best_solution = best.clone()
    }

    fn on_iteration_start(&mut self, _: &Args) {
        self.iterations.push(IterationData::default())
    }

    fn on_iteration_end(&mut self, _: &Args) {}

    fn on_end(&mut self, _: &Args) {
        self.flush();
    }
}

pub fn into_vec(m: &FMatrix) -> Vec<Vec<f64>> {
    let mut m_vec: Vec<Vec<f64>> = Vec::new();

    for row in m.row_iter() {
        m_vec.push(row.iter().copied().collect_vec());
    }
    m_vec
}
