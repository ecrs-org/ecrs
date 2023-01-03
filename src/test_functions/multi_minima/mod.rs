use std::f64;

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

/// # Shubert function
/// 2-dimensional only \
/// Global minima: 18\
/// f(x*) = -186.7309

pub fn shubert(x: &Vec<f64>) -> f64 {
  assert_eq!(
    x.len(),
    2,
    "Shubert function takes only a two dimensional vector as a parameter."
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
