use itertools::Itertools;
use serde::Serialize;

use crate::aco::probe::Probe;
use crate::aco::Solution;
use crate::aco::{AdditionalArgs, FMatrix};

#[derive(Serialize)]
#[doc(hidden)]
struct BestSolutionRecord {
    from: usize,
    to: usize,
    iter: usize,
}

#[derive(Serialize)]
#[doc(hidden)]
struct FMatrixRecord {
    from: usize,
    to: usize,
    iter: usize,
    val: f64,
}

/// Probe implementation for writing algorithm output into a csv file.
///
/// Data is flushed only after algorithm ends.
pub struct CsvProbe {
    iteration: usize,
    best_sols: Vec<BestSolutionRecord>,
    pher: Vec<FMatrixRecord>,
    best_sol: Solution,
}

impl CsvProbe {
    pub fn new() -> CsvProbe {
        CsvProbe {
            iteration: 0,
            best_sols: vec![],
            pher: vec![],
            best_sol: Solution::default(),
        }
    }

    #[doc(hidden)]
    fn flush(&mut self) {
        let mut wrt = csv::WriterBuilder::new().from_path("best.csv").unwrap();

        for record in self.best_sols.iter() {
            wrt.serialize(record).expect("Could not serialize record");
        }
        wrt.flush().expect("Could not flush data");

        let mut wrt = csv::WriterBuilder::new().from_path("pheromone.csv").unwrap();

        for record in self.pher.iter() {
            wrt.serialize(record).expect("Could not serialize record");
        }
        wrt.flush().expect("Could not flush data");
    }
}

impl<Args: AdditionalArgs> Probe<FMatrix, Args> for CsvProbe {
    fn on_pheromone_update(&mut self, new_pheromone: &FMatrix, _: &Args) {
        for (i, row) in new_pheromone.row_iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                self.pher.push(FMatrixRecord {
                    from: i,
                    to: j,
                    iter: self.iteration,
                    val: *val,
                })
            }
        }
    }

    fn on_current_best(&mut self, best: &Solution, _: &Args) {
        self.best_sol = best.clone();
    }

    fn on_iteration_start(&mut self, _: &Args) {
        self.iteration += 1;
    }

    fn on_iteration_end(&mut self, _: &Args) {
        for (i, j) in self
            .best_sol
            .path
            .iter()
            .circular_tuple_windows::<(&usize, &usize)>()
        {
            self.best_sols.push(BestSolutionRecord {
                from: *i,
                to: *j,
                iter: self.iteration,
            })
        }
    }

    fn on_end(&mut self, _: &Args) {
        self.flush();
    }
}
