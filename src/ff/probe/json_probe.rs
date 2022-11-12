use crate::ff::probe::Probe;

pub struct JSONProbe {}

impl JSONProbe {
    fn new() -> JSONProbe {
        JSONProbe {}
    }
}

impl Probe for JSONProbe {
    fn on_start(&mut self) { unimplemented!() }
    fn on_iteration_start(&mut self, _num: &u32) {
        unimplemented!();
    }
    fn on_iteration_end(&mut self, _num: &u32) {
        unimplemented!();
    }
    fn on_new_best(&mut self, _newbest: &f64) {
        unimplemented!();
    }
    fn on_current_best(&mut self) {
        unimplemented!();
    }
    fn on_end(&mut self) {
        unimplemented!();
    }
}