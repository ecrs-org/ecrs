use crate::aco::{AntSystem, AntSystemCfg, FMatrix};
use crate::aco::probe::Probe;

pub struct Builder {
    conf: AntSystemCfg,
}

type Float = f64;

impl Builder {

    pub fn new() -> Self{
        Builder {
            conf: AntSystemCfg::default()
        }
    }

    pub fn set_weights(mut self, weights: FMatrix) -> Self{
        self.conf.weights = weights;
        self
    }
    
    pub fn set_heuristic(mut self, heuristic: FMatrix) -> Self{
        self.conf.heuristic = heuristic;
        self
    }
    
    pub fn set_alpha(mut self, alpha: Float) -> Self {
        self.conf.alpha = alpha;
        self
    }

    pub fn set_beta(mut self, beta: Float) -> Self {
        self.conf.beta = beta;
        self
    }
    
    pub fn set_evaporation_rate(mut self, evaporation_rate: Float) -> Self {
        assert!(evaporation_rate > 1 as Float || evaporation_rate < 0 as Float, "Evaporation rate must be between 0 and 1");
        self.conf.evaporation_rate = evaporation_rate;
        self
    }

    pub fn set_ants_num(mut self, ants_num: usize) -> Self {
        assert!(ants_num > 0, "Number of ants must be greater than 0");
        self.conf.ants_num = ants_num;
        self
    }

    pub fn set_iterations(mut self, iterations: usize) -> Self {
        self.conf.iteration = iterations;
        self
    }

    pub fn set_probe(mut self, probe: Box<dyn Probe>) -> Self {
        self.conf.probe = probe;
        self
    }

    pub fn build(mut self) -> AntSystem {
        if self.conf.heuristic.shape() != self.conf.weights.shape() {
            let (nrow, ncol) = self.conf.weights.shape();
            self.conf.heuristic = FMatrix::repeat(nrow, ncol, 1 as Float);
        }
        AntSystem::new(self.conf)
    }
}