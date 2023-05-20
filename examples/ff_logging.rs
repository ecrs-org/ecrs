#[cfg(feature = "ff")]
use ecrs::ff::probe::{
    aggregated_probe::AggregatedProbe, csv_probe::CsvProbe, policy_driven_probe::PolicyDrivenProbe,
    probing_policy::GenerationInterval, stdout_probe::StdoutProbe,
};

#[cfg(feature = "ff")]
use ecrs::ff::FireflyAlgorithm;

#[cfg(feature = "ff")]
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

#[cfg(not(feature = "ff"))]
fn main() {
    panic!("Required feature \"ff\" is not enabled");
}
