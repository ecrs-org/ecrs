use std::f64;

pub fn rastrigin(params: &Vec<f64>) -> f64 {
    let mut res = 0 as f64;
    for param in params.iter() {
        res += param * param - 10_f64 * f64::cos(2_f64 * f64::consts::PI * param);
    }
    res + 10_f64 * params.len() as f64
}

pub fn cartesian_distance(a: &Vec<f64>, b: &[f64]) -> f64 {
    //Distance between two points
    let mut res: f64 = 0 as f64;
    for dimension in 0..a.len() {
        res += f64::powi(a[dimension] - b[dimension], 2)
    }
    f64::sqrt(res)
}

pub fn taxi_measure_distance(a: &Vec<f64>, b: &[f64]) -> f64 {
    let mut res: f64 = 0 as f64;
    for dimension in 0..a.len() {
        res += f64::abs(a[dimension] - b[dimension])
    }
    res
}
