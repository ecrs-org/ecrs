use itertools::Itertools;
use ecrs::aco::{FMatrix};
use ecrs::aco::ants_behaviour::AntSystemAB;
use ecrs::aco::goodness::ExpRandGoodness;
use ecrs::aco::pheromone::PartFromEvalPU;
use ecrs::aco::probe::{StdoutProbe};


fn main() {
  let (cities, cost) = ecrs::aco::util::generate_tsp_cost(100);
  ecrs::aco::util::write_cities_csv(&cities, "cities.csv").expect("Error while writing city file");

  let heuristic = ecrs::aco::util::create_heuristic_from_weights(&cost);
  let start_pheromone = (0..5)
    .map(|_| FMatrix::repeat(heuristic.nrows(), heuristic.ncols(), 1.0))
    .collect_vec();
  let ant_s = ecrs::aco::Builder::new(100)
    .set_weights(cost)
    .with_standard_ants(10)
    .set_ants_behaviour(AntSystemAB)
    .set_goodness(ExpRandGoodness::new(3.0,3.0, heuristic))
    .set_pheromone_update(PartFromEvalPU::new())
    .with_iteration_termination(300)
    .set_probe(StdoutProbe::new())
    .set_start_pheromone(start_pheromone)
    .build();

  ant_s.run();
}