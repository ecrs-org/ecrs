fn main() {
  let (cities, cost) = ecrs::aco::util::generate_tsp_cost(30);
  ecrs::aco::util::write_cities_csv(&cities, "cities.csv").expect("Error while writing city file");

  let heuristic = ecrs::aco::util::create_heuristic_from_weights(&cost);

  let csv_probe = ecrs::aco::probe::CsvProbe::new();
  let stdout_probe = ecrs::aco::probe::StdoutProbe::new();
  let aggregated_probe = ecrs::aco::probe::AggregatedProbe::new()
    .add_probe(csv_probe)
    .add_probe(stdout_probe);
  let probing_policy = ecrs::aco::probe::GenerationInterval::new(15, 25);
  let policy_driven_probe =
    ecrs::aco::probe::PolicyDrivenProbe::new(Box::new(aggregated_probe), Box::new(probing_policy));

  let ant_s = ecrs::aco::Builder::new_as(30)
    .set_weights(cost)
    .set_heuristic(heuristic)
    .with_standard_ants(10)
    .with_iteration_termination(300)
    .set_probe(Box::new(policy_driven_probe))
    .build();

  ant_s.run();
}
