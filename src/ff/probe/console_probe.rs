use crate::ff::probe::Probe;

pub struct ConsoleProbe {}

impl ConsoleProbe {
  pub fn new() -> ConsoleProbe {
    ConsoleProbe {}
  }
}

impl Probe for ConsoleProbe {
  fn on_start(&mut self) {
    println!("##START##")
  }
  fn on_iteration_start(&mut self, num: &u32) {
    print!("Iteration nr {} ", num)
  }
  fn on_iteration_end(&mut self, _num: &u32) {
    println!(); //TODO CO TU
  }
  fn on_new_best(&mut self, newbest: &f64) {
    print!("New best result: {} ", newbest);
  }
  fn on_current_best(&mut self) {
    print!("No brighter fireflies found ");
  }
  fn on_end(&mut self) {
    println!("##END##");
  }
}
