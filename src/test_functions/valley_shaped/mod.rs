/// # Three-hump camel function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, 0) = 0
pub fn three_hump_camel(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Three-hump camel function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  2.0 * f64::powi(x1, 2) + -1.05 * f64::powi(x1, 4) + f64::powi(x1, 6) / 6.0 + x1 * x2 + f64::powi(x2, 2)
}

/// # Six-hump camel function
/// 2-dimensional \
/// Local mini
/// Global minima: 2\
/// f(0.898, -0.7126) = f(0.0898, 0.7126) = -1.0316
pub fn six_hump_camel(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Six-hump camel function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  (4_f64 - 2.1 * f64::powi(x1, 2) + f64::powi(x1, 3) / 3_f64) * f64::powi(x1, 2)
    + x1 * x2
    + (-4_f64 + 4_f64 * f64::powi(x2, 2)) * f64::powi(x2, 2)
}

/// # Dixon-Price function
/// n-dimensional \
/// Global minimum: \
/// f((x_1, ..., x_n)) = 0, where x_i = 2^( -(2^i - 2) / 2^i)
pub fn dixon_price(x: &Vec<f64>) -> f64 {
  assert!(
    x.len() >= 2,
    "Dixon-Price function takes an at least two dimensional vector as a parameter."
  );
  let mut result: f64 = f64::powi(x[0] - 1.0, 2);
  for i in 1..x.len() {
    result += (i as f64) * f64::powi(2.0 * f64::powi(x[i], 2) - x[i - 1], 2);
  }
  result
}

/// # Rosenbrock function
/// n-dimensional \
/// Global minimum: \
/// f(1, ..., 1) = 0
pub fn rosenbrock(x: &Vec<f64>) -> f64 {
  assert!(
    x.len() >= 2,
    "Rosenbrock function takes an at least two dimensional vector as a parameter."
  );
  let mut result: f64 = 0.0;
  for i in 0..x.len() - 1 {
    result += 100.0 * f64::powi(x[i + 1] - f64::powi(x[i], 2), 2) + f64::powi(1.0 - x[i], 2);
  }
  result
}
