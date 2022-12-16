use crate::aco::ants_system::Solution;
use crate::aco::probe::Probe;
use crate::aco::{AntSystem, AntSystemCfg, FMatrix};
/// Builder for [AntSystem]
///
/// For more details about parameters look [here](http://www.scholarpedia.org/article/Ant_colony_optimization) at Ant system section
pub struct Builder {
  conf: AntSystemCfg,
}

impl Builder {
  /// Creates a new instance of Builder.
  pub fn new() -> Self {
    Builder {
      conf: AntSystemCfg::default(),
    }
  }

  /// Sets the weighted graph to be searched.
  ///
  /// ## Arguments
  /// * `weights` - Weighted graph in matrix representation.
  pub fn set_weights(mut self, weights: FMatrix) -> Self {
    self.conf.weights = weights;
    self
  }

  /// Sets the heuristic to aide path finding.
  ///
  /// Setting a heuristic is optional.
  ///
  /// ## Arguments
  /// * `heuristic` - Weighted graph in matrix representation.
  pub fn set_heuristic(mut self, heuristic: FMatrix) -> Self {
    self.conf.heuristic = heuristic;
    self
  }

  /// Sets the importance of weights in edge choosing
  ///
  /// ## Arguments
  /// * `alpha` - importance of weights in edge choosing.
  pub fn set_alpha(mut self, alpha: f64) -> Self {
    self.conf.alpha = alpha;
    self
  }

  /// Sets the importance of heuristic in edge choosing, pass 0 to ignore.
  ///
  /// ## Arguments
  /// * `beta` - importance of heuristic in edge choosing.
  pub fn set_beta(mut self, beta: f64) -> Self {
    self.conf.beta = beta;
    self
  }

  /// Sets the evaporation rate.
  ///
  /// If 1 then old pheromone will fully evaporate every iteration. In other words old pheromone
  /// will be fully replaced with new pheromone.
  ///
  /// ## Arguments
  /// * `evaporation rate` - number in interval \[0, 1].
  pub fn set_evaporation_rate(mut self, evaporation_rate: f64) -> Self {
    assert!(
      (0.0..=1.0).contains(&evaporation_rate),
      "Evaporation rate must be between 0 and 1"
    );
    self.conf.evaporation_rate = evaporation_rate;
    self
  }

  /// Sets the number of ants (number of solution per iteration).
  ///
  /// ## Arguments
  /// * `ants_num` - number of ants.
  pub fn set_ants_num(mut self, ants_num: usize) -> Self {
    assert!(ants_num > 0, "Number of ants must be greater than 0");
    self.conf.ants_num = ants_num;
    self
  }

  /// Sets the number of algorithm iterations.
  ///
  /// ## Arguments
  /// * `iterations` - number of iteration the algorithm should make.
  pub fn set_iterations(mut self, iterations: usize) -> Self {
    self.conf.iteration = iterations;
    self
  }

  /// Sets the used [Probe].
  ///
  ///
  /// ## Arguments
  /// * `probe` - Box of [Probe] trait implementation.
  pub fn set_probe(mut self, probe: Box<dyn Probe>) -> Self {
    self.conf.probe = probe;
    self
  }

  /// Builds [AntSystem] with provided building blocks.
  ///
  /// If specific building block is not provided a default value will be used.
  /// ### Defaults
  /// * `weights` - 0 x 0 matrix
  /// * `heuristic` - matrix of ones with `weights` shape
  /// * `alpha` - 1.0
  /// * `beta` - 1.0
  /// * `evaporation_rate` - 0.1
  /// * `ants_num` - 10
  /// * `iterations` - 300
  /// * `probe` - [crate::aco::probe::StdoutProbe]
  pub fn build(mut self) -> AntSystem {
    let (nrow, ncol) = self.conf.weights.shape();

    if self.conf.heuristic.shape() != (nrow, ncol) {
      self.conf.heuristic = FMatrix::repeat(nrow, ncol, 1.0);
    }

    let pheromone = FMatrix::repeat(nrow, ncol, 0.5f64);

    AntSystem {
      cfg: self.conf,
      pheromone,
      best_sol: Solution::default(),
    }
  }
}
