use crate::pso::swarm::Swarm;
use std::time::Instant;

pub trait TerminationCondition {
  fn initialize(&mut self, swarm: &Swarm);
  fn is_met(&self, generation: usize, swarm: &Swarm) -> bool;
}

pub struct GenerationLimit {
  generation_limit: usize,
}

impl GenerationLimit {
  pub fn new(generation_limit: usize) -> GenerationLimit {
    GenerationLimit { generation_limit }
  }
}

impl TerminationCondition for GenerationLimit {
  fn initialize(&mut self, _swarm: &Swarm) {}

  fn is_met(&self, generation: usize, _swarm: &Swarm) -> bool {
    generation >= self.generation_limit
  }
}

pub struct TimeLimit {
  start_time: Instant,
  time_limit: usize,
}

impl TimeLimit {
  pub fn new(time_limit: usize) -> TimeLimit {
    TimeLimit {
      start_time: Instant::now(),
      time_limit,
    }
  }
}

impl TerminationCondition for TimeLimit {
  fn initialize(&mut self, _swarm: &Swarm) {
    self.start_time = Instant::now();
  }

  fn is_met(&self, _generation: usize, _swarm: &Swarm) -> bool {
    self.start_time.elapsed().as_secs() >= self.time_limit as u64
  }
}
