fn main() {
  let (cities, cost) = ecrs::aco::util::generate_tsp_cost(30);
  ecrs::aco::util::write_cities_csv(&cities, "cities.csv").expect("Error while writing city file");

  let heuristic = ecrs::aco::util::create_heuristic_from_weights(&cost);

  let ant_s = ecrs::aco::Builder::new()
    .set_weights(cost)
    .set_heuristic(heuristic)
    .set_pheromone_update(ecrs::aco::pheromone::AntSystemPU)
    .build();

  ant_s.run();
}
