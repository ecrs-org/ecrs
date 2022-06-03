use crate::pso::probe::probe::Probe;
use crate::pso::swarm::Swarm;
use serde::{Serialize};
use serde_json;
use std::fs::File;
use std::io::Write;


#[derive(Serialize)]
struct Record {
    generation: usize,
    best_value: f64,
    best_position: Vec<f64>
}

pub struct JsonProbe{
    filename: &'static str,
    records: Vec<Record>,
    generations: usize,
    last_generation: usize
}

impl JsonProbe {
    pub fn new(filename: &'static str, generations: usize) -> JsonProbe {
        JsonProbe {
            filename,
            records: vec![],
            generations,
            last_generation: 0
        }
    }
}

impl Probe for JsonProbe {
    fn on_begin(&mut self, swarm: &Swarm) {
        self.on_new_generation(swarm, 0);
    }

    fn on_end(&mut self, swarm: &Swarm) {
        if self.generations > self.last_generation {
            self.on_new_generation(swarm, self.generations);
        }

        {
            let mut writer = &File::create(&self.filename).unwrap();
            serde_json::to_writer_pretty(writer, &self.records);
        }
    }

    fn on_new_generation(&mut self, swarm: &Swarm, generation: usize) {
        self.last_generation = generation;
        self.records.push(Record{
            generation,
            best_value: swarm.best_position_value,
            best_position: swarm.best_position.clone()
        });
    }
}