pub fn main() {
  let (cities, cost) = ecrs::aco::generate_tsp_cost(30);
  ecrs::aco::write_cities_csv(&cities, "cities.csv").expect("Error while writing city file");

  let heuristic = ecrs::aco::create_heuristic_from_weights(&cost);

  let ant_s = ecrs::aco::Builder::new()
    .set_weights(cost)
    .set_heuristic(heuristic)
    .build();

  ant_s.execute();
}
