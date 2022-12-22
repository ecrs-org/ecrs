use crate::aco::probe::Probe;

/// Struct wrapping all common parameters needed for Ant Colony Optimization algorithms.
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
/// For more details look [here](http://www.scholarpedia.org/article/Ant_colony_optimization)
pub(in crate::aco) struct AntColonyOptimizationCfg {
  pub evaporation_rate: f64,
  pub iteration: usize,
  pub probe: Box<dyn Probe>,
}

pub(in crate::aco) struct AntColonyOptimizationCfgOpt {
  pub evaporation_rate: f64,
  pub iteration: usize,
  pub probe: Box<dyn Probe>,
}

impl TryFrom<AntColonyOptimizationCfgOpt> for AntColonyOptimizationCfg {
  type Error = &'static str;

  fn try_from(value: AntColonyOptimizationCfgOpt) -> Result<Self, Self::Error> {
    Ok(AntColonyOptimizationCfg {
      evaporation_rate: value.evaporation_rate,
      iteration: value.iteration,
      probe: value.probe,
    })
  }
}
