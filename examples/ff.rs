use ecrs::ff::auxiliary::*;
use ecrs::ff::probe::console_probe::ConsoleProbe;
use ecrs::ff::*;

mod aco_tsp;

fn main() {
  let mut alg = FireflyAlgorithm {
    config: FireflyAlgorithmCfg::default(),
    brightness_function: rastrigin,
    probe: Box::new(ConsoleProbe {}),
  };

  alg.execute();
}
