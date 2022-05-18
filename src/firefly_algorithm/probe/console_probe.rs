use crate::firefly_algorithm::probe::Probe;

pub struct ConsoleProbe {}

impl Probe for ConsoleProbe {
    fn on_iteration_start() {
        unimplemented!();
    }
    fn on_iteration_end() {
        unimplemented!();
    }
    fn on_new_best() {
        unimplemented!();
    }
    fn on_current_best() {
        unimplemented!();
    }
    fn on_end(){
        unimplemented!();
    }
}