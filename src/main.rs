mod firefly_algorithm;
mod auxiliary;

use firefly_algorithm::*;
use auxiliary::*;

fn main(){
  let alg = FireflyAlgorithm{
    config: FireflyAlgorithmCfg::default(),
    brightness_function: rastrigin,
  };

  alg.execute();
}