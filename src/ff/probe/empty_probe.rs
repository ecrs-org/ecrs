use crate::ff::probe::Probe;

pub struct EmptyProbe {}
impl Probe for EmptyProbe {
    fn on_start(&mut self) {}

    fn on_iteration_start(&mut self, _iteration: u32) {}

    fn on_iteration_end(&mut self, _iteration: u32) {}

    fn on_current_best(&mut self, _solution: f64, _position: &[f64]) {}

    fn on_end(&mut self) {}
}
