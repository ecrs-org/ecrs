use crate::pso::probe::Probe;
use crate::pso::swarm::Swarm;

pub struct ConsoleProbe {}

impl ConsoleProbe {
  pub fn new() -> ConsoleProbe {
    ConsoleProbe {}
  }
}

impl Probe for ConsoleProbe {
  fn on_begin(&mut self, swarm: &Swarm) {
    println!("Swarm at the start:\n{}", swarm);
  }

  fn on_end(&mut self, swarm: &Swarm, generation: usize) {
    println!("\nSwarm at the end:\n{}", swarm);
  }

  fn on_new_generation(&mut self, swarm: &Swarm, generation: usize) {
    println!(
      "\nSwarm after {} {}:\n{}",
      generation,
      if generation == 1 {
        "generation"
      } else {
        "generations"
      },
      swarm
    );
  }
}
