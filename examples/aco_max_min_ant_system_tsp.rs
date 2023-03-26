fn main() {
  let (cities, cost) = ecrs::aco::util::generate_tsp_cost(30);
  ecrs::aco::util::write_cities_csv(&cities, "cities.csv").expect("Error while writing city file");

  let heuristic = ecrs::aco::util::create_heuristic_from_weights(&cost);
  let start_pheromone =
    ecrs::aco::FMatrix::repeat(heuristic.nrows(), heuristic.ncols(), 1.0);

  let ant_s = ecrs::aco::Builder::new_mmas(30)
    .set_weights(cost)
    .with_iteration_termination(300)
    .with_standard_ants(10)
    .with_stdout_probe()
    .set_heuristic(heuristic)
    .set_pheromone_bound(0.0, 10.0)
    .set_start_pheromone(start_pheromone)
    .build();

  ant_s.run();
}
