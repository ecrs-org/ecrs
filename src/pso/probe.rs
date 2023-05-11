pub mod aggregated_probe;
pub mod csv_probe;
pub mod json_probe;
pub mod policy_driven_probe;
pub mod probing_policy;
pub mod stdout_probe;

use crate::pso::swarm::Swarm;

pub trait Probe {
    fn on_begin(&mut self, swarm: &Swarm);
    fn on_end(&mut self, swarm: &Swarm, generation: usize);
    fn on_new_generation(&mut self, swarm: &Swarm, generation: usize);
}

pub trait ProbingPolicy {
    fn on_begin(&mut self) -> bool;
    fn on_end(&mut self, generation: usize) -> bool;
    fn on_new_generation(&mut self, generation: usize) -> bool;
}
