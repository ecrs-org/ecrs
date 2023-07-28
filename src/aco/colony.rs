use crate::aco::ant::Ant;
use crate::aco::ants_behaviour::AntsBehaviour;
use crate::aco::goodness::Goodness;
use crate::aco::pheromone::Pheromone;
use crate::aco::AdditionalArgs;
use std::marker::PhantomData;

pub trait Colony<P: Pheromone, Args: AdditionalArgs = ()> {
    fn build_solutions(&mut self, pheromone: &mut P, args: &Args) -> Vec<Vec<usize>>;
}

pub struct LegacyColony<P: Pheromone, AB: AntsBehaviour<A, G, P>, A: Ant, G: Goodness<P>> {
    ants_behaviour: AB,
    goodness: G,
    ants: Vec<A>,
    _phantom: PhantomData<P>,
}

impl<P: Pheromone, AB: AntsBehaviour<A, G, P>, A: Ant, G: Goodness<P>> LegacyColony<P, AB, A, G> {
    pub fn new(ants_behaviour: AB, goodness: G, ants: Vec<A>) -> Self {
        Self {
            ants_behaviour,
            goodness,
            ants,
            _phantom: Default::default(),
        }
    }
}

impl<P, AB, A, G, Args> Colony<P, Args> for LegacyColony<P, AB, A, G>
where
    P: Pheromone,
    AB: AntsBehaviour<A, G, P>,
    A: Ant,
    G: Goodness<P>,
    Args: AdditionalArgs,
{
    fn build_solutions(&mut self, pheromone: &mut P, _: &Args) -> Vec<Vec<usize>> {
        self.ants_behaviour
            .simulate_ants(&mut self.ants, pheromone, &mut self.goodness)
    }
}
