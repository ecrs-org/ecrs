pub mod console_probe;
pub mod csv_probe;
pub mod json_probe;

pub trait Probe {
    fn on_iteration_start();
    fn on_iteration_end();
    fn on_new_best();
    fn on_current_best();
    fn on_end();
}