pub mod console_probe;
pub mod csv_probe;
pub mod json_probe;

pub trait Probe {
  fn on_start(&mut self);
  fn on_iteration_start(&mut self, num: &u32);
  fn on_iteration_end(&mut self, num: &u32);
  fn on_new_best(&mut self, newbest: &f64);
  fn on_current_best(&mut self);
  fn on_end(&mut self);
}
