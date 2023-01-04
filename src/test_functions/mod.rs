use std::{f64, i32};
//use std::any::{Any, TypeId};

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

/// # Wordmax
/// Global maximum in `chromosome.len()`
#[allow(clippy::ptr_arg)]
pub fn wordmax(chromosome: &Vec<bool>) -> f64 {
  chromosome.iter().filter(|gene| **gene).count() as f64
}

/// # Bohhachevsky N.1 function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, 1.25313) = 0.292579 \
/// f(0,0) = 0

pub fn bohachevsky_n1(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Bohhachevsky function N.1 takes only a two dimensional vector as a parameter."
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
/// f(0,0) = 0

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
/// f(0, 1.25313) = 0.292579 \
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

/// # Shubert function
/// 2-dimensional only \
/// Global minima: 18\
/// f(x*) = -186.7309

pub fn shubert(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Shubert function N.3 takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  let mut y1 = 0_f64;
  let mut y2 = 0_f64;
  for i in 1..6 {
    y1 += i as f64 * f64::cos((i + 1) as f64 * x1 + i as f64);
    y2 += i as f64 * f64::cos((i + 1) as f64 * x2 + i as f64);
  }
  y1 * y2
}

/// # Langermann function
/// D-dimensional function \
/// Given d,m: \
/// c is a 1-dimensional vector of length m\
/// a is a d-dimensional vector of length m
pub fn langermann(x: &[f64], m: i32, c: &[f64], a: &[Vec<f64>]) -> f64 {
  let mut res = 0_f64;
  for i in 1..m + 1 {
    let mut inner = 0_f64;
    for (j, _) in x.iter().enumerate() {
      inner += f64::powi(x[j] - a[i as usize][j], 2)
    }
    res += c[i as usize] * f64::exp(-1_f64 * f64::consts::PI / inner) * f64::cos(f64::consts::PI * inner)
  }
  res
}

/// # Gramacy & Lee (2012) function
/// 1-dimensional only \

pub fn gramacy_lee(x: &f64) -> f64 {
  // assert_eq!(
  //   x.type_id(),
  //   TypeId::of::<f64>,
  //   "Gramacy & Lee (2012) function N.3 takes only a single floating-point argument."
  // );
  f64::sin(10_f64 * f64::consts::PI * x) / 2_f64 * x + f64::powi(x - 1_f64, 4)
}

/// # Drop-wave function
/// 2-dimensional only \
/// Global minimum:\
/// f(0,0) = -1

pub fn drop_wave(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Drop-wave function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  -1_f64 * (1_f64 + f64::cos(12_f64 * f64::sqrt(f64::powi(x1, 2) + f64::powi(x2, 2))))
    / (0.5 * (f64::powi(x1, 2) + f64::powi(x2, 2)) + 2_f64)
}

/// # Shekel function
/// 4-dimensional only \
/// Global minimum:\
/// at m=5: f(4,4,4,4) = -10.1532 \
/// at m=7: f(4,4,4,4) = -10.4029 \
/// at m=10: f(4,4,4,4) = -10.5364

pub fn shekel(x: &Vec<f64>, m: i32, beta: &Vec<f64>, c: &Vec<Vec<f64>>) -> f64 {
  assert_eq!(
    x.len(),
    4,
    "Shekel function takes only a four dimensional vector as a parameter."
  );
  assert_eq!(c[0].len(), 4, "C is a 4-by-m-dimensional matrix");
  assert_eq!(c.len(), m as usize, "C is a 4-by-m-dimensional matrix");
  assert_eq!(beta.len(), m as usize, "Beta is an -m-dimensional vector");
  let mut res = 0_f64;
  for i in 1..m + 1 {
    let mut sum = 0_f64;
    for j in 1..5 {
      sum += f64::powi(x[j] - c[j][j], 2)
    }
    res += f64::powi(sum + beta[i as usize], -1)
  }
  res
}

/// # Shekel function with recommended m, beta and c values
/// 4-dimensional only \
/// Assuming m = 10
/// Global minimum:\
/// f(4,4,4,4) = -10.5364

pub fn shekel_default(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    4,
    "Shekel function takes only a four dimensional vector as a parameter."
  );
  let mut res = 0.0;
  let raw = vec![1.0, 2.0, 2.0, 4.0, 4.0, 6.0, 3.0, 7.0, 5.0, 5.0];
  let beta = raw.into_iter().map(|x| 0.1 * x).collect::<Vec<_>>();
  let c = vec![
    vec![4.0, 1.0, 8.0, 6.0, 3.0, 2.0, 5.0, 8.0, 6.0, 7.0],
    vec![4.0, 1.0, 8.0, 6.0, 7.0, 9.0, 3.0, 1.0, 2.0, 3.6],
    vec![4.0, 1.0, 8.0, 6.0, 3.0, 2.0, 5.0, 8.0, 6.0, 7.0],
    vec![4.0, 1.0, 8.0, 6.0, 7.0, 9.0, 3.0, 1.0, 2.0, 3.6],
  ];
  for i in 1..11 {
    let mut sum = 0_f64;
    for j in 1..5 {
      sum += f64::powi(x[j] - c[j][j], 2)
    }
    res += f64::powi(sum + beta[i as usize], -1)
  }
  res
}

/// # Powell function
/// D-dimensional, D>4 \
/// Global minimum: \
/// f(0,..,0) = 0

pub fn powell(x: &Vec<f64>) -> f64 {
  assert!(
    x.len() > 3,
    "Powell function takes at least a four dimensional vector as a parameter."
  );
  let d = f64::floor((x.len() / 4) as f64) as i32;
  let mut res = 0_f64;
  for i in 1_usize..(d + 1) as usize {
    res += f64::powi(x[4 * i - 3] + 10_f64 * x[4 * i - 2], 2)
      + 5_f64 * f64::powi(x[4 * i - 1] - x[4 * i], 2)
      + f64::powi(x[4 * i - 2] - 2_f64 * x[4 * i - 1], 4)
      + 10_f64 * f64::powi(x[4 * i - 3] - x[4 * i], 4);
  }
  res
}

/// # Perm d, beta function
/// D-dimensional \
/// Global minimum:\
/// f(1,2,...,D) = 0

pub fn perm_d_beta(x: &Vec<f64>, beta: &f64) -> f64 {
  let d = x.len();
  let mut res = 0_f64;
  for i in 1..d + 1 {
    let mut sum = 0_f64;
    for (j, _) in x.iter().enumerate() {
      sum += (f64::powi(j as f64, i as i32) + beta) * (f64::powi(x[j] / j as f64, i as i32) - 1_f64);
    }
    res += f64::powi(sum, 2);
  }
  res
}

/// # Colville function
/// 4-dimensional \
/// Global minimum: \
/// f(1,1,1,1) = 0
pub fn colville(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    4,
    "Colville function takes only a four dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  let x3 = x[2];
  let x4 = x[3];
  100_f64 * f64::powi(f64::powi(x1, 2) - x2, 2)
    + f64::powi(x1 - 1_f64, 2)
    + f64::powi(x3 - 1_f64, 2)
    + 90_f64 * f64::powi(f64::powi(x3, 2) - x4, 2)
    + 10.1 * (f64::powi(x2 - 1_f64, 2) + f64::powi(x4 - 1_f64, 2))
    + 19.8 * (x2 - 1_f64) * (x4 - 1_f64)
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

/// # Branin function
/// 2-dimensional only

pub fn branin(x: &Vec<f64>, a: &f64, b: &f64, c: &f64, r: &f64, s: &f64, t: &f64) -> f64 {
  assert_eq!(
    2,
    x.len(),
    "Branin function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  a * f64::powi(x2 - b * f64::powi(x1, 2) + c * x1 - r, 2) + s * (1_f64 - t) * f64::cos(x1) + s
}

///# Branin function with default parameters
/// 2-dimensional only
/// Global minimum: \
/// f(-pi, 2.275) = 0.397887
pub fn branin_default(x: &Vec<f64>) -> f64 {
  assert_eq!(
    2,
    x.len(),
    "Branin function takes only a two dimensional vector as a parameter."
  );
  let a = 1_f64;
  let b = 5.1 / 4_f64 * f64::powi(f64::consts::PI, 2);
  let c = 5_f64 / f64::consts::PI;
  let r = 6_f64;
  let s = 10_f64;
  let t = 1_f64 / 8_f64 * f64::consts::PI;
  let x1 = x[0];
  let x2 = x[1];
  a * f64::powi(x2 - b * f64::powi(x1, 2) + c * x1 - r, 2) + s * (1_f64 - t) * f64::cos(x1) + s
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

///# Forrester et al. (2008) function
/// 1-dimensional only\
/// Global minimum: \
/// f(0.75) = -5.2911
pub fn forrester_et_al(x: &f64) -> f64 {
  // assert_eq!(
  //   x.type_id(),
  //   TypeId::of::<f64>,
  //   "Forrester et al (2012) function N.3 takes only a single floating-point argument."
  // );
  f64::powi(6_f64 * x - 2_f64, 2) * f64::sin(12_f64 * x - 4_f64)
}

///# De Jong n. 5 function
/// 2-dimensional only\

pub fn de_jong_n5(x: &Vec<f64>) -> f64 {
  assert_eq!(
    2,
    x.len(),
    "De Jong n.5 function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
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
    sum += 1_f64 / i as f64 + f64::powi(x1 - a[1][i], 6) + f64::powi(x2 - a[2][i], 6);
  }
  f64::powf(0.002 + sum, -1_f64)
}

///# Hartmann 3-dimensional function
/// 3-dimensional only\
/// Global minimum:\
/// f(0.114614, 0.555649, 0.852547)

pub fn hartmann_3d(x: &Vec<f64>) -> f64 {
  assert_eq!(
    3,
    x.len(),
    "Hartmann 3-dimensional function takes only a three dimensional vector as a parameter."
  );
  let alfa = vec![1.0, 1.2, 3.0, 3.2];
  let a = vec![
    vec![3.0, 10.0, 30.0],
    vec![0.1, 10.0, 35.0],
    vec![3.0, 10.0, 30.0],
    vec![0.1, 10.0, 35.0],
  ];
  let p = vec![
    vec![3.689, 1.17, 2.673],
    vec![4.699, 4.387, 7.47],
    vec![1.091, 8.732, 5.547],
    vec![0.381, 5.743, 8.828],
  ];
  let mut res = 0_f64;
  for i in 1..5 {
    let mut sum = 0_f64;
    for (j, _) in x.iter().enumerate() {
      sum += a[i][j] * f64::powi(x[j] - p[i][j], 2);
    }
    res -= alfa[i] * f64::exp(-1_f64 * sum);
  }
  res
}

///# Hartmann 4-dimensional function
/// 4-dimensional only\

pub fn hartmann_4d(x: &Vec<f64>) -> f64 {
  assert_eq!(
    4,
    x.len(),
    "Hartmann 4-dimensional function takes only a three dimensional vector as a parameter."
  );
  let alfa = vec![1.0, 1.2, 3.0, 3.2];
  let a = vec![
    vec![10.0, 3.0, 17.0, 3.5, 1.7, 8.0],
    vec![0.05, 10.0, 17.0, 0.1, 8.0, 14.0],
    vec![3.0, 3.5, 1.7, 10.0, 17.0, 8.0],
    vec![17.0, 8.0, 0.05, 10.0, 0.1, 14.0],
  ];
  let p = vec![
    vec![1.312, 1.696, 5.569, 1.24, 8.283, 5.886],
    vec![2.329, 4.135, 8.307, 3.736, 1.004, 9.991],
    vec![2.348, 1.451, 3.522, 2.883, 3.047, 6.650],
    vec![4.047, 8.828, 8.732, 5.743, 1.091, 3.81],
  ];
  let mut sum = 0_f64;
  for i in 1..5 {
    let mut inner_sum = 0_f64;
    for (j, _) in x.iter().enumerate() {
      inner_sum += a[i][j] * f64::powi(x[j] - p[i][j], 2);
    }
    sum += alfa[i] * f64::exp(-inner_sum);
  }
  (1_f64 / 0.839) * (1.1 - sum)
}

///# Hartmann 4-dimensional function
/// 6-dimensional only\
/// Global minimum:\
/// f(0.20169, 0.150011, 0.476874, 0.275332, 0.311652, 0.6573) = -3.32237

pub fn hartmann_6d(x: &Vec<f64>) -> f64 {
  assert_eq!(
    6,
    x.len(),
    "Hartmann 6-dimensional function takes only a six dimensional vector as a parameter."
  );
  let alfa = vec![1.0, 1.2, 3.0, 3.2];
  let a = vec![
    vec![10.0, 3.0, 17.0, 3.5, 1.7, 8.0],
    vec![0.05, 10.0, 17.0, 0.1, 8.0, 14.0],
    vec![3.0, 3.5, 1.7, 10.0, 17.0, 8.0],
    vec![17.0, 8.0, 0.05, 10.0, 0.1, 14.0],
  ];
  let p = vec![
    vec![1.312, 1.696, 5.569, 1.24, 8.283, 5.886],
    vec![2.329, 4.135, 8.307, 3.736, 1.004, 9.991],
    vec![2.348, 1.451, 0.522, 2.883, 3.047, 6.650],
    vec![4.047, 8.828, 8.732, 5.743, 1.091, 3.81],
  ];
  let mut sum = 0_f64;
  for i in 1..5 {
    let mut inner_sum = 0_f64;
    for (j, _) in x.iter().enumerate() {
      inner_sum += a[i][j] * f64::powi(x[j] - p[i][j], 2);
    }
    sum += alfa[i] * f64::exp(-1_f64 * inner_sum);
  }
  -1_f64 * sum
}

///# Freudenstein and Roth function
/// 2-dimensional only\
/// Global minimum:\
/// f(5,4) = 0

pub fn freudenstein_and_roth(x: &Vec<f64>) -> f64 {
  assert_eq!(
    2,
    x.len(),
    "Freudenstein and Roth function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::powi(-13_f64 + x1 + ((5_f64 - x2) * x2 - 2_f64) * x2, 2)
    + f64::powi(-29_f64 + x1 + ((1_f64 + x2) * x2 + 14_f64) * x2, 2)
}

///# Ackley, 2nd function
/// 2-dimensional only\
/// Global minimum:\
/// f(0,0) = -200

pub fn ackley2(x: &Vec<f64>) -> f64 {
  assert_eq!(
    2,
    x.len(),
    "Ackley 2nd function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::powf(-200_f64 * f64::consts::E, -0.02) * f64::sqrt(f64::powi(x1, 2) + f64::powi(x2, 2))
}

///# Ackley, 3rd function
/// 2-dimensional only\
/// Global minimum:\
/// f(0, ~ -0.4) = ~ -219.1418

pub fn ackley3(x: &Vec<f64>) -> f64 {
  assert_eq!(
    2,
    x.len(),
    "Ackley 3rd function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  200_f64
    * f64::powf(
      f64::consts::E,
      -0.02 * f64::sqrt(f64::powi(x1, 2) + f64::powi(x2, 2)),
    )
    + 5_f64 * f64::powf(f64::consts::E, f64::cos(3_f64 * x1) + f64::sin(3_f64 * x2))
}

///# Adjiman function
/// 2-dimensional only\
/// Global minimum:\
/// f(2, 0.10578) = ~ −2.02181

pub fn adijman(x: &Vec<f64>) -> f64 {
  assert_eq!(
    2,
    x.len(),
    "Adjiman function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::cos(x1) * f64::sin(x2) - (x1 / (f64::powi(x2, 2) + 1_f64))
}

///# Alpine function
/// Multidimensional
/// Global minimum:\
/// f(0, 0, ..., 0) = 0

pub fn alpine(x: &Vec<f64>) -> f64 {
  let mut res = 0_f64;
  for arg in x {
    res += f64::abs(arg * f64::sin(*arg) + arg / 10_f64)
  }
  res
}

///# Alpine function no 2
/// Multidimensional
/// Global minimum:\
/// f(7.917, ..., 7.917) = 2.808 ^ D(imensions)

pub fn alpine2(x: &Vec<f64>) -> f64 {
  let mut res = 0_f64;
  for arg in x {
    res *= f64::sqrt(*arg) * f64::sin(*arg)
  }
  res
}

///# Brad function
/// 3-dimensional only\
/// Global minimum:\
/// f(0.0824, 1.133, 2.3437) = 0.00821487

pub fn brad(x: &Vec<f64>) -> f64 {
  assert_eq!(
    3,
    x.len(),
    "Brad function takes only a three dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  let x3 = x[2];
  let y = vec![
    0.14, 0.18, 0.22, 0.25, 0.29, 0.32, 0.35, 0.39, 0.37, 0.58, 0.73, 0.96, 1.34, 2.10, 4.39,
  ];
  let mut res = 0_f64;
  for i in 0..15 {
    res += f64::powi(
      (y[0] - x1 - i as f64) / ((16 - i) as f64 * x2 + f64::min(i as f64, (16 - i) as f64) * x3),
      2,
    )
  }
  res
}

///# Bartels Conn function
/// 2-dimensional only\
/// Global minimum:\
/// f(0, 0) = 1

pub fn bartels_conn(x: &Vec<f64>) -> f64 {
  assert_eq!(
    3,
    x.len(),
    "Brad function takes only a three dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::abs(f64::powi(x1, 2) + f64::powi(x2, 2) + x1 * x2) + f64::abs(f64::sin(x1)) + f64::abs(f64::cos(x2))
}

///# Biggs EXP2 function
/// 2-dimensional only\
/// Global minimum:\
/// f(1, 10) = 0

pub fn biggs_exp2(x: &Vec<f64>) -> f64 {
  assert_eq!(
    2,
    x.len(),
    "Biggs EXP2 function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  let mut res = 0_f64;
  for i in 1..11 {
    res += f64::powi(
      f64::powf(f64::consts::E, -0.1 * i as f64 * x1)
        - 5_f64 * f64::powf(f64::consts::E, -0.1 * i as f64 * x2)
        - f64::powf(f64::consts::E, -0.1 * i as f64)
        - 5_f64 * f64::powf(f64::consts::E, i as f64),
      2,
    )
  }
  res
}

///# Biggs EXP3 function
/// 3-dimensional only\
/// Global minimum:\
/// f(1, 10, 5) = 0

pub fn biggs_exp3(x: &Vec<f64>) -> f64 {
  assert_eq!(
    3,
    x.len(),
    "Biggs EXP3 function takes only a three dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  let x3 = x[2];

  let mut res = 0_f64;
  for i in 1..11 {
    res += f64::powi(
      f64::powf(f64::consts::E, -0.1 * i as f64 * x1)
        - x3 * f64::powf(f64::consts::E, -0.1 * i as f64 * x2)
        - f64::powf(f64::consts::E, -0.1 * i as f64)
        - 5_f64 * f64::powf(f64::consts::E, i as f64),
      2,
    )
  }
  res
}

///# Biggs EXP4 function
/// 4-dimensional only\
/// Global minimum:\
/// f(1, 10, 1, 5) = 0

pub fn biggs_exp4(x: &Vec<f64>) -> f64 {
  assert_eq!(
    4,
    x.len(),
    "Biggs EXP3 function takes only a four dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  let x3 = x[2];
  let x4 = x[3];

  let mut res = 0_f64;
  for i in 1..11 {
    res += f64::powi(
      x3 * f64::powf(f64::consts::E, -0.1 * i as f64 * x1)
        - x4 * f64::powf(f64::consts::E, -0.1 * i as f64 * x2)
        - f64::powf(f64::consts::E, -0.1 * i as f64)
        - 5_f64 * f64::powf(f64::consts::E, i as f64),
      2,
    )
  }
  res
}

///# Biggs EXP5 function
/// 4-dimensional only\
/// Global minimum:\
/// f(1, 10, 1, 5, 4) = 0

pub fn biggs_exp5(x: &Vec<f64>) -> f64 {
  assert_eq!(
    5,
    x.len(),
    "Biggs EXP3 function takes only a five dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  let x3 = x[2];
  let x4 = x[3];
  let x5 = x[4];

  let mut res = 0_f64;
  for i in 1..11 {
    res += f64::powi(
      x3 * f64::powf(f64::consts::E, -0.1 * i as f64 * x1)
        - x4 * f64::powf(f64::consts::E, -0.1 * i as f64 * x2)
        - f64::powf(f64::consts::E, -0.1 * i as f64)
        - 5_f64
          * f64::powf(
            f64::consts::E,
            i as f64 + 3_f64 * f64::powf(f64::consts::E, -0.1 * x5),
          )
        + 3_f64 * f64::powf(f64::consts::E, -0.4 * i as f64),
      2,
    )
  }
  res
}

///# Biggs EXP5 function
/// 4-dimensional only\
/// Global minimum:\
/// f(1, 10, 1, 5, 4, 3) = 0

pub fn biggs_exp6(x: &Vec<f64>) -> f64 {
  assert_eq!(
    6,
    x.len(),
    "Biggs EXP3 function takes only a six dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  let x3 = x[2];
  let x4 = x[3];
  let x5 = x[4];
  let x6 = x[5];

  let mut res = 0_f64;
  for i in 1..12 {
    res += f64::powi(
      x3 * f64::powf(f64::consts::E, -0.1 * i as f64 * x1)
        - x4 * f64::powf(f64::consts::E, -0.1 * i as f64 * x2)
        - f64::powf(f64::consts::E, -0.1 * i as f64)
        - 5_f64
          * f64::powf(
            f64::consts::E,
            i as f64 + x6 * f64::powf(f64::consts::E, -0.1 * x5),
          )
        + 3_f64 * f64::powf(f64::consts::E, -0.4 * i as f64),
      2,
    )
  }
  res
}

///# Bird function
/// Two-dimensional
/// Global minimum:\
/// f(4.70104, 3.15294) = f(-1.58214, −3.13024) = −106.764537

pub fn bird(x: &Vec<f64>) -> f64 {
  assert_eq!(
    2,
    x.len(),
    "Bird function takes only a two-dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::sin(x1) * f64::powf(f64::consts::E, f64::powi(1_f64 - f64::cos(x2), 2))
    + f64::cos(x2) * f64::powf(f64::consts::E, f64::powi(1_f64 - f64::sin(x1), 2))
    + f64::powi(x1 - x2, 2)
}

///# Box-Betts Quadratic Sum function
/// Three-dimensional
/// Global minimum:\
/// f(1, 10, 1) = 0

pub fn box_betts(x: &Vec<f64>) -> f64 {
  assert_eq!(
    3,
    x.len(),
    "Box-Betts function takes only a three-dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  let x3 = x[2];
  let mut res = 0_f64;
  for i in 0..3 {
    res += f64::powi(
      f64::powf(f64::consts::E, -0.1 * (i + 1) as f64 * x1)
        + f64::powf(f64::consts::E, -0.1 * (i + 1) as f64 * x2)
        + f64::powf(
          f64::consts::E,
          x3 * (-0.1 * i as f64) - f64::powf(f64::consts::E, -1_f64 * (i + 1) as f64),
        ),
      2,
    )
  }
  res
}

///# Branin RCOS function
/// Two-dimensional
/// Global minimum:\
/// f(−3.2, 12.53) = 5.559037

pub fn branin_rcos(x: &Vec<f64>) -> f64 {
  assert_eq!(
    2,
    x.len(),
    "Branin RCOS function takes only a two-dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::powi(
    x2 - 5.1 * f64::powi(x1, 2) / 4_f64 * f64::powi(f64::consts::PI, 2) + 5_f64 * x1 / f64::consts::PI
      - 6_f64,
    2,
  ) + 10_f64 * (1_f64 - 1_f64 / 8_f64 * f64::consts::PI) * f64::cos(x1)
    + 10_f64
}

///# Branin RCOS function no2
/// Two-dimensional
/// Global minimum:\
/// f(−pi, 12.275) = f(pi, 2.275) = f(3 pi, 2.425) = 0.3978873

pub fn branin_rcos2(x: &Vec<f64>) -> f64 {
  assert_eq!(
    2,
    x.len(),
    "Branin RCOS function no2 takes only a two-dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::powi(
    x2 - 5.1 * f64::powi(x1, 2) / 4_f64 * f64::powi(f64::consts::PI, 2) + 5_f64 * x1 / f64::consts::PI
      - 6_f64,
    2,
  ) + 10_f64
    * (1_f64 - 1_f64 / 8_f64 * f64::consts::PI)
    * f64::cos(x1)
    * f64::cos(x2)
    * f64::ln(f64::powi(x1, 2) + f64::powi(x2, 2) + 1_f64)
    + 10_f64
}

///# Brent function
/// Two-dimensional
/// Global minimum:\
/// f(0, 0) = 0

pub fn brent(x: &Vec<f64>) -> f64 {
  assert_eq!(
    2,
    x.len(),
    "Brent function takes only a two-dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  f64::powi(x1 + 10_f64, 2)
    + f64::powi(x2 + 10_f64, 2)
    + f64::powf(f64::consts::E, -1_f64 * f64::powi(x1, 2) - f64::powi(x2, 2))
}

///# Brown function
/// Multidimensional
/// Global minimum:\
/// f(0,..., 0) = 0

pub fn brown(x: &Vec<f64>) -> f64 {
  let mut res = 0_f64;
  for i in 0..x.len() - 1 {
    res += f64::powf(f64::powi(x[i], 2), f64::powi(x[i + 1], 2) + 1_f64)
      + f64::powf(f64::powi(x[i + 1], 2), f64::powi(x[i], 2) + 1_f64)
  }
  res
}

/// # Bukin function N.2
/// 2-dimensional only \
/// Global minimum: \
/// f(-10, 0) = 0
///
pub fn bukin_2(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Bukin function N.2 takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  100_f64 * (x2 - 0.01 * f64::powi(x1, 2) + 1_f64) + 0.01 * f64::powi(x1 + 10_f64, 2)
}

/// # Bukin function N.4
/// 2-dimensional only \
/// Global minimum: \
/// f(-10, 0) = 0
///
pub fn bukin_4(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Bukin function N.4 takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  100_f64 * f64::powi(x2, 2) + 0.01 * f64::abs(x1 + 10_f64)
}

/// # Chen Bird function
/// 2-dimensional only \
/// Global minimum: \
/// f(-7/18, -13/18) = -2000
///
pub fn chen_bird(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Chen Bird function takes only a two dimensional vector as a parameter."
  );
  let x1 = x[0];
  let x2 = x[1];
  -0.001 / (0.001 * 0.001 + f64::powi(x1 - 0.4 * x2 + 0.1, 2))
    - 0.001 / (0.001 * 0.001 + f64::powi(2_f64 * x1 + x2 - 1.5, 2))
}
