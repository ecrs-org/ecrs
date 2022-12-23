use crate::aco;
use crate::aco::aco_cfg::AntColonyOptimizationCfgOpt;
use crate::aco::ant::{Ant, CanonicalAnt, ExploitingAnt};
use crate::aco::ants_behaviour::{AntColonySystemAB, AntSystemAB, AntsBehaviour};
use crate::aco::fitness::{CanonicalFitness, Fitness};
use crate::aco::goodness::{CanonicalGoodness, Goodness};
use crate::aco::local_update::LocalUpdate;
use crate::aco::pheromone::best_policy::{BestPolicy, OverallBest};
use crate::aco::pheromone::{AntColonySystemPU, AntSystemPU, MMAntSystemPU, PheromoneUpdate};
use crate::aco::probe::Probe;
use crate::aco::termination_condition::{IterationCond, TerminationCondition};
use crate::aco::{AntColonyOptimization, AntColonyOptimizationCfg, FMatrix};
use itertools::Itertools;
use rand::prelude::ThreadRng;
use rand::Rng;

/// Builder for [AntColonyOptimization]
///
pub struct Builder<P, A, G, AB, F, T>
where
  P: PheromoneUpdate,
  A: Ant,
  G: Goodness,
  AB: AntsBehaviour<A, G>,
  F: Fitness,
  T: TerminationCondition<A>,
{
  conf: AntColonyOptimizationCfgOpt,
  evaporation_rate: f64,
  solution_size: usize,
  pheromone_update: Option<P>,
  ants_behaviour: Option<AB>,
  fitness: Option<F>,
  ants: Option<Vec<A>>,
  goodness: Option<G>,
  termination_cond: Option<T>,
  start_pheromone: FMatrix,
}

impl<P, A, G, AB, F, T> Builder<P, A, G, AB, F, T>
where
  P: PheromoneUpdate,
  A: Ant,
  G: Goodness,
  AB: AntsBehaviour<A, G>,
  F: Fitness,
  T: TerminationCondition<A>,
{
  /// Creates a new instance of Builder.
  ///
  /// ### Defaults
  /// * `evaporation_rate` - 0.1
  /// * `start_pheromone` - matrix of 1.0
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
      termination_cond: None,
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

  /// Sets the termination condition.
  ///
  /// For more info see [aco::termination_condition] module.
  ///
  /// ## Arguments
  /// * `goodness` - Implementation of [TerminationCondition] trait.
  pub fn set_termination_condition(mut self, termination_condition: T) -> Self {
    self.termination_cond = Some(termination_condition);
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
  pub fn build(self) -> AntColonyOptimization<P, A, G, AB, F, T> {
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
      termination_cond: self.termination_cond.expect("Termination condition wasn't set"),
      ants: self.ants.expect("Ants weren't set"),
    }
  }
}

impl<P, F, A, AB, T> Builder<P, A, CanonicalGoodness, AB, F, T>
where
  P: PheromoneUpdate,
  A: Ant,
  AB: AntsBehaviour<A, CanonicalGoodness>,
  F: Fitness,
  T: TerminationCondition<A>,
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

impl<P, A, G, AB, T> Builder<P, A, G, AB, CanonicalFitness, T>
where
  P: PheromoneUpdate,
  A: Ant,
  G: Goodness,
  AB: AntsBehaviour<A, G>,
  T: TerminationCondition<A>,
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

impl<P, G, AB, F, T> Builder<P, CanonicalAnt<ThreadRng>, G, AB, F, T>
where
  P: PheromoneUpdate,
  G: Goodness,
  AB: AntsBehaviour<CanonicalAnt<ThreadRng>, G>,
  F: Fitness,
  T: TerminationCondition<CanonicalAnt<ThreadRng>>,
{
  /// Creates the given number of [CanonicalAnt] with thread RNG
  pub fn with_standard_ants(mut self, ants_number: usize) -> Self {
    let ants = (0..ants_number)
      .map(|_| CanonicalAnt::<ThreadRng>::new(self.solution_size))
      .collect_vec();
    self.ants = Some(ants);
    self
  }
}

impl<P, G, AB, F, T> Builder<P, ExploitingAnt<ThreadRng>, G, AB, F, T>
where
  P: PheromoneUpdate,
  G: Goodness,
  AB: AntsBehaviour<ExploitingAnt<ThreadRng>, G>,
  F: Fitness,
  T: TerminationCondition<ExploitingAnt<ThreadRng>>,
{
  /// Creates the given number of [ExploitingAnt] with thread RNG
  pub fn with_standard_exploiting_ants(mut self, ants_number: usize, exploiting_rate: f64) -> Self {
    let ants = (0..ants_number)
      .map(|_| ExploitingAnt::<ThreadRng>::new(self.solution_size, exploiting_rate))
      .collect_vec();
    self.ants = Some(ants);
    self
  }
}

impl<P, A, G, AB, F> Builder<P, A, G, AB, F, IterationCond>
where
  P: PheromoneUpdate,
  A: Ant,
  G: Goodness,
  AB: AntsBehaviour<A, G>,
  F: Fitness,
{
  /// Sets iteration termination condition.
  ///
  /// ## Arguments
  /// * `iterations_limit` - maximal number of iterations.
  pub fn with_iteration_termination(mut self, iterations_limit: usize) -> Self {
    self.termination_cond = Some(IterationCond::new(iterations_limit));
    self
  }
}

impl<B, G, A, AB, F, T> Builder<MMAntSystemPU<B>, A, G, AB, F, T>
where
  B: BestPolicy,
  G: Goodness,
  A: Ant,
  AB: AntsBehaviour<A, G>,
  F: Fitness,
  T: TerminationCondition<A>,
{
  /// Sets the lower bound of pheromone value
  ///
  /// Panics if:
  /// * pheromone update wasn't set.
  /// * lower bound >= upper_bound
  /// * lower_bound < 0
  ///
  /// ## Arguments
  /// * `lower_bound` - Minimal possible pheromone value.
  /// * `upper_bound` - Maximal possible pheromone value.
  pub fn set_pheromone_bound(mut self, lower_bound: f64, upper_bound: f64) -> Self {
    assert!(
      lower_bound < upper_bound,
      "lower bound needs to be smaller than upper bound"
    );
    assert!(0.0 <= lower_bound, "lower bound must be greater or equal 0");

    let mut pu = self
      .pheromone_update
      .expect("Set pheromone update before setting its bounds");
    pu.upper_bound = upper_bound;
    pu.lower_bound = lower_bound;
    self.pheromone_update = Some(pu);
    self
  }
}

type AntSystemBuilder<R, T> =
  Builder<AntSystemPU, CanonicalAnt<R>, CanonicalGoodness, AntSystemAB, CanonicalFitness, T>;

impl<R, T> AntSystemBuilder<R, T>
where
  R: Rng,
  T: TerminationCondition<CanonicalAnt<R>>,
{
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
      termination_cond: None,
      start_pheromone: FMatrix::repeat(solution_size, solution_size, 1.0),
    }
  }
}

type MaxMinAntSystemBuilder<R, T> =
  Builder<MMAntSystemPU<OverallBest>, CanonicalAnt<R>, CanonicalGoodness, AntSystemAB, CanonicalFitness, T>;

impl<R: Rng, T: TerminationCondition<CanonicalAnt<R>>> MaxMinAntSystemBuilder<R, T> {
  /// Creates a new instance of [Builder] with operators used for MAX-MIN Ant System version of the algorithm.
  /// Best solution is chosen using [OverallBest].
  ///
  ///
  /// ### Defaults
  /// * `evaporation_rate` - 0.1
  /// * `start_pheromone` - matrix of 1.0
  /// * `probe` - [aco::probe::StdoutProbe]
  /// * `alpha` - 1.0
  /// * `beta` - 1.0
  /// * `heuristic` - matrix of 1.0
  /// * `lower_bound` - 0.0
  /// * `upper_bound` - 1.0
  pub fn new_mmas(solution_size: usize) -> Self {
    let pheromone_update = MMAntSystemPU::new(0.0, 1.0);
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
      termination_cond: None,
      start_pheromone: FMatrix::repeat(solution_size, solution_size, 1.0),
    }
  }
}

type AntColonySystemBuilder<L, R, T> = Builder<
  AntColonySystemPU<OverallBest>,
  ExploitingAnt<R>,
  CanonicalGoodness,
  AntColonySystemAB<L>,
  CanonicalFitness,
  T,
>;

impl<L: LocalUpdate, R: Rng, T: TerminationCondition<ExploitingAnt<R>>> AntColonySystemBuilder<L, R, T> {
  /// Creates a new instance of [Builder] with operators used for Ant Colony System version of the algorithm with provided local update rule.
  /// Best solution is chosen using [OverallBest].
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
  pub fn new_acs(solution_size: usize, local_update: L) -> Self {
    let pheromone_update = AntColonySystemPU::new();
    let goodness = CanonicalGoodness::new(1.0, 1.0, FMatrix::repeat(solution_size, solution_size, 1.0));
    let ants_behaviour = AntColonySystemAB::with_rule(local_update);
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
      termination_cond: None,
      start_pheromone: FMatrix::repeat(solution_size, solution_size, 1.0),
    }
  }
}
