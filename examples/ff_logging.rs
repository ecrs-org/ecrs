use ecrs::ff::auxiliary::*;
use ecrs::ff::probe::aggregated_probe::AggregatedProbe;
use ecrs::ff::probe::csv_probe::CsvProbe;
use ecrs::ff::probe::policy_driven_probe::PolicyDrivenProbe;
use ecrs::ff::probe::probing_policy::GenerationInterval;
use ecrs::ff::probe::stdout_probe::StdoutProbe;
use ecrs::ff::probe::Probe;
use ecrs::ff::{FireflyAlgorithm, FireflyAlgorithmCfg};

fn main() {
  let stdout_probe = Box::new(StdoutProbe::new());
  let csv_probe = Box::new(CsvProbe::new("firefly_example.csv"));
  let probes: Vec<Box<dyn Probe>> = vec![stdout_probe, csv_probe];
  let aggregated_probe = Box::new(AggregatedProbe::new(probes));
  let probing_policy = Box::new(GenerationInterval::new(15, 25));
  let policy_driven_probe = Box::new(PolicyDrivenProbe::new(aggregated_probe, probing_policy));

  let mut alg = FireflyAlgorithm {
    config: FireflyAlgorithmCfg::default(),
    brightness_function: rastrigin,
    probe: policy_driven_probe,
  };

  alg.execute();
}
