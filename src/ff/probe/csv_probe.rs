use crate::ff::probe::Probe;
use serde::Serialize;

#[derive(Serialize)]
struct Record {
  iteration: u32,
  best_value: f64,
}

pub struct CsvProbe {
  filename: &'static str,
  records: Vec<Record>,
  iteration: u32,
  best_solution: f64,
}

impl CsvProbe {
  pub fn new(filename: &'static str) -> CsvProbe {
    CsvProbe {
      filename,
      records: vec![],
      iteration: 0,
      best_solution: 0.0,
    }
  }
}

impl Probe for CsvProbe {
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
    });
  }

  fn on_current_best(&mut self, solution: f64, _position: &Vec<f64>) {
    self.best_solution = solution;
  }

  fn on_end(&mut self) {
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
}
