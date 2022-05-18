extern crate core;

use std::error::Error;

use nalgebra::{Dynamic, OMatrix};
use rand::Rng;

use crate::aco::probe::CsvProbe;
use crate::aco::AntSystemCfg;

mod aco;

// DOI: 10.1109/MCI.2006.329691
// http://www.scholarpedia.org/article/Ant_colony_optimization

type FMatrix = OMatrix<f64, Dynamic, Dynamic>;


fn generate_tsp_cost(sol_size: usize) -> (Vec<(f64,f64)>, FMatrix) {
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


fn write_cities_csv(cities: &Vec<(f64, f64)>, path: &str) -> Result<(), Box<dyn Error>> {
  let mut wtr = csv::Writer::from_path(path)?;
  wtr.write_record(&["x", "y"])?;
  for (x,y) in cities.iter() {
    wtr.write_record(&[x.to_string(), y.to_string()])?;
  }
  wtr.flush()?;

  Ok(())
}

fn main() {

  let (cities, cost) = generate_tsp_cost(60);
  write_cities_csv(&cities, "cities.csv").expect("Error while writing city file");

  let probe = Box::new(CsvProbe::new());
  let heuristic = aco::create_heuristic_from_weights(&cost);

  let mut ant_s = aco::AntSystem::new(AntSystemCfg {
    weights: cost,
    heuristic,
    ..AntSystemCfg::default()
  });

  for _ in 0..300 {
    ant_s.iterate();
  }


  ant_s.end();


}
