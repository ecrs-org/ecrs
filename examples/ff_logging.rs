use ecrs::ff::probe::aggregated_probe::AggregatedProbe;
use ecrs::ff::probe::csv_probe::CsvProbe;
use ecrs::ff::probe::policy_driven_probe::PolicyDrivenProbe;
use ecrs::ff::probe::probing_policy::GenerationInterval;
use ecrs::ff::probe::stdout_probe::StdoutProbe;
use ecrs::ff::FireflyAlgorithm;

fn main() {
  let stdout_probe = StdoutProbe::new();
  let csv_probe = CsvProbe::new("firefly_example.csv");
  let aggregated_probe = AggregatedProbe::new()
    .add_probe(stdout_probe)
    .add_probe(csv_probe);
  let probing_policy = GenerationInterval::new(15, 25);
  let policy_driven_probe = PolicyDrivenProbe::new(Box::new(aggregated_probe), Box::new(probing_policy));

  let mut alg = FireflyAlgorithm {
    probe: Box::new(policy_driven_probe),
    ..Default::default()
  };

  alg.run();
}
