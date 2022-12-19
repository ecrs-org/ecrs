use crate::aco::pheromone::PheromoneUpdate;
use crate::aco::probe::Probe;
use crate::aco::FMatrix;

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
pub(in crate::aco) struct AntColonyOptimizationCfg<P: PheromoneUpdate> {
  pub weights: FMatrix,
  pub heuristic: FMatrix,
  pub alpha: f64,
  pub beta: f64,
  pub evaporation_rate: f64,
  pub ants_num: usize,
  pub iteration: usize,
  pub probe: Box<dyn Probe>,
  pub pheromone_update: P,
}

pub(in crate::aco) struct AntColonyOptimizationCfgOpt<P: PheromoneUpdate> {
  pub weights: FMatrix,
  pub heuristic: FMatrix,
  pub alpha: f64,
  pub beta: f64,
  pub evaporation_rate: f64,
  pub ants_num: usize,
  pub iteration: usize,
  pub probe: Box<dyn Probe>,
  pub pheromone_update: Option<P>,
}

impl<P: PheromoneUpdate> TryFrom<AntColonyOptimizationCfgOpt<P>> for AntColonyOptimizationCfg<P> {
  type Error = &'static str;

  fn try_from(value: AntColonyOptimizationCfgOpt<P>) -> Result<Self, Self::Error> {
    if let Some(pheromone_update) = value.pheromone_update {
      Ok(AntColonyOptimizationCfg {
        weights: value.weights,
        heuristic: value.heuristic,
        alpha: value.alpha,
        beta: value.beta,
        evaporation_rate: value.evaporation_rate,
        ants_num: value.ants_num,
        iteration: value.iteration,
        probe: value.probe,
        pheromone_update,
      })
    } else {
      Err("Pheromone update is not specified")
    }
  }
}
