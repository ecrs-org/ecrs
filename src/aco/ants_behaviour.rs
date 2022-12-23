//! Ants simulation strategies
//!
//! Contains strategies on how should an ant behave, and when to update edge [crate::aco::goodness].
use crate::aco::ant::Ant;
use crate::aco::goodness::Goodness;
use crate::aco::FMatrix;

/// # Ants Behaviour
///
/// Trait contains common actions of ants simulation.
pub trait AntsBehaviour<A: Ant, G: Goodness> {
  /// Simulates ant by deciding on order of operations.
  ///
  /// ## Arguments
  /// * `pheromone` - Pheromone after global pheromone update rule was applied.
  /// * `ants` - ants to be simulated
  /// * `goodness_op` - Implementation of [Goodness].
  fn simulate_ants(
    &mut self,
    ants: &mut [A],
    pheromone: &mut FMatrix,
    goodness_op: &mut G,
  ) -> Vec<Vec<usize>>;
}
/// # Ant System ants behaviour
///
/// Implements [AntsBehaviour]. Ants are simulated as described in Ant System algorithm with the
/// exception of goodness calculation. By providing [crate::aco::goodness::CanonicalGoodness] simulations
/// will be fully equivalent to Ant System.
pub struct AntSystemAB;

impl<A: Ant, G: Goodness> AntsBehaviour<A, G> for AntSystemAB {
  fn simulate_ants(
    &mut self,
    ants: &mut [A],
    pheromone: &mut FMatrix,
    goodness_op: &mut G,
  ) -> Vec<Vec<usize>> {
    let goodness = goodness_op.apply(pheromone);
    let solution_size = pheromone.nrows();

    let mut paths: Vec<Vec<usize>> = Vec::with_capacity(ants.len());
    for ant in ants.iter_mut() {
      ant.clear();
      ant.chose_staring_place();
      for _ in 1..solution_size {
        ant.go_to_next_place(&goodness);
      }

      if ant.is_stuck() {
        break;
      }
      let path = ant.path();
      paths.push(path.to_vec())
    }

    paths
  }
}

/// # Ant Colony System ants behaviour
///
/// Implements [AntsBehaviour]. Ants are simulated as described in Ant Colony System algorithm with the
/// exception of goodness calculation.
pub struct AntColonySystemAB;

impl<A: Ant, G: Goodness> AntsBehaviour<A, G> for AntColonySystemAB {
  fn simulate_ants(
    &mut self,
    ants: &mut [A],
    pheromone: &mut FMatrix,
    goodness_op: &mut G,
  ) -> Vec<Vec<usize>> {
    let solution_size = pheromone.nrows();

    ants.iter_mut().for_each(|a| {
      a.clear();
      a.chose_staring_place()
    });

    let mut paths: Vec<Vec<usize>> = Vec::with_capacity(ants.len());

    for _ in 1..solution_size {
      let goodness = goodness_op.apply(pheromone);
      paths.clear();
      for ant in ants.iter_mut() {
        if ant.is_stuck() {
          continue;
        }
        ant.go_to_next_place(&goodness);

        let path = ant.path();
        paths.push(path.to_vec())
      }
      local_update(pheromone, &paths)
    }
    paths
  }
}

fn local_update(pheromone: &mut FMatrix, partial_paths: &[Vec<usize>]) {
  for p_path in partial_paths {
    let l = p_path.len();
    let s = p_path[l - 2];
    let r = p_path[l - 1];

    pheromone[(s, r)] *= 0.9
  }
}
