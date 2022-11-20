use crate::aco::probe::{ConsoleProbe, Probe};
use crate::aco::FMatrix;

/// Struct wrapping all parameters needed for Ant System Algorithm.
///
/// # Parameters
/// * weights -  graph matrix representation.
/// * heuristic - data to help find optimal path. Pass matrix of 1 if no heuristic.
/// * alpha - importance of weights in edge choosing.
/// * beta - importance of heuristic in edge choosing, pass 0 to ignore.
/// * evaporation_rate - number in interval \[0, 1]. If 0 then new pheromone won't be influenced by old pheromone.
/// * ants_num - number of ants.
/// * iteration - number of iteration the algorithm should make.
/// * probe - logging probe.
///
/// For more details look [here](http://www.scholarpedia.org/article/Ant_colony_optimization) at Ant system section
pub struct AntSystemCfg {
  pub weights: FMatrix,
  pub heuristic: FMatrix,
  pub alpha: f64,
  pub beta: f64,
  pub evaporation_rate: f64,
  pub ants_num: usize,
  pub iteration: usize,
  pub probe: Box<dyn Probe>,
}

impl Default for AntSystemCfg {
  fn default() -> Self {
    AntSystemCfg {
      weights: FMatrix::zeros(0, 0),
      heuristic: FMatrix::zeros(0, 0),
      alpha: 1.0,
      beta: 1.0,
      evaporation_rate: 0.1,
      ants_num: 10,
      iteration: 300,
      probe: Box::new(ConsoleProbe::new()),
    }
  }
}
