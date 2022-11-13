use crate::ga::{Probe, individual::{Chromosome, ChromosomeWrapper}};

pub struct StdoutProbe {
  // no state held
}

impl<T: Chromosome, S: ChromosomeWrapper<T>> Probe<T, S> for StdoutProbe {
  fn on_start(&mut self) {
    println!("Execution of genetic algorithm started");
  }

  fn on_new_best(&mut self, individual: &S) {
    // TODO: Take new best solution as a parameter
    println!("New best: {:?}", individual);
  }

  fn on_mutation(&mut self, before: &S, after: &S) {
    // TODO: Take mutated chromosome and present it here (before & after)
    println!("Mutation {:?} ---> {:?}", before, after);
  }

  fn on_new_generation(&mut self, _generation: &[S]) {
    // TODO: Take reference to whole generation as a parameter and display it here!
    println!("New generation created")
  }

  fn on_best_fit_in_generation(&mut self, individual: &S) {
    // TODO: Take reference to the best chromosome & display it here!
    println!("Best fit in generation: {:?}, value: {}", individual, individual.get_fitness());
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
