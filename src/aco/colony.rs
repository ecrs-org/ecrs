use crate::aco::pheromone::Pheromone;
use crate::aco::AdditionalArgs;

pub trait Colony<P: Pheromone, Args: AdditionalArgs = ()> {
    fn build_solutions(&mut self, pheromone: &mut P, args: &Args) -> Vec<Vec<usize>>;
}
