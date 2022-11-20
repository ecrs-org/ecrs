use std::f64;

pub fn rastrigin(params: &Vec<f64>) -> f64 {
    let mut res = 0 as f64;
    for param in params.iter() {
        res += param * param - 10_f64 * f64::cos(2_f64 * f64::consts::PI * param);
    }
    res + 10_f64 * params.len() as f64
}
