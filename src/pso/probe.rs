pub mod console_probe;
pub mod csv_probe;
pub mod iteration_count_probe;
pub mod json_probe;
pub mod multi_probe;
pub mod timed_probe;

use crate::pso::swarm::Swarm;

pub trait Probe {
    fn on_begin(&mut self, swarm: &Swarm);
    fn on_end(&mut self, swarm: &Swarm);
    fn on_new_generation(&mut self, swarm: &Swarm, generation: usize);
}
