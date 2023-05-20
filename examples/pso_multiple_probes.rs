#[cfg(feature = "pso")]
use ecrs::pso::{self, builder::PSOAlgorithmBuilder};

#[cfg(feature = "pso")]
fn main() {
    let stdout_probe = Box::new(pso::probe::stdout_probe::StdoutProbe::new());
    let csv_probe = Box::new(pso::probe::csv_probe::CsvProbe::new("pso_example.csv"));
    let json_probe = Box::new(pso::probe::json_probe::JsonProbe::new("pso_example.json"));
    let probes: Vec<Box<dyn pso::probe::Probe>> = vec![stdout_probe, csv_probe, json_probe];
    let aggregated_probe = Box::new(pso::probe::aggregated_probe::AggregatedProbe::from_probes(probes));
    let probing_policy = Box::new(pso::probe::probing_policy::GenerationInterval::new(50));
    let policy_driven_probe = Box::new(pso::probe::policy_driven_probe::PolicyDrivenProbe::new(
        aggregated_probe,
        probing_policy,
    ));

    let mut algorithm = PSOAlgorithmBuilder::new()
        .set_dimensions(3)
        .set_generation_limit(2000)
        .set_probe(policy_driven_probe)
        .build();

    algorithm.run();
}

#[cfg(not(feature = "pso"))]
fn main() {
    panic!("Required feature \"pso\" is not enabled");
}
