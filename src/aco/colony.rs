use std::marker::PhantomData;
use crate::aco::ant::Ant;
use crate::aco::ants_behaviour::AntsBehaviour;
use crate::aco::goodness::Goodness;
use crate::aco::pheromone::Pheromone;

pub trait Colony<P: Pheromone> {
  fn build_solutions(&mut self, pheromone: &mut P) -> Vec<Vec<usize>>;
}

pub struct LegacyColony<P: Pheromone, AB: AntsBehaviour<A, G, P>, A: Ant, G: Goodness<P>> {
  ants_behaviour: AB,
  goodness: G,
  ants: Vec<A>,
  _phantom: PhantomData<P>
}

impl<P: Pheromone, AB: AntsBehaviour<A, G, P>, A: Ant, G: Goodness<P>> LegacyColony<P, AB, A, G> {
  pub fn new(ants_behaviour: AB, goodness: G, ants: Vec<A>) -> Self {
    Self { ants_behaviour, goodness, ants, _phantom: Default::default() }
  }
}

impl<P: Pheromone, AB: AntsBehaviour<A, G, P>, A: Ant, G: Goodness<P>> Colony<P> for LegacyColony<P,AB,A,G> {
  fn build_solutions(&mut self, pheromone: &mut P) -> Vec<Vec<usize>> {
    self.ants_behaviour.simulate_ants(&mut self.ants, pheromone, &mut self.goodness)
  }
}

