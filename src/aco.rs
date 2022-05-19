use std::error::Error;
use nalgebra::{Dynamic, OMatrix};
use rand::Rng;
pub use ant_system_cfg::AntSystemCfg;
pub use ants_system_v2::AntSystem;
pub use ants_system_v2::probe;

mod ants_system_v2;
mod ant_system_cfg;

// DOI: 10.1109/MCI.2006.329691
// http://www.scholarpedia.org/article/Ant_colony_optimization

pub type FMatrix = OMatrix<f64, Dynamic, Dynamic>;

pub fn create_heuristic_from_weights(weights: &FMatrix) -> FMatrix {
    let mut heu = FMatrix::zeros(weights.nrows(), weights.ncols());
    heu.iter_mut()
        .zip(weights.iter())
        .for_each(|(h, w)| if *w == 0.0 { *h = 0.0; } else { *h = 1.0 / *w });

    heu
}

pub fn generate_tsp_cost(sol_size: usize) -> (Vec<(f64,f64)>, FMatrix) {
    let mut cities: Vec<(f64,f64)> = Vec::new();
    let mut r = rand::thread_rng();
    for _ in 0..sol_size {
        let x: f64 = r.gen_range(0.0..100.0);
        let y: f64 = r.gen_range(0.0..100.0);

        cities.push((x,y))
    }

    let mut cost: FMatrix = FMatrix::zeros(sol_size, sol_size);
    for i in 0..sol_size {
        for j in i..sol_size {
            let (x1,y1) = cities[i];
            let (x2,y2) = cities[j];
            let x = x1 - x2;
            let y = y1 - y2;

            let dist = f64::sqrt(x*x + y*y);

            cost[(i,j)] = dist;
            cost[(j,i)] = dist;
        }
    }


    (cities, cost)
}

pub fn write_cities_csv(cities: &Vec<(f64, f64)>, path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(&["x", "y"])?;
    for (x,y) in cities.iter() {
        wtr.write_record(&[x.to_string(), y.to_string()])?;
    }
    wtr.flush()?;

    Ok(())
}