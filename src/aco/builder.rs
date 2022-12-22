use crate::aco;
use crate::aco::aco_cfg::AntColonyOptimizationCfgOpt;
use crate::aco::ant::{Ant, StandardAnt};
use crate::aco::ants_behaviour::{AntSystemAB, AntsBehaviour};
use crate::aco::fitness::{CanonicalFitness, Fitness};
use crate::aco::goodness::{CanonicalGoodness, Goodness};
use crate::aco::pheromone::{AntSystemPU, PheromoneUpdate};
use crate::aco::probe::Probe;
use crate::aco::{AntColonyOptimization, AntColonyOptimizationCfg, CanonicalAnt, FMatrix};
use itertools::Itertools;
use rand::Rng;

/// Builder for [AntColonyOptimization]
///
pub struct Builder<P, A, G, AB, F>
where
  P: PheromoneUpdate,
  A: Ant,
  G: Goodness,
  AB: AntsBehaviour<A, G>,
  F: Fitness,
{
  conf: AntColonyOptimizationCfgOpt,
  evaporation_rate: f64,
  solution_size: usize,
  pheromone_update: Option<P>,
  ants_behaviour: Option<AB>,
  fitness: Option<F>,
  ants: Option<Vec<A>>,
  goodness: Option<G>,
  start_pheromone: FMatrix,
}

impl<P, A, G, AB, F> Builder<P, A, G, AB, F>
where
  P: PheromoneUpdate,
  A: Ant,
  G: Goodness,
  AB: AntsBehaviour<A, G>,
  F: Fitness,
{
  /// Creates a new instance of Builder.
  ///
  /// ### Defaults
  /// * `evaporation_rate` - 0.1
  /// * `start_pheromone` - matrix of 1.0
  /// * `iterations` - 300
  /// * `probe` - [aco::probe::StdoutProbe]
  pub fn new(solution_size: usize) -> Self {
    Builder {
      conf: AntColonyOptimizationCfgOpt {
        iteration: 300,
        probe: Box::new(aco::probe::StdoutProbe::new()),
      },
      evaporation_rate: 0.1,
      solution_size,
      pheromone_update: None,
      ants_behaviour: None,
      fitness: None,
      ants: None,
      goodness: None,
      start_pheromone: FMatrix::repeat(solution_size, solution_size, 1.0),
    }
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
    self.evaporation_rate = evaporation_rate;
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

  /// Sets the the way to calculate pheromone update.
  ///
  /// For more info see [aco::pheromone] module.
  ///
  /// ## Arguments
  /// * `pheromone_update` - Implementation of [PheromoneUpdate] trait.
  pub fn set_pheromone_update(mut self, pheromone_update: P) -> Self {
    self.pheromone_update = Some(pheromone_update);
    self
  }

  /// Sets the the way to calculate fitness.
  ///
  /// For more info see [aco::fitness] module.
  ///
  /// ## Arguments
  /// * `fitness` - Implementation of [Fitness] trait.
  pub fn set_fitness(mut self, fitness: F) -> Self {
    self.fitness = Some(fitness);
    self
  }

  /// Sets the the way to simulate ants behaviour.
  ///
  /// For more info see [aco::ants_behaviour] module.
  ///
  /// ## Arguments
  /// * `ants_behaviour` - Implementation of [AntsBehaviour] trait.
  pub fn set_ants_behaviour(mut self, ants_behaviour: AB) -> Self {
    self.ants_behaviour = Some(ants_behaviour);
    self
  }

  /// Sets the goodness operator.
  ///
  /// For more info see [aco::goodness] module.
  ///
  /// ## Arguments
  /// * `goodness` - Implementation of [Goodness] trait.
  pub fn set_goodness(mut self, goodness: G) -> Self {
    self.goodness = Some(goodness);
    self
  }

  /// Sets ants.
  ///
  /// ## Arguments
  /// * `ant` - vector of implementation of [Ant] trait.
  pub fn set_ants(mut self, ants: Vec<A>) -> Self {
    self.ants = Some(ants);
    self
  }

  /// Sets the initial pheromone.
  /// ## Arguments
  /// * `start_pheromone` - Initial pheromone.
  pub fn set_start_pheromone(mut self, start_pheromone: FMatrix) -> Self {
    self.start_pheromone = start_pheromone;
    self
  }

  /// Builds [AntColonyOptimization] with provided building blocks.
  ///
  /// * `pheromone_update` needs to be specified, if not program will panic
  /// * `start_pheromone` needs to be specified, if not program will panic
  /// * `ants_behaviour` needs to be specified, if not program will panic
  /// * `fitness` needs to be specified, if not program will panic
  /// * `goodness` needs to be specified, if not program will panic
  /// * `ants` need to be specified, if not program will panic
  pub fn build(self) -> AntColonyOptimization<P, A, G, AB, F> {
    let cfg_opt = AntColonyOptimizationCfg::try_from(self.conf);
    if let Err(err) = cfg_opt {
      panic!("{}", err);
    }

    let cfg = cfg_opt.unwrap();

    AntColonyOptimization {
      cfg,
      evaporation_rate: self.evaporation_rate,
      pheromone: self.start_pheromone,
      pheromone_update: self.pheromone_update.expect("Pheromone update rule wasn't set"),
      ants_behaviour: self.ants_behaviour.expect("Ants behaviour wasn't set"),
      fitness: self.fitness.expect("Fitness operator wasn't set"),
      goodness: self.goodness.expect("Goodness operator wasn't set"),
      ants: self.ants.expect("Ants weren't set"),
    }
  }
}

impl<P, F, A, AB> Builder<P, A, CanonicalGoodness, AB, F>
where
  P: PheromoneUpdate,
  A: Ant,
  AB: AntsBehaviour<A, CanonicalGoodness>,
  F: Fitness,
{
  /// Sets the importance of weights in edge choosing
  ///
  /// If goodness wasn't set then before setting the parameter, a new instance of CanonicalGoodness
  /// will be created with defaults:
  /// * `alpha` - 1.0
  /// * `beta` - 1.0
  /// * `heuristic` - matrix of 1.0
  ///
  /// ## Arguments
  /// * `alpha` - importance of weights in edge choosing.
  pub fn set_alpha(mut self, alpha: f64) -> Self {
    let mut g = self
      .goodness
      .unwrap_or_else(|| CanonicalGoodness::default(self.solution_size));
    g.alpha = alpha;
    self.goodness = Some(g);
    self
  }

  /// Sets the importance of heuristic in edge choosing, pass 0 to ignore.
  ///
  /// If goodness wasn't set then before setting the parameter, a new instance of CanonicalGoodness
  /// will be created with defaults:
  /// * `alpha` - 1.0
  /// * `beta` - 1.0
  /// * `heuristic` - matrix of 1.0
  ///
  /// ## Arguments
  /// * `beta` - importance of heuristic in edge choosing.
  pub fn set_beta(mut self, beta: f64) -> Self {
    let mut g = self
      .goodness
      .unwrap_or_else(|| CanonicalGoodness::default(self.solution_size));
    g.beta = beta;
    self.goodness = Some(g);
    self
  }

  /// Sets the heuristic to aide path finding.
  ///
  /// If goodness wasn't set then before setting the parameter, a new instance of CanonicalGoodness
  /// will be created with defaults:
  /// * `alpha` - 1.0
  /// * `beta` - 1.0
  /// * `heuristic` - matrix of 1.0
  ///
  /// Setting a heuristic is optional.
  ///
  /// ## Arguments
  /// * `heuristic` - Weighted graph in matrix representation.
  pub fn set_heuristic(mut self, heuristic: FMatrix) -> Self {
    assert_eq!(
      heuristic.nrows(),
      heuristic.ncols(),
      "Heuristic needs to be a square matrix"
    );
    assert_eq!(
      heuristic.nrows(),
      self.solution_size,
      "Heuristic needs to have length equal to solution_size"
    );
    let mut g = self
      .goodness
      .unwrap_or_else(|| CanonicalGoodness::default(self.solution_size));
    g.heuristic = heuristic;
    self.goodness = Some(g);
    self
  }
}

impl<P, A, G, AB> Builder<P, A, G, AB, CanonicalFitness>
where
  P: PheromoneUpdate,
  A: Ant,
  G: Goodness,
  AB: AntsBehaviour<A, G>,
{
  /// Sets the weighted graph to be searched.
  ///
  /// ## Arguments
  /// * `weights` - Weighted graph in matrix representation.
  pub fn set_weights(mut self, weights: FMatrix) -> Self {
    assert_eq!(
      weights.nrows(),
      weights.nrows(),
      "Weights should be a square matrix"
    );
    assert_eq!(
      weights.nrows(),
      self.solution_size,
      "Weights should be of length equal to solution size"
    );
    if let Some(mut f) = self.fitness {
      f.weights = weights;
      self.fitness = Some(f)
    } else {
      self.fitness = Some(CanonicalFitness::new(weights))
    }
    self
  }
}

impl<P, G, AB, F> Builder<P, StandardAnt, G, AB, F>
where
  P: PheromoneUpdate,
  G: Goodness,
  AB: AntsBehaviour<StandardAnt, G>,
  F: Fitness,
{
  pub fn with_standard_ants(mut self, ants_number: usize) -> Self {
    let ants = (0..ants_number)
      .map(|_| StandardAnt::new(self.solution_size))
      .collect_vec();
    self.ants = Some(ants);
    self
  }
}

type AntSystemBuilder<R> =
  Builder<AntSystemPU, CanonicalAnt<R>, CanonicalGoodness, AntSystemAB, CanonicalFitness>;

impl<R: Rng> AntSystemBuilder<R> {
  /// Creates a new instance of [Builder] with operators used for Ant System version of the algorithm.
  ///
  ///
  /// ### Defaults
  /// * `evaporation_rate` - 0.1
  /// * `start_pheromone` - matrix of 1.0
  /// * `iterations` - 300
  /// * `probe` - [aco::probe::StdoutProbe]
  /// * `alpha` - 1.0
  /// * `beta` - 1.0
  /// * `heuristic` - matrix of 1.0
  pub fn new_as(solution_size: usize) -> Self {
    let pheromone_update = AntSystemPU::new();
    let goodness = CanonicalGoodness::new(1.0, 1.0, FMatrix::repeat(solution_size, solution_size, 1.0));
    let ants_behaviour = AntSystemAB;
    let fitness = CanonicalFitness::new(FMatrix::repeat(solution_size, solution_size, 1.0));

    Self {
      conf: AntColonyOptimizationCfgOpt {
        iteration: 300,
        probe: Box::new(aco::probe::StdoutProbe::new()),
      },
      evaporation_rate: 0.1,
      solution_size,
      pheromone_update: Some(pheromone_update),
      ants_behaviour: Some(ants_behaviour),
      fitness: Some(fitness),
      ants: None,
      goodness: Some(goodness),
      start_pheromone: FMatrix::repeat(solution_size, solution_size, 1.0),
    }
  }
}
