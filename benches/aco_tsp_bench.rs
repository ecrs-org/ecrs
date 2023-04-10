use std::fs::OpenOptions;
use std::io::Write;
use itertools::Itertools;
use ecrs::aco::pheromone::{PartFromEvalPU, Pheromone};
use ecrs::aco::probe::Probe;
use ecrs::aco::{FMatrix, Solution};
use ecrs::aco::ants_behaviour::AntSystemAB;
use ecrs::aco::goodness::ExpRandGoodness;

const BERLIN_52: [(f64, f64); 52] = [(565.0, 575.0),
  (25.0, 185.0),
  (345.0, 750.0),
  (945.0, 685.0),
  (845.0, 655.0),
  (880.0, 660.0),
  (25.0, 230.0),
  (525.0, 1000.0),
  (580.0, 1175.0),
  (650.0, 1130.0),
  (1605.0, 620.0),
  (1220.0, 580.0),
  (1465.0, 200.0),
  (1530.0, 5.0),
  (845.0, 680.0),
  (725.0, 370.0),
  (145.0, 665.0),
  (415.0, 635.0),
  (510.0, 875.0),
  (560.0, 365.0),
  (300.0, 465.0),
  (520.0, 585.0),
  (480.0, 415.0),
  (835.0, 625.0),
  (975.0, 580.0),
  (1215.0, 245.0),
  (1320.0, 315.0),
  (1250.0, 400.0),
  (660.0, 180.0),
  (410.0, 250.0),
  (420.0, 555.0),
  (575.0, 665.0),
  (1150.0, 1160.0),
  (700.0, 580.0),
  (685.0, 595.0),
  (685.0, 610.0),
  (770.0, 610.0),
  (795.0, 645.0),
  (720.0, 635.0),
  (760.0, 650.0),
  (475.0, 960.0),
  (95.0, 260.0),
  (875.0, 920.0),
  (700.0, 500.0),
  (555.0, 815.0),
  (830.0, 485.0),
  (1170.0, 65.0),
  (830.0, 610.0),
  (605.0, 625.0),
  (595.0, 360.0),
  (1340.0, 725.0),
  (1740.0, 245.0)
];

const ITERS: usize = 200;
const ANTS_NUMBER: usize = 100;
const EVAPORATION: f64 = 0.05;

struct MyCsvProbe<'a, const C: usize> {
  fitness: [f64; C],
  iter: usize,
  label: &'a str,
}

impl<'a, const C: usize> MyCsvProbe<'a, C> {
  fn flush(&mut self) {
    let mut file = OpenOptions::new()
      .append(true)
      .create(true)
      .open("berlin52_results.csv")
      .expect("Could not open file");

    for i in 0..self.iter {
      writeln!(file, "{},{},{}", i, self.fitness[i], self.label).expect("Error while writing to file");
    }
    file.flush().expect("Could not flush")
  }
}

impl<'a, P: Pheromone, const C: usize> Probe<P> for MyCsvProbe<'a, C> {
  fn on_current_best(&mut self, best: &Solution) {
    self.fitness[self.iter] = best.fitness;
    self.iter += 1;
  }

  fn on_end(&mut self) {
    self.flush();
  }
}

fn ant_system(cost: FMatrix, heuristic: FMatrix) {
  let probe = MyCsvProbe {
    fitness: [0.0; ITERS],
    iter: 0,
    label: "as",
  };

  let start_pheromone =
    ecrs::aco::FMatrix::repeat(heuristic.nrows(), heuristic.ncols(), 1.0);

  let ant_s = ecrs::aco::Builder::new_as(heuristic.nrows())
    .set_weights(cost)
    .with_iteration_termination(ITERS)
    .with_standard_ants(ANTS_NUMBER)
    .set_alpha(3.0)
    .set_beta(3.0)
    .set_heuristic(heuristic)
    .set_probe(probe)
    .set_evaporation_rate(EVAPORATION)
    .set_start_pheromone(start_pheromone)
    .build();

  ant_s.run();
}

fn mmant_system(cost: FMatrix, heuristic: FMatrix) {
  let probe = MyCsvProbe {
    fitness: [0.0; ITERS],
    iter: 0,
    label: "mmas",
  };
  let start_pheromone =
    ecrs::aco::FMatrix::repeat(heuristic.nrows(), heuristic.ncols(), 1.0);

  let ant_s = ecrs::aco::Builder::new_mmas(heuristic.nrows())
    .set_weights(cost)
    .with_iteration_termination(ITERS)
    .with_standard_ants(ANTS_NUMBER)
    .set_probe(probe)
    .set_evaporation_rate(EVAPORATION)
    .set_heuristic(heuristic)
    .set_pheromone_bound(0.0, 10.0)
    .set_alpha(3.0)
    .set_beta(3.0)
    .set_start_pheromone(start_pheromone)
    .build();

  ant_s.run();
}

fn as2d(cost: FMatrix, heuristic: FMatrix) {
  let probe = MyCsvProbe {
    fitness: [0.0; ITERS],
    iter: 0,
    label: "as2d",
  };

  let start_pheromone = (0..8)
    .map(|_| FMatrix::repeat(heuristic.nrows(), heuristic.ncols(), 1.0))
    .collect_vec();
  let ant_s = ecrs::aco::Builder::new(heuristic.nrows())
    .set_weights(cost)
    .with_standard_ants(ANTS_NUMBER)
    .set_ants_behaviour(AntSystemAB)
    .set_goodness(ExpRandGoodness::new(3.0, 3.0, heuristic))
    .set_pheromone_update(PartFromEvalPU::new())
    .with_iteration_termination(ITERS)
    .set_probe(probe)
    .set_start_pheromone(start_pheromone)
    .set_evaporation_rate(EVAPORATION)
    .build();

  ant_s.run();
}

fn aco(cost: FMatrix, heuristic: FMatrix) {
  let probe = MyCsvProbe {
    fitness: [0.0; ITERS],
    iter: 0,
    label: "acs",
  };


  let rule = ecrs::aco::local_update::Decay::new(0.99);
  let start_pheromone =
    ecrs::aco::FMatrix::repeat(heuristic.nrows(), heuristic.ncols(), 1.0);

  let ant_s = ecrs::aco::Builder::new_acs(heuristic.nrows(), rule)
    .set_weights(cost)
    .with_iteration_termination(ITERS)
    .with_standard_exploiting_ants(ANTS_NUMBER, 0.05)
    .set_probe(probe)
    .set_alpha(3.0)
    .set_beta(3.0)
    .set_heuristic(heuristic)
    .set_start_pheromone(start_pheromone)
    .set_evaporation_rate(EVAPORATION)
    .build();

  ant_s.run();
}

fn distance(p1: &(f64,f64), p2: &(f64,f64)) -> f64 {
  let dx = p1.0 - p2.0;
  let dy = p1.1 - p2.1;
  (dx*dx + dy*dy).sqrt()
}

fn main() {
  let mut cost = FMatrix::zeros(BERLIN_52.len(), BERLIN_52.len());
  for i in 0..BERLIN_52.len() {
    for j in (i+1)..BERLIN_52.len() {
      let dist = distance(&BERLIN_52[i], &BERLIN_52[j]);
      cost[(i,j)] = dist;
      cost[(j,i)] = dist;
    }
  }


  let heuristic = ecrs::aco::util::create_heuristic_from_weights(&cost);

  for _ in 0..10 {
    ant_system(cost.clone(), heuristic.clone());
    mmant_system(cost.clone(), heuristic.clone());
    aco(cost.clone(), heuristic.clone());
    as2d(cost.clone(), heuristic.clone());
  }
}