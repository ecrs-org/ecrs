use crate::aco::ant::Ant;
use crate::aco::goodness::Goodness;
use crate::aco::{path_to_matrix, FMatrix};
use rand::Rng;

pub trait AntsBehaviour {
  fn calc_goodness(&mut self, pheromone: &FMatrix) -> FMatrix;
  fn simulate_ants(&mut self, pheromone: &mut FMatrix) -> Vec<FMatrix>;
}

pub struct AntSystemAB<R: Rng, G: Goodness> {
  ants: Vec<Ant<R>>,
  goodness: G,
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
