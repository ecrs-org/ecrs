use crate::particle_swarm_optimization::swarm::Swarm;

pub trait Probe {
    fn on_begin(&mut self, swarm: &Swarm);
    fn on_end(&mut self, swarm: &Swarm);
    fn on_new_generation(&mut self, swarm: &Swarm, generation: usize);
}