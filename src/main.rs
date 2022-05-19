mod firefly_algorithm;
mod auxiliary;
mod probe;

use firefly_algorithm::*;
use auxiliary::*;

use probe::console_probe::ConsoleProbe;

fn main(){
  let mut alg = FireflyAlgorithm{
    config: FireflyAlgorithmCfg::default(),
    brightness_function: rastrigin,
    probe: Box::new((ConsoleProbe{}))
  };

  alg.execute();
}