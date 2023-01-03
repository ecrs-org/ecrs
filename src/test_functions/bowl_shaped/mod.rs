use std::f64;

/// # Bohachevsky N.1 function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, 1.25313) = 0.292579 \

pub fn bohachevsky_n1(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Bohachevsky function N.1 takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::powi(x1, 2) + 2_f64 * f64::powi(x2, 2)
    - 0.3 * f64::cos(3_f64 * f64::consts::PI * x1)
    - 0.4 * f64::cos(4_f64 * f64::consts::PI * x2)
    + 0.7
}

/// # Bohhachevsky N.2 function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, 1.25313) = 0.292579 \

pub fn bohachevsky_n2(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Bohhachevsky function N.2 takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::powi(x1, 2) + 2_f64 * f64::powi(x2, 2)
    - 0.3 * f64::cos(3_f64 * f64::consts::PI * x1) * f64::cos(4_f64 * f64::consts::PI * x2)
    + 0.3
}

/// # Bohhachevsky N.3 function
/// 2-dimensional only \
/// Global minimum: \
/// f(0,0) = 0

pub fn bohachevsky_n3(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Bohhachevsky function N.3 takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::powi(x1, 2) + 2_f64 * f64::powi(x2, 2)
    - 0.3 * f64::cos(3_f64 * f64::consts::PI * x1 + 4_f64 * f64::consts::PI * x2)
    + 0.3
}

///# Perm 0, D, Beta function
/// 2-dimensional only\
/// Global minimum: \
/// f(1, 0.5, ... , 1/d) = 0
pub fn perm_0_d_beta(x: &Vec<f64>, beta: &f64) -> f64 {
  let d = x.len();
  let mut res = 0_f64;
  for i in 1..d + 1 {
    let mut sum = 0_f64;
    for (j, _) in x.iter().enumerate() {
      sum += (j as f64 + beta) * f64::powi(x[j], i as i32) - 1_f64 / (f64::powi(j as f64, i as i32));
    }
    res += f64::powi(sum, 2);
  }
  res
}

/// # Rotated Hyper-Ellipsoid function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn rotated_hyper_ellipsoid(x: &Vec<f64>) -> f64 {
  assert!(
    !x.is_empty(),
    "Rotated Hyper-Ellipsoid function takes an at least one dimensional vector as a parameter."
  );
  let mut result: f64 = 0.0;
  for i in 0..x.len() {
    for x_j in x.iter().take(i + 1) {
      result += f64::powi(*x_j, 2);
    }
  }
  result
}

/// # Sphere function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn sphere(x: &Vec<f64>) -> f64 {
  assert!(
    !x.is_empty(),
    "Sphere function takes an at least one dimensional vector as a parameter."
  );
  let mut result: f64 = 0.0;
  for x_curr in x {
    result += f64::powi(*x_curr, 2)
  }
  result
}

/// # Sum of Different Powers function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn sum_of_powers(x: &Vec<f64>) -> f64 {
  assert!(
    !x.is_empty(),
    "Sum of Different Powers function takes an at least one dimensional vector as a parameter."
  );
  let mut result: f64 = 0.0;
  for (index, x_curr) in x.iter().enumerate() {
    result += f64::powi(f64::abs(*x_curr), (index + 1) as i32);
  }
  result
}

/// # Sum Squares (Axis Parallel Hyper-Ellipsoid) function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn sum_squares(x: &Vec<f64>) -> f64 {
  assert!(
    !x.is_empty(),
    "Sum Squares function takes an at least one dimensional vector as a parameter."
  );
  let mut result: f64 = 0.0;
  for (index, x_curr) in x.iter().enumerate() {
    result += ((index + 1) as f64) * f64::powi(*x_curr, 2);
  }
  result
}

/// # Trid function
/// n-dimensional \
/// Global minimum: \
/// f((x_1, ..., x_n)) = -n * ( n + 4 ) * ( n - 1 ) / 6, where x_i = i * (n + 1 - i)
pub fn trid(x: &Vec<f64>) -> f64 {
  assert!(
    x.len() >= 2,
    "Trid function takes an at least two dimensional vector as a parameter."
  );
  let mut result: f64 = f64::powi(x[0] - 1.0, 2);
  for i in 1..x.len() {
    result += f64::powi(x[i] - 1.0, 2) - x[i] * x[i - 1];
  }
  result
}
