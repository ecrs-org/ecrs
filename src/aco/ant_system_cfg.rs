use nalgebra::{Dynamic, OMatrix};

use crate::aco::probe::{ConsoleProbe, Probe};

type FMatrix = OMatrix<f64, Dynamic, Dynamic>;

pub struct AntSystemCfg {
    pub weights: FMatrix,
    pub heuristic: FMatrix,
    pub alpha: f64,
    pub beta: f64,
    pub evaporation_rate: f64,
    pub ants_num: usize,
    pub iteration: usize,
    pub probe: Box<dyn Probe>
}

impl Default for AntSystemCfg {
    fn default() -> Self {
        AntSystemCfg {
            weights: FMatrix::zeros(0, 0),
            heuristic: FMatrix::zeros(0, 0),
            alpha: 1.0,
            beta: 1.0,
            evaporation_rate: 0.1,
            ants_num: 10,
            iteration: 300,
            probe: Box::new(ConsoleProbe::new())
        }
    }
}