use crate::aco::FMatrix;
use itertools::Itertools;
use rand::Rng;
use std::error::Error;

pub fn into_vec(m: &FMatrix) -> Vec<Vec<f64>> {
    let mut m_vec: Vec<Vec<f64>> = Vec::new();

    for row in m.row_iter() {
        m_vec.push(row.iter().copied().collect_vec());
    }
    m_vec
}

/// Utility function for generating heuristic from cost(weights)
pub fn create_heuristic_from_weights(weights: &FMatrix) -> FMatrix {
    let mut heu = FMatrix::zeros(weights.nrows(), weights.ncols());
    heu.iter_mut().zip(weights.iter()).for_each(|(h, w)| {
        if *w == 0.0 {
            *h = 0.0;
        } else {
            *h = 1.0 / *w
        }
    });

    heu
}

/// Utility function for generating TSP input data.
///
/// ## Arguments
/// * `sol_size` - number of cites.
pub fn generate_tsp_cost(sol_size: usize) -> (Vec<(f64, f64)>, FMatrix) {
    let mut cities: Vec<(f64, f64)> = Vec::new();
    let mut r = rand::thread_rng();
    for _ in 0..sol_size {
        let x: f64 = r.gen_range(0.0..100.0);
        let y: f64 = r.gen_range(0.0..100.0);

        cities.push((x, y))
    }

    let mut cost: FMatrix = FMatrix::zeros(sol_size, sol_size);
    for i in 0..sol_size {
        for j in i..sol_size {
            let (x1, y1) = cities[i];
            let (x2, y2) = cities[j];
            let x = x1 - x2;
            let y = y1 - y2;

            let dist = f64::sqrt(x * x + y * y);

            cost[(i, j)] = dist;
            cost[(j, i)] = dist;
        }
    }

    (cities, cost)
}

/// Utility function for writing TSP input data to a CSV file.
///
/// ## Arguments
/// * `cities` - Vector of tuples representing cities x and y positions
/// * `path` - Where to save file.
pub fn write_cities_csv(cities: &[(f64, f64)], path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(["x", "y"])?;
    for (x, y) in cities.iter() {
        wtr.write_record(&[x.to_string(), y.to_string()])?;
    }
    wtr.flush()?;

    Ok(())
}
