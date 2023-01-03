use std::f64;

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

///# Hartmann 6-dimensional function
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

/// # Wordmax
/// Global maximum in `chromosome.len()`
#[allow(clippy::ptr_arg)]
pub fn wordmax(chromosome: &Vec<bool>) -> f64 {
  chromosome.iter().filter(|gene| **gene).count() as f64
}
