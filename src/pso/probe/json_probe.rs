use crate::pso::probe::Probe;
use crate::pso::swarm::Swarm;
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Record {
    generation: usize,
    best_value: f64,
    best_position: Vec<f64>,
}

pub struct JsonProbe {
    filename: &'static str,
    records: Vec<Record>,
}

impl JsonProbe {
    pub fn new(filename: &'static str) -> JsonProbe {
        JsonProbe {
            filename,
            records: vec![],
        }
    }
}

impl Probe for JsonProbe {
    fn on_begin(&mut self, swarm: &Swarm) {
        self.on_new_generation(swarm, 0);
    }

    fn on_end(&mut self, _swarm: &Swarm, _generation: usize) {
        let mut writer = &File::create(self.filename).unwrap();
        if serde_json::to_writer_pretty(writer, &self.records).is_err() {
            eprintln!("Failed to serialize records");
        }
        if writer.flush().is_err() {
            eprintln!("Failed to save algorithm results");
        }
    }

    fn on_new_generation(&mut self, swarm: &Swarm, generation: usize) {
        self.records.push(Record {
            generation,
            best_value: swarm.best_position_value,
            best_position: swarm.best_position.clone(),
        });
    }
}
