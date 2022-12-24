use ecrs::ff::auxiliary::*;
use ecrs::ff::probe::stdout_probe::StdoutProbe;
use ecrs::ff::{FireflyAlgorithm, FireflyAlgorithmCfg};

fn main() {
  let mut alg = FireflyAlgorithm::default();

  alg.execute();
}
