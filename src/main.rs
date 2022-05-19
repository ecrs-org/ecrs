mod ff;

use ff::*;
use ff::auxiliary::*;

use ff::probe::console_probe::ConsoleProbe;

fn main(){
  let mut alg = FireflyAlgorithm{
    config: FireflyAlgorithmCfg::default(),
    brightness_function: rastrigin,
    probe: Box::new((ConsoleProbe{}))
  };

  alg.execute();
}