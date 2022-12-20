//! Ants simulation strategies
//!
//! Contains strategies on how should an ant behave, and when to update edge [crate::aco::goodness].
use crate::aco::ant::Ant;
use crate::aco::goodness::Goodness;
use crate::aco::{path_to_matrix, FMatrix};
use itertools::Itertools;
use rand::rngs::ThreadRng;
use rand::Rng;

/// # Ants Behaviour
///
/// Trait contains common actions of ants simulation.
pub trait AntsBehaviour {
  /// Calculates goodness based on provided pheromone. Calculations are often delegated to an object
  /// implementing [Goodness] trait.
  ///
  /// ## Arguments
  /// * `pheromone` - Pheromone to base goodness of.
  fn calc_goodness(&mut self, pheromone: &FMatrix) -> FMatrix;

  /// Simulates ant by deciding on order of operations.
  ///
  /// ## Arguments
  /// * `pheromone` - Pheromone after global pheromone update rule was applied.
  fn simulate_ants(&mut self, pheromone: &mut FMatrix) -> Vec<FMatrix>;
}
/// # Ant System ants behaviour
///
/// Implements [AntsBehaviour]. Ants are simulated as described in Ant System algorithm with the
/// exception of goodness calculation. By providing [crate::aco::goodness::CanonicalGoodness] simulations
/// will be fully equivalent to Ant System.
pub struct AntSystemAB<R: Rng, G: Goodness> {
  pub(crate) ants: Vec<Ant<R>>,
  pub(crate) goodness: G,
}

impl<G: Goodness> AntSystemAB<ThreadRng, G> {
  /// Creates a new instance of [AntSystemAB]. With default RNG
  ///
  /// ## Arguments
  /// * `ants_number` - number of ants to simulate.
  /// * `solution_size` - number of graph vertices
  /// * `goodness` - goodness calculation struct.
  pub fn new(ants_number: usize, solution_size: usize, goodness: G) -> Self {
    let ants = (0..ants_number).map(|_| Ant::new(solution_size)).collect_vec();
    Self::with_ants(ants, goodness)
  }
}

impl<R: Rng, G: Goodness> AntSystemAB<R, G> {
  /// Creates a new instance of [AntSystemAB] with user initialized ants.
  ///
  /// ## Arguments
  /// * `ants` - vector of ants.
  /// * `goodness` - goodness calculation struct.
  pub fn with_ants(ants: Vec<Ant<R>>, goodness: G) -> Self {
    Self { ants, goodness }
  }
}

impl<R: Rng, G: Goodness> AntsBehaviour for AntSystemAB<R, G> {
  fn calc_goodness(&mut self, pheromone: &FMatrix) -> FMatrix {
    self.goodness.apply(pheromone)
  }

  fn simulate_ants(&mut self, pheromone: &mut FMatrix) -> Vec<FMatrix> {
    let goodness = self.calc_goodness(pheromone);
    let solution_size = pheromone.nrows();

    let mut sols: Vec<FMatrix> = Vec::with_capacity(self.ants.len());
    for ant in self.ants.iter_mut() {
      ant.clear();
      ant.chose_staring_place();
      for _ in 1..solution_size {
        ant.go_to_next_place(&goodness);
      }

      if ant.is_stuck() {
        break;
      }
      let path = ant.get_path();
      sols.push(path_to_matrix(path));
    }

    sols
  }
}
