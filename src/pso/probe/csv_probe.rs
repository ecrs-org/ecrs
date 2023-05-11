use crate::pso::probe::Probe;
use crate::pso::swarm::Swarm;
use serde::Serialize;

#[derive(Serialize)]
struct Record {
    generation: usize,
    best_value: f64,
}

pub struct CsvProbe {
    filename: &'static str,
    records: Vec<Record>,
}

impl CsvProbe {
    pub fn new(filename: &'static str) -> CsvProbe {
        CsvProbe {
            filename,
            records: vec![],
        }
    }
}

impl Probe for CsvProbe {
    fn on_begin(&mut self, swarm: &Swarm) {
        self.on_new_generation(swarm, 0);
    }

    fn on_end(&mut self, _swarm: &Swarm, _generation: usize) {
        let mut writer = csv::WriterBuilder::new().from_path(self.filename).unwrap();
        for record in self.records.iter() {
            if writer.serialize(record).is_err() {
                eprintln!("Failed to serialize a record");
            }
        }
        if writer.flush().is_err() {
            eprintln!("Failed to save algorithm results");
        }
    }

    fn on_new_generation(&mut self, swarm: &Swarm, generation: usize) {
        self.records.push(Record {
            generation,
            best_value: swarm.best_position_value,
        });
    }
}
