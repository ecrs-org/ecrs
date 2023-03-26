use ecrs::aco::pheromone::AntSystemPU;

fn main() {
  let (cities, cost) = ecrs::aco::util::generate_tsp_cost(30);
  ecrs::aco::util::write_cities_csv(&cities, "cities.csv").expect("Error while writing city file");

  let heuristic = ecrs::aco::util::create_heuristic_from_weights(&cost);
  let start_pheromone =
    ecrs::aco::FMatrix::repeat(heuristic.nrows(), heuristic.ncols(), 1.0);

  let ab = ecrs::aco::ants_behaviour::AntColonySystemAB::with_rule(ecrs::aco::local_update::Decay::new(0.95));

  let ant_s = ecrs::aco::Builder::new(30)
    .set_ants_behaviour(ab)
    .set_pheromone_update(AntSystemPU)
    .with_iteration_termination(100)
    .with_stdout_probe()
    .set_weights(cost)
    .set_start_pheromone(start_pheromone)
    .with_standard_ants(10)
    .set_heuristic(heuristic)
    .build();

  ant_s.run();
}
