/// # Booth function
/// 2-dimensional only \
/// Global minimum: \
/// f(1, 3) = 0
pub fn booth(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Booth function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::powi(x1 + 2.0 * x2 - 7.0, 2) + f64::powi(2.0 * x1 + x2 - 5.0, 2)
}

/// # Matyas function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, 0) = 0
pub fn matyas(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Matyas function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  0.26 * (f64::powi(x1, 2) + f64::powi(x2, 2)) + -0.48 * x1 * x2
}

/// # McCormick function
/// 2-dimensional only \
/// Global minimum: \
/// f(-0.54719, -1.54719) = -1.9133
pub fn mcormick(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "McCormick table function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::sin(x1 + x2) + f64::powi(x1 - x2, 2) + -1.5 * x1 + 2.5 * x2 + 1.0
}

/// # Power sum function
/// D-dimensional, where b is a vector of length D \
/// Global minimum: \
/// f(0,...,0) = 0

pub fn power_sum(x: &Vec<f64>, b: &Vec<f64>) -> f64 {
  assert_eq!(
    b.len(),
    x.len(),
    "Power sum function requires b to be equal in length to number of dimensions"
  );
  let mut res = 0_f64;
  for (i, _) in b.iter().enumerate() {
    let mut sum = 0_f64;
    for j in x.iter() {
      sum += f64::powi(*j, i as i32);
    }
    res += f64::powi(sum - b[i], 2);
  }
  res
}

/// # Zakharov function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn zakharov(x: &Vec<f64>) -> f64 {
  assert!(
    !x.is_empty(),
    "Zakharov function takes an at least one dimensional vector as a parameter."
  );
  let mut temp1 = 0.0;
  let mut temp2 = 0.0;
  for (index, x_curr) in x.iter().enumerate() {
    temp1 += f64::powi(*x_curr, 2);
    temp2 += 0.5 * ((index + 1) as f64) * *x_curr;
  }
  temp1 + f64::powi(temp2, 2) + f64::powi(temp2, 4)
}
