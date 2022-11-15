use ecrs::pso::{builder::PSOAlgorithmBuilder, self};

pub fn pso_example() {
    let iterations = 2000;

    let console_probe = Box::new(pso::probe::console_probe::ConsoleProbe::new());
    let csv_probe = Box::new(pso::probe::csv_probe::CsvProbe::new("pso_example.csv"));
    let json_probe = Box::new(pso::probe::json_probe::JsonProbe::new("pso_example.json"));
    let probes : Vec<Box<dyn pso::probe::Probe>> = vec![console_probe, csv_probe, json_probe];
    let multi_probe = Box::new(pso::probe::multi_probe::MultiProbe::new(probes));
    let iteration_count_probe = Box::new(pso::probe::iteration_count_probe::IterationCountProbe::new(multi_probe, 50, iterations));
    // let timed_probe = Box::new(pso::probe::timed_probe::TimedProbe::new(multi_probe, 3, iterations));


    let mut algorithm = PSOAlgorithmBuilder::new()
        .set_dimensions(3)
        .set_iterations(iterations)
        .set_probe(iteration_count_probe)
        .build();

    algorithm.run();
}
