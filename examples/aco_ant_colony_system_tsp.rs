fn main() {
  let (cities, cost) = ecrs::aco::util::generate_tsp_cost(30);
  ecrs::aco::util::write_cities_csv(&cities, "cities.csv").expect("Error while writing city file");

  let heuristic = ecrs::aco::util::create_heuristic_from_weights(&cost);
  let rule = ecrs::aco::local_update::Decay::new(0.99);

  let ant_s = ecrs::aco::Builder::new_acs(30, rule)
    .set_weights(cost)
    .with_iteration_termination(300)
    .with_standard_exploiting_ants(10, 0.2)
    .set_heuristic(heuristic)
    .build();

  ant_s.run();
}
