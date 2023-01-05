use std::time::Instant;

pub trait TerminationCondition{
  fn is_met(&self, gen:Option<i32>)->bool;
}

pub struct TimeElapsed{
  start_time: Instant,
  time_limit: f64,
}

impl TerminationCondition for TimeElapsed{
  fn is_met(&self, _gen:Option<i32>) -> bool {
    (self.time_limit as u64) < self.start_time.elapsed().as_secs()
  }
}

pub struct GenerationsElapsed{
  pub(crate) generation_limit: i32,
}

impl TerminationCondition for GenerationsElapsed{
  fn is_met(&self, gen: Option<i32>) -> bool {
    self.generation_limit>gen.unwrap()
  }
}