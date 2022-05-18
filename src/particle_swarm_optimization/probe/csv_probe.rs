use crate::particle_swarm_optimization::probe::probe::Probe;
use crate::particle_swarm_optimization::swarm::Swarm;
use serde::{Serialize};


#[derive(Serialize)]
struct Record {
    generation: usize,
    best_value: f64
}

pub struct CsvProbe{
    records: Vec<Record>,
    generations: usize,
    last_generation: usize
}

impl CsvProbe {
    pub fn new(generations: usize) -> CsvProbe {
        CsvProbe {
            records: vec![],
            generations,
            last_generation: 0
        }
    }
}

impl Probe for CsvProbe {
    fn on_begin(&mut self, swarm: &Swarm) {
        self.on_new_generation(swarm, 0);
    }

    fn on_end(&mut self, swarm: &Swarm) {
        if self.generations > self.last_generation {
            self.on_new_generation(swarm, self.generations);
        }

        let mut writer = csv::WriterBuilder::new().from_path("testing.csv").unwrap();
        for record in self.records.iter() {
            writer.serialize(record);
        }
        writer.flush();
    }

    fn on_new_generation(&mut self, swarm: &Swarm, generation: usize) {
        self.last_generation = generation;
        self.records.push(Record{
            generation,
            best_value: swarm.best_position_value
        });
    }
}