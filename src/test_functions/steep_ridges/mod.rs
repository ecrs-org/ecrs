use std::f64;

///# De Jong n. 5 function
/// 2-dimensional only\

pub fn de_jong_n5(x: &Vec<f64>) -> f64 {
  assert_eq!(
    2,
    x.len(),
    "De Jong n.5 function takes only a two dimensional vector as a parameter."
  );
  let mut sum = 0_f64;
  let mut a1 = vec![];
  for _i in 1..6 {
    a1.append(&mut vec![-32.0, -16.0, 16.0, 32.0]);
  }
  let mut b: Vec<f64> = vec![];
  for i in (-32..33).step_by(16) {
    b.append(&mut vec![i as f64; 5]);
  }
  let a = vec![a1, b];
  for i in 1..26 {
    sum += 1_f64 / i as f64 + f64::powi(x[1] - a[1][i], 6) + f64::powi(x[2] - a[2][i], 6);
  }
  f64::powf(0.002 + sum, -1_f64)
}

/// # Easom function
/// 2-dimensional only \
/// Global minimum: \
/// f(PI, PI) = -1, where PI = 3.14159...
pub fn easom(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Easom function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  -1.0
    * f64::cos(x1)
    * f64::cos(x2)
    * f64::exp(-1.0 * (f64::powi(x1 - f64::consts::PI, 2) + f64::powi(x2 - f64::consts::PI, 2)))
}

/// # Michalewicz function
/// n-dimensional \
/// Global minimum: \
/// depends of number of dimensions \
/// for two-dimensional argument: f(2.2, 1.57) = -1.8013
pub fn michalewicz(x: &Vec<f64>) -> f64 {
  assert!(
    !x.is_empty(),
    "Michalewicz function takes an at least one dimensional vector as a parameter."
  );
  let mut result: f64 = 0.0;
  for (index, x_curr) in x.iter().enumerate() {
    result -= f64::sin(*x_curr)
      * f64::powi(
        f64::sin(((index + 1) as f64) * f64::powi(*x_curr, 2) / f64::consts::PI),
        20,
      );
  }
  result
}
