use nalgebra::{Dynamic, OMatrix};
pub use ant_system_cfg::AntSystemCfg;
pub use ants_system_v2::AntSystem;
pub use ants_system_v2::probe;

mod ants_system_v2;
mod ant_system_cfg;

type FMatrix = OMatrix<f64, Dynamic, Dynamic>;

pub fn create_heuristic_from_weights(weights: &FMatrix) -> FMatrix {
    let mut heu = FMatrix::zeros(weights.nrows(), weights.ncols());
    heu.iter_mut()
        .zip(weights.iter())
        .for_each(|(h, w)| if *w == 0.0 { *h = 0.0; } else { *h = 1.0 / *w });

    heu
}

