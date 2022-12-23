fn main() {
  let (cities, cost) = ecrs::aco::util::generate_tsp_cost(30);
  ecrs::aco::util::write_cities_csv(&cities, "cities.csv").expect("Error while writing city file");

  let heuristic = ecrs::aco::util::create_heuristic_from_weights(&cost);

  let ant_s = ecrs::aco::Builder::new_as(30)
    .set_weights(cost)
    .with_iteration_termination(300)
    .with_standard_ants(10)
    .set_heuristic(heuristic)
    .with_stdout_probe()
    .build();

  ant_s.run();
}
