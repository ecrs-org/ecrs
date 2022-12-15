#[allow(clippy::ptr_arg)]
pub fn rastrigin_fitness(chromosome: &Vec<f64>) -> f64 {
  1000.0 * f64::exp(-rastrigin(chromosome))
}

fn rastrigin(chromosome: &[f64]) -> f64 {
  10.0 * chromosome.len() as f64
    + chromosome.iter().fold(0.0, |sum, x| {
      sum + x.powi(2) - 10.0 * (2.0 * std::f64::consts::PI * x).cos()
    })
}
