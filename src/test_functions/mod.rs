use std::f64;

/// # Rastrigin function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn rastrigin(x: &Vec<f64>) -> f64 {
  assert!(
    !x.is_empty(),
    "Rastrigin function takes an at least one dimensional vector as a parameter."
  );
  let mut result: f64 = 0.0;
  for x_curr in x {
    result += f64::powi(*x_curr, 2) - 10.0 * f64::cos(2.0 * f64::consts::PI * *x_curr);
  }
  result += 10.0 * x.len() as f64;
  result
}

/// # Ackley function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn ackley(x: &Vec<f64>) -> f64 {
  assert!(
    !x.is_empty(),
    "Ackley function takes an at least one dimensional vector as a parameter."
  );
  let mut temp1 = 0.0;
  let mut temp2 = 0.0;
  for x_curr in x {
    temp1 += f64::powi(*x_curr, 2);
    temp2 += f64::cos(2.0 * f64::consts::PI * *x_curr)
  }
  temp1 *= 1.0 / x.len() as f64;
  temp2 *= 1.0 / x.len() as f64;
  -20.0 * f64::exp(-0.2 * f64::sqrt(temp1)) - f64::exp(temp2) + 20.0 + f64::exp(1.0)
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

/// # Styblinski-Tang function
/// n-dimensional \
/// Global minimum: \
/// f(-2.903534, ..., -2.903534) = -39.16599n, where n - number of dimensions of argument vector
pub fn styblinski_tang(x: &Vec<f64>) -> f64 {
  assert!(
    !x.is_empty(),
    "Styblinski-Tang function takes an at least one dimensional vector as a parameter."
  );
  let mut result: f64 = 0.0;
  for x_curr in x {
    result += f64::powi(*x_curr, 4) - 16.0 * f64::powi(*x_curr, 2) + 5.0 * *x_curr;
  }
  result *= 0.5;
  result
}

/// # Griewank function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn griewank(x: &Vec<f64>) -> f64 {
  assert!(
    !x.is_empty(),
    "Griewank function takes an at least one dimensional vector as a parameter."
  );
  let mut temp1 = 0.0;
  let mut temp2 = 1.0;
  for (index, x_curr) in x.iter().enumerate() {
    temp1 += f64::powi(*x_curr, 2) / 4000.0;
    temp2 *= f64::cos(*x_curr / f64::sqrt((index + 1) as f64));
  }
  temp1 - temp2 + 1.0
}

/// # Schwefel function
/// n-dimensional \
/// Global minimum: \
/// f(420.9687, ..., 420.9687) = 0
pub fn schwefel(x: &Vec<f64>) -> f64 {
  assert!(
    !x.is_empty(),
    "Schwefel function takes an at least one dimensional vector as a parameter."
  );
  let mut result: f64 = 418.9829 * (x.len() as f64);
  for x_curr in x {
    result -= *x_curr * f64::sin(f64::sqrt(f64::abs(*x_curr)));
  }
  result
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

/// # Levy function
/// n-dimensional \
/// Global minimum: \
/// f(1, ..., 1) = 0
pub fn levy(x: &Vec<f64>) -> f64 {
  assert!(
    !x.is_empty(),
    "Levy function takes an at least one dimensional vector as a parameter."
  );
  let mut result: f64 = f64::powi(f64::sin(f64::consts::PI * (1.0 + (x[0] - 1.0) / 4.0)), 2);
  for x_i in x.iter().take(x.len() - 1) {
    let temp: f64 = 1.0 + (x_i - 1.0) / 4.0;
    result += f64::powi(temp - 1.0, 2) * (1.0 + 10.0 * f64::powi(f64::sin(f64::consts::PI * temp + 1.0), 2));
  }
  let temp: f64 = 1.0 + (x[x.len() - 1] - 1.0) / 4.0;
  result += f64::powi(temp - 1.0, 2) * (1.0 + f64::powi(f64::sin(2.0 * f64::consts::PI * temp), 2));
  result
}

/// #  Beale function
/// 2-dimensional only \
/// Global minimum: \
/// f(3, 0.5) = 0
pub fn beale(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Beale function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::powi(1.5 - x1 + x1 * x2, 2)
    + f64::powi(2.25 - x1 + x1 * f64::powi(x2, 2), 2)
    + f64::powi(2.625 - x1 + x1 * f64::powi(x2, 3), 2)
}

/// # Goldstein-Price function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, -1) = 3
pub fn goldstein_price(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Goldstein-Price function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  (1.0
    + f64::powi(x1 + x2 + 1.0, 2)
      * (19.0 - 14.0 * x1 + 3.0 * f64::powi(x1, 2) - 14.0 * x2 + 6.0 * x1 * x2 + 3.0 * f64::powi(x2, 2)))
    * (30.0
      + f64::powi(2.0 * x1 - 3.0 * x2, 2)
        * (18.0 - 32.0 * x1 + 12.0 * f64::powi(x1, 2) + 48.0 * x2 - 36.0 * x1 * x2 + 27.0 * f64::powi(x2, 2)))
}

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

/// # Bukin function N.6
/// 2-dimensional only \
/// Global minimum: \
/// f(-10, 1) = 0
pub fn bukin_n6(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Bukin function N.6 takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  100.0 * f64::sqrt(f64::abs(x2 - 0.01 * f64::powi(x1, 2))) + 0.01 * f64::abs(x1 + 10.0)
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

/// # Levy function N.13
/// 2-dimensional only \
/// Global minimum: \
/// f(1, 1) = 0
pub fn levy_n13(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Levy function N.13 takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::powi(f64::sin(3.0 * f64::consts::PI * x1), 2)
    + f64::powi(x1 - 1.0, 2) * (1.0 + f64::powi(f64::sin(3.0 * f64::consts::PI * x2), 2))
    + f64::powi(x2 - 1.0, 2) * (1.0 + f64::powi(f64::sin(2.0 * f64::consts::PI * x2), 2))
}

/// # Himmelblau's function
/// 2-dimensional only \
/// Global minima: \
/// f(3, 2) = 0 \
/// f(-2.805118, 3.131312) = 0 \
/// f(-3.779310, -3.283186) = 0 \
/// f(3.584428, -1.848126) = 0
pub fn himmelblau(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Himmelblau's function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::powi(f64::powi(x1, 2) + x2 - 11.0, 2) + f64::powi(x1 + f64::powi(x2, 2) - 7.0, 2)
}

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

/// # Cross-in-tray function
/// 2-dimensional only \
/// Global minima: \
/// f(1.3491, -1.3491) = -2.06261 \
/// f(1.3491, 1.3491) = -2.06261 \
/// f(-1.3491, 1.3491) = -2.06261 \
/// f(-1.3491, -1.3491) = -2.06261
pub fn cross_in_tray(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Cross-in-tray function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  -0.0001
    * f64::powf(
      f64::abs(
        f64::sin(x1)
          * f64::sin(x2)
          * f64::exp(f64::abs(
            100.0 - f64::sqrt(f64::powi(x1, 2) + f64::powi(x2, 2)) / f64::consts::PI,
          )),
      ) + 1.0,
      0.1,
    )
}

/// ## Eggholder function
/// 2-dimensional only \
/// Global minimum: \
/// f(512, 404.2319) = -959.6407
pub fn eggholder(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Eggholder function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  -1.0 * (x2 + 47.0) * f64::sin(f64::sqrt(f64::abs((x1 / 2.0) + x2 + 47.0)))
    + -1.0 * x1 * f64::sin(f64::sqrt(f64::abs(x1 - x2 - 47.0)))
}

/// # Holder table function
/// 2-dimensional only \
/// Global minima: \
/// f(8.05502, 9.66459) = -19.2085 \
/// f(8.05502, -9.66459) = -19.2085 \
/// f(-8.05502, 9.66459) = -19.2085 \
/// f(-8.05502, -9.66459) = -19.2085
pub fn holder_table(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Holder table function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  -1.0
    * f64::abs(
      f64::sin(x1)
        * f64::cos(x2)
        * f64::exp(f64::abs(
          1.0 - f64::sqrt(f64::powi(x1, 2) + f64::powi(x2, 2)) / f64::consts::PI,
        )),
    )
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

/// # Schaffer N.2 function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, 0) = 0
pub fn schaffer_n2(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Schaffer function N.2 takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  0.5
    + (f64::powi(f64::sin(f64::powi(x1, 2) - f64::powi(x2, 2)), 2) - 0.5)
      / f64::powi(1.0 + 0.001 * (f64::powi(x1, 2) + f64::powi(x2, 2)), 2)
}

/// # Schaffer N.4 function
/// 2-dimensional only \
/// Global minima: \
/// f(0, 1.25313) = 0.292579 \
/// f(0, -1.25313) = 0.292579 \
/// f(1.25313, 0) = 0.292579 \
/// f(-1.25313, 0) = 0.292579
pub fn schaffer_n4(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Schaffer function N.4 takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  0.5
    + (f64::powi(
      f64::cos(f64::sin(f64::abs(f64::powi(x1, 2) - f64::powi(x2, 2)))),
      2,
    ) - 0.5)
      / f64::powi(1.0 + 0.001 * (f64::powi(x1, 2) + f64::powi(x2, 2)), 2)
}
