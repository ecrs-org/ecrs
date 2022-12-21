use ecrs::ff::auxiliary::*;
use ecrs::ff::probe::console_probe::ConsoleProbe;
use ecrs::ff::{FireflyAlgorithm, FireflyAlgorithmCfg};

fn main() {
  let mut alg = FireflyAlgorithm {
    config: FireflyAlgorithmCfg::default(),
    brightness_function: rastrigin,
    probe: Box::new(ConsoleProbe {}),
  };

  alg.execute();
}
