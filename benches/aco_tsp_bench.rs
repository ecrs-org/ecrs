use itertools::Itertools;
use ecrs::aco::pheromone::{PartFromEvalPU, Pheromone};
use ecrs::aco::probe::Probe;
use ecrs::aco::{FMatrix, Solution};
use ecrs::aco::ants_behaviour::AntSystemAB;
use ecrs::aco::goodness::ExpRandGoodness;

const ITERS: usize = 500;
const SOL_SIZE: usize = 400;
struct MyCsvProbe<'a, const C: usize> {
  fitness: [f64; C],
  iter: usize,
  save_path: &'a str
}

impl <'a,const C: usize> MyCsvProbe<'a, C> {
  fn flush(&mut self) {
    let mut wrt = csv::WriterBuilder::new().from_path(self.save_path,).unwrap();

    for rec in self.fitness.iter().enumerate() {
      wrt.serialize(rec).expect("Could not serialize record");
    }
    wrt.flush().expect("Could not flush data");
  }
}

impl <'a,P: Pheromone,const C: usize> Probe<P> for MyCsvProbe<'a,C> {
  fn on_current_best(&mut self, best: &Solution) {
    self.fitness[self.iter] = best.fitness;
    self.iter+=1;
  }

  fn on_end(&mut self) {
    self.flush();
  }
}

fn ant_system(cost: FMatrix, heuristic:FMatrix) {
  let probe = MyCsvProbe{
    fitness: [0.0; ITERS],
    iter:0,
    save_path: "as_a10"
  };

  let start_pheromone =
    ecrs::aco::FMatrix::repeat(heuristic.nrows(), heuristic.ncols(), 1.0);

  let ant_s = ecrs::aco::Builder::new_as(heuristic.nrows())
    .set_weights(cost)
    .with_iteration_termination(ITERS)
    .with_standard_ants(SOL_SIZE)
    .set_heuristic(heuristic)
    .set_probe(probe)
    .set_start_pheromone(start_pheromone)
    .build();

  ant_s.run();

}

fn mmant_system(cost: FMatrix, heuristic:FMatrix) {
  let probe = MyCsvProbe{
    fitness: [0.0; ITERS],
    iter:0,
    save_path: "mmas_a10"
  };
  let start_pheromone =
    ecrs::aco::FMatrix::repeat(heuristic.nrows(), heuristic.ncols(), 1.0);

  let ant_s = ecrs::aco::Builder::new_mmas(heuristic.nrows())
    .set_weights(cost)
    .with_iteration_termination(ITERS)
    .with_standard_ants(SOL_SIZE)
    .set_probe(probe)
    .set_heuristic(heuristic)
    .set_pheromone_bound(0.0, 10.0)
    .set_start_pheromone(start_pheromone)
    .build();

  ant_s.run();
}

fn as2d(cost: FMatrix, heuristic:FMatrix) {
  let probe = MyCsvProbe{
    fitness: [0.0; ITERS],
    iter:0,
    save_path: "as2d_a10"
  };

  let start_pheromone = (0..8)
    .map(|_| FMatrix::repeat(heuristic.nrows(), heuristic.ncols(), 1.0))
    .collect_vec();
  let ant_s = ecrs::aco::Builder::new(heuristic.nrows())
    .set_weights(cost)
    .with_standard_ants(SOL_SIZE)
    .set_ants_behaviour(AntSystemAB)
    .set_goodness(ExpRandGoodness::new(1.0,1.0, heuristic))
    .set_pheromone_update(PartFromEvalPU::new())
    .with_iteration_termination(ITERS)
    .set_probe(probe)
    .set_start_pheromone(start_pheromone)
    .build();

  ant_s.run();
}

fn aco(cost: FMatrix, heuristic:FMatrix) {
  let probe = MyCsvProbe{
    fitness: [0.0; ITERS],
    iter:0,
    save_path: "acs_a10"
  };


  let rule = ecrs::aco::local_update::Decay::new(0.95);
  let start_pheromone =
    ecrs::aco::FMatrix::repeat(heuristic.nrows(), heuristic.ncols(), 1.0);

  let ant_s = ecrs::aco::Builder::new_acs(heuristic.nrows(), rule)
    .set_weights(cost)
    .with_iteration_termination(ITERS)
    .with_standard_exploiting_ants(SOL_SIZE, 0.05)
    .set_probe(probe)
    .set_heuristic(heuristic)
    .set_start_pheromone(start_pheromone)
    .build();

  ant_s.run();
}

fn main() {
  let (cities, cost) = ecrs::aco::util::generate_tsp_cost(SOL_SIZE);
  ecrs::aco::util::write_cities_csv(&cities, "cities.csv").expect("Error while writing city file");

  let heuristic = ecrs::aco::util::create_heuristic_from_weights(&cost);

  for _ in 0..10 {
    ant_system(cost.clone(), heuristic.clone());
    mmant_system(cost.clone(), heuristic.clone());
    aco(cost.clone(), heuristic.clone());
    as2d(cost.clone(), heuristic.clone());
  }



}