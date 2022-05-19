use crate::ga::Individual;

pub trait Probe {
  fn on_start(&mut self)                      { /* defaults to noop */ }
  fn on_new_best(&mut self, individual: &Individual)                   { /* defaults to noop */ }
  fn on_mutation(&mut self, before: &Individual, after: &Individual)                   { /* defaults to noop */ }
  fn on_new_generation(&mut self)             { /* defaults to noop */ }
  fn on_best_fit_in_generation(&mut self, individual: &Individual)     { /* defaults to noop */ }
  fn on_iteration_start(&mut self, iteration: usize)            { /* defaults to noop */ }
  fn on_iteration_end(&mut self, iteration: usize)              { /* defaults to noop */ }
}

pub struct GAStdoutProbe {
  // no state held
}

impl Probe for GAStdoutProbe {
  fn on_start(&mut self) {
    println!("Execution of genetic algorithm started");
  }

  fn on_new_best(&mut self, individual: &Individual) {
    // TODO: Take new best solution as a parameter
    println!("New best: {:?}", individual);
  }

  fn on_mutation(&mut self, before: &Individual, after: &Individual) {
    // TODO: Take mutated chromosome and present it here (before & after)
    println!("Mutation {:?} ---> {:?}", before, after);
  }

  fn on_new_generation(&mut self) {
    // TODO: Take reference to whole generation as a parameter and display it here!
    println!("New generation created")
  }

  fn on_best_fit_in_generation(&mut self, individual: &Individual) {
    // TODO: Take reference to the best chromosome & display it here!
    println!("Best fit in generation: {:?}", individual);
  }

  fn on_iteration_start(&mut self, iteration: usize) {
    // TODO: Take iteration count & maybe some more info here (best so far, etc.)
    println!("Start of iteration: {}", iteration);
  }

  fn on_iteration_end(&mut self, iteration: usize) {
    // TODO: Take iteration count & maybe some more info here (best so far, etc.)
    println!("End of iteration: {}", iteration);
  }
}
