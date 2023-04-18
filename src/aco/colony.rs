use crate::aco::pheromone::Pheromone;

pub trait Colony<P: Pheromone> : Copy {
  fn build_solutions(&mut self, pheromone: &mut P) -> Vec<Vec<usize>>;
}

