use nalgebra::{Dynamic, OMatrix};
use crate::aco::ants_system_v2::{AntSystem, Solution};
use crate::aco::ants_system_v2::probe::{ConsoleProbe, Probe};

type FMatrix = OMatrix<f64, Dynamic, Dynamic>;

pub struct AntSystemBuilder {
    system: AntSystem,
}

impl AntSystemBuilder {
    pub fn from_weights(weights: FMatrix) -> AntSystemBuilder {
        let heuristic = create_heuristic_from_weights(&weights);
        let pheromone = FMatrix::repeat(weights.nrows(), weights.ncols(), 1.0);
        let best_sol = Solution::default();

        let system = AntSystem{
            weights,
            heuristic,
            pheromone,
            best_sol,
            alpha: 1.0,
            beta: 1.0,
            evaporation_rate: 0.1,
            ants_num: 32,
            iteration: 0,
            probe: Box::new(ConsoleProbe::new())
        };

        AntSystemBuilder{
            system
        }
    }

    pub fn set_heuristic(mut self, heuristic: FMatrix) -> Self {
        // TODO: Check for dimensions
        self.system.heuristic = heuristic;
        self
    }

    pub fn set_alpha(mut self, alpha: f64) -> Self {
        self.system.alpha = alpha;
        self
    }

    pub fn set_beta(mut self, beta: f64) -> Self {
        self.system.beta = beta;
        self
    }

    pub fn set_evaporation_rate(mut self, evaporation_rate: f64) -> Self {
        self.system.evaporation_rate = evaporation_rate;
        self
    }

    pub fn set_ants_num(mut self, ants_num: usize) -> Self {
        self.system.ants_num = ants_num;
        self
    }

    pub fn set_probe(mut self, probe: Box<dyn Probe>) -> Self {
        self.system.probe = probe;
        self
    }

    pub fn build(self) -> AntSystem {
        self.system
    }
}

fn create_heuristic_from_weights(weights: &FMatrix) -> FMatrix {
    let mut heu = FMatrix::zeros(weights.nrows(), weights.ncols());
    heu.iter_mut()
        .zip(weights.iter())
        .for_each(|(h, w)| if *w == 0.0 { *h = 0.0; } else { *h = 1.0 / *w });

    heu
}