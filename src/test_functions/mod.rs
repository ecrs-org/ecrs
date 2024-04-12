#![cfg(feature = "test_functions")]
//! Implementations of a variety of test functions

use std::{f64, i32};
// use rand::{Rng, thread_rng};
//use std::any::{Any, TypeId};

/// # Ackley function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn ackley(x: &[f64]) -> f64 {
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

///# Ackley, 2nd function
/// 2-dimensional only\
/// Global minimum:\
/// f(0,0) = -200

pub fn ackley2(x: &[f64]) -> f64 {
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

pub fn ackley3(x: &[f64]) -> f64 {
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

//TODO: Implement modified/Ackley no. 4

///# Adjiman function
/// 2-dimensional only\
/// Global minimum:\
/// f(2, 0.10578) = ~ −2.02181

pub fn adijman(x: &[f64]) -> f64 {
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

pub fn alpine(x: &[f64]) -> f64 {
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

pub fn alpine2(x: &[f64]) -> f64 {
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

pub fn brad(x: &[f64]) -> f64 {
    assert_eq!(
        3,
        x.len(),
        "Brad function takes only a three dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];
    let y = [
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

pub fn bartels_conn(x: &[f64]) -> f64 {
    assert_eq!(
        3,
        x.len(),
        "Brad function takes only a three dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::abs(f64::powi(x1, 2) + f64::powi(x2, 2) + x1 * x2) + f64::abs(f64::sin(x1)) + f64::abs(f64::cos(x2))
}

/// #  Beale function
/// 2-dimensional only \
/// Global minimum: \
/// f(3, 0.5) = 0
pub fn beale(x: &[f64]) -> f64 {
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

///# Biggs EXP2 function
/// 2-dimensional only\
/// Global minimum:\
/// f(1, 10) = 0

pub fn biggs_exp2(x: &[f64]) -> f64 {
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

pub fn biggs_exp3(x: &[f64]) -> f64 {
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

pub fn biggs_exp4(x: &[f64]) -> f64 {
    assert_eq!(
        4,
        x.len(),
        "Biggs EXP4 function takes only a four dimensional vector as a parameter."
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
/// 5-dimensional only\
/// Global minimum:\
/// f(1, 10, 1, 5, 4) = 0

pub fn biggs_exp5(x: &[f64]) -> f64 {
    assert_eq!(
        5,
        x.len(),
        "Biggs EXP5 function takes only a five dimensional vector as a parameter."
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

///# Biggs EXP6 function
/// 6-dimensional only\
/// Global minimum:\
/// f(1, 10, 1, 5, 4, 3) = 0

pub fn biggs_exp6(x: &[f64]) -> f64 {
    assert_eq!(
        6,
        x.len(),
        "Biggs EXP6 function takes only a six dimensional vector as a parameter."
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

pub fn bird(x: &[f64]) -> f64 {
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
/// # Bohhachevsky N.1 function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, 1.25313) = 0.292579 \
/// f(0,0) = 0

pub fn bohachevsky_n1(x: &[f64]) -> f64 {
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

pub fn bohachevsky_n2(x: &[f64]) -> f64 {
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

pub fn bohachevsky_n3(x: &[f64]) -> f64 {
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

/// # Booth function
/// 2-dimensional only \
/// Global minimum: \
/// f(1, 3) = 0
pub fn booth(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Booth function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(x1 + 2.0 * x2 - 7.0, 2) + f64::powi(2.0 * x1 + x2 - 5.0, 2)
}

///# Box-Betts Quadratic Sum function
/// Three-dimensional
/// Global minimum:\
/// f(1, 10, 1) = 0

pub fn box_betts(x: &[f64]) -> f64 {
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

/// # Branin function
/// 2-dimensional only

pub fn branin(x: &[f64], a: &f64, b: &f64, c: &f64, r: &f64, s: &f64, t: &f64) -> f64 {
    assert_eq!(
        2,
        x.len(),
        "Branin function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    a * f64::powi(x2 - b * f64::powi(x1, 2) + c * x1 - r, 2) + s * (1_f64 - t) * f64::cos(x1) + s
}

///# Branin RCOS function
/// Two-dimensional
/// Global minimum:\
/// f(−3.2, 12.53) = 5.559037

pub fn branin_rcos(x: &[f64]) -> f64 {
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

pub fn branin_rcos2(x: &[f64]) -> f64 {
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

///# Branin function with default parameters
/// 2-dimensional only
/// Global minimum: \
/// f(-pi, 2.275) = 0.397887
pub fn branin_default(x: &[f64]) -> f64 {
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

///# Brent function
/// Two-dimensional
/// Global minimum:\
/// f(0, 0) = 0

pub fn brent(x: &[f64]) -> f64 {
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

pub fn brown(x: &[f64]) -> f64 {
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
pub fn bukin_2(x: &[f64]) -> f64 {
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
pub fn bukin_4(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Bukin function N.4 takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    100_f64 * f64::powi(x2, 2) + 0.01 * f64::abs(x1 + 10_f64)
}

/// # Bukin function N.6
/// 2-dimensional only \
/// Global minimum: \
/// f(-10, 1) = 0
pub fn bukin_n6(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Bukin function N.6 takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    100.0 * f64::sqrt(f64::abs(x2 - 0.01 * f64::powi(x1, 2))) + 0.01 * f64::abs(x1 + 10.0)
}

/// # Three-hump camel function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, 0) = 0
pub fn three_hump_camel(x: &[f64]) -> f64 {
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
pub fn six_hump_camel(x: &[f64]) -> f64 {
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

/// # Chen Bird function
/// 2-dimensional only \
/// Global minimum: \
/// f(-7/18, -13/18) = -2000
///
pub fn chen_bird(x: &[f64]) -> f64 {
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

/// # Chen V function
/// 2-dimensional only \
/// Global minimum: \
/// f(−0.3888889, 0.7222222) = -2000
///
pub fn chen_v(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Chen V function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    -0.001 / (0.001 * 0.001 + f64::powi(f64::powi(x1, 2) + f64::powi(x2, 2) - 1_f64, 2))
        - 0.001 / (0.001 * 0.001 + f64::powi(f64::powi(x1, 2) + f64::powi(x2, 2) - 0.5, 2))
        - 0.001 / (0.001 * 0.001 + f64::powi(f64::powi(x1, 2) - f64::powi(x2, 2), 2))
}

/// # Chichinadze function
/// 2-dimensional only \
/// Global minimum: \
/// f(5.90133, 0.5) = −43.3159
///
pub fn chichinadze(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Chichinadze function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(x1, 2) - 12_f64 * x1
        + 11_f64
        + 10_f64 * f64::cos(f64::consts::PI * x1 / 2_f64)
        + 8_f64 * f64::sin(5_f64 * f64::consts::PI * x1 / 2_f64)
        - f64::powf(0.2, 0.5) * f64::exp(-0.5 * f64::powi(x2 - 0.5, 2))
}

/// # Chung Reynolds function
/// Global minimum: \
/// f(0, 0) = 0
///
pub fn chung_reynolds(x: &[f64]) -> f64 {
    let mut res = 0_f64;
    for arg in x {
        res += f64::powi(*arg, 2);
    }
    f64::powi(res, 2)
}

//TODO: Implement Cola function

/// # Colville function
/// 4-dimensional \
/// Global minimum: \
/// f(1,1,1,1) = 0
pub fn colville(x: &[f64]) -> f64 {
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

//TODO: Implement Corana function

/// # Cosine Mixture function
/// Global minimum: \
/// f(0,..., 0) = 0
///
pub fn cosine_mixture(x: &[f64]) -> f64 {
    let mut sum1 = 0_f64;
    let mut sum2 = 0_f64;
    for arg in x {
        sum1 += f64::cos(5_f64 * f64::consts::PI * arg);
        sum2 += f64::powi(*arg, 2)
    }
    -0.1 * sum1 - sum2
}

/// # Cross-in-tray function
/// 2-dimensional only \
/// Global minima: \
/// f(1.3491, -1.3491) = -2.06261 \
/// f(1.3491, 1.3491) = -2.06261 \
/// f(-1.3491, 1.3491) = -2.06261 \
/// f(-1.3491, -1.3491) = -2.06261
pub fn cross_in_tray(x: &[f64]) -> f64 {
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

/// # Csendes function
/// Global minimum: \
/// f(0,..., 0) = 0
///
pub fn csendes(x: &[f64]) -> f64 {
    let mut res = 0_f64;
    for arg in x {
        res += f64::powi(*arg, 6) * (2_f64 + f64::sin(1_f64 / arg))
    }
    res
}

/// # Cube function
/// 2-dimensional only \
/// Global minimum: \
/// f(-1, 1) = 0
///
pub fn cube(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Cube function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    100_f64 * f64::powi(x2 - f64::powi(x1, 3), 2) + f64::powi(1_f64 - x1, 2)
}

/// # Damavandi function
/// 2-dimensional only \
/// Global minimum: \
/// f(-1, 1) = 0
///
pub fn damavandi(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Damavandi function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    (1_f64
        - f64::powi(
            f64::sin(f64::consts::PI * (x1 - 2_f64)) * f64::sin(f64::consts::PI * (x2 - 2_f64))
                / f64::powi(f64::consts::PI, 2)
                * (x1 - 2_f64)
                * (x2 - 2_f64),
            5,
        ))
        * (2_f64 + f64::powi(x1 - 7_f64, 2) + 2_f64 * f64::powi(x2 - 7_f64, 2))
}

/// # Deb 1 function
/// Global minimum: \
/// f(0,..., 0) = 0
///
pub fn deb1(x: &[f64]) -> f64 {
    let mut res = 0_f64;
    for arg in x {
        res += f64::powi(f64::sin(5_f64 * f64::consts::PI * arg), 6)
    }
    -1_f64 / (x.len() - 1) as f64 * res
}

/// # Deb 3 function
/// Global minimum: \
/// f(0,..., 0) = 0
///
pub fn deb3(x: &[f64]) -> f64 {
    let mut res = 0_f64;
    for arg in x {
        res += f64::powi(
            f64::sin(5_f64 * f64::consts::PI * (f64::powf(*arg, 0.75) - 0.05)),
            6,
        )
    }
    -1_f64 / (x.len() - 1) as f64 * res
}

/// # Deckkers-Aarts function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, +-15) = −24777
///
pub fn deckkers_aarts(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Deckkers-Aarts function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    100000_f64 * f64::powi(x1, 2) + f64::powi(x2, 2) - f64::powi(f64::powi(x1, 2) + f64::powi(x2, 2), 2)
        + (1 / 100000) as f64 * f64::powi(f64::powi(x1, 2) + f64::powi(x2, 2), 4)
}

/// # deVilliers Glasser 1 function
/// 4-dimensional only \
/// Global minimum: \
/// f(0, 0, 0, 0) = 0
///
pub fn devilliers_glasser1(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "deVilliers Glasser 1 function takes only a four dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];
    let x4 = x[3];
    let mut res = 0_f64;
    for i in 1..25 {
        res += f64::powi(
            x1 * f64::powf(x2, 0.1 * (i - 1) as f64) * f64::sin(x3 * 0.1 * (i - 1) as f64 + x4)
                - 60.137
                    * f64::powf(
                        1.371,
                        0.1 * (i - 1) as f64 * f64::sin(3.112 * 0.1 * (i - 1) as f64 + 1.761),
                    ),
            2,
        )
    }
    res
}

/// # deVilliers Glasser 2 function
/// 5-dimensional only \
/// Global minimum: \
/// f(0, 0, 0, 0, 0) = 0
///
pub fn devilliers_glasser2(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        5,
        "deVilliers Glasser 2 function takes only a five dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];
    let x4 = x[3];
    let x5 = x[4];
    let mut res = 0_f64;
    for i in 1..25 {
        res += f64::powi(
            x1 * f64::powf(x2, 0.1 * (i - 1) as f64)
                * f64::tanh(x3 * 0.1 * (i - 1) as f64 + f64::sin(x4 * 0.1 * (i - 1) as f64))
                * f64::cos(0.1 * (i - 1) as f64 * f64::powf(f64::consts::E, x5))
                - 53.81
                    * f64::powf(1.27, 0.1 * (i - 1) as f64)
                    * f64::tanh(3.012 * 0.1 * (i - 1) as f64 + f64::sin(2.13 * 0.1 * (i - 1) as f64))
                    * f64::cos(f64::powf(f64::consts::E, 0.507) * 0.1 * (i - 1) as f64),
            2,
        )
    }
    res
}

/// # Dixon-Price function
/// n-dimensional \
/// Global minimum: \
/// f((x_1, ..., x_n)) = 0, where x_i = 2^( -(2^i - 2) / 2^i)
pub fn dixon_price(x: &[f64]) -> f64 {
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

///# De Jong n. 5 function
/// 2-dimensional only\

pub fn de_jong_n5(x: &[f64]) -> f64 {
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
    let a = [a1, b];
    for i in 1..26 {
        sum += 1_f64 / i as f64 + f64::powi(x1 - a[0][i], 6) + f64::powi(x2 - a[1][i], 6);
    }
    f64::powf(0.002 + sum, -1_f64)
}

/// # Dolan function
/// 5-dimensional only \
/// Global minimum: \
/// f(0, 0, 0, 0, 0) = 0
///
pub fn dolan(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        5,
        "Dolan function takes only a five dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];
    let x4 = x[3];
    let x5 = x[4];
    (x1 + 1.7 * x2) * f64::sin(x1) - 1.5 * x3 - 0.1 * x4 * f64::cos(x4 + x5 - x1) + 0.2 * f64::powi(x5, 2)
        - x2
        - 1_f64
}

/// # Drop-wave function
/// 2-dimensional only \
/// Global minimum:\
/// f(0,0) = -1

pub fn drop_wave(x: &[f64]) -> f64 {
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

/// # Easom function
/// 2-dimensional only \
/// Global minimum: \
/// f(PI, PI) = -1, where PI = 3.14159...
pub fn easom(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Easom function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    -1.0 * f64::cos(x1)
        * f64::cos(x2)
        * f64::exp(-1.0 * (f64::powi(x1 - f64::consts::PI, 2) + f64::powi(x2 - f64::consts::PI, 2)))
}

/// # El-Attar-Vidyasagar-Dutta function
/// 2-dimensional only \
/// Global minimum: \
/// f(2.842503, 1.920175) = 0.470427
///
pub fn eavd(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "El-Attar-Vidyasagar-Dutta function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(f64::powi(x1, 2) + x2 - 10_f64, 2)
        + f64::powi(x1 + f64::powi(x2, 2) - 7_f64, 2)
        + f64::powi(f64::powi(x1, 2) + f64::powi(x2, 3) - 1_f64, 2)
}

/// # Egg crate function
/// 2-dimensional only \
/// Global minimum: \
/// f(2.842503, 1.920175) = 0.470427
///
pub fn egg_crate(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Egg crate function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(x1, 2) + f64::powi(x2, 2) + 25_f64 * (f64::powi(f64::sin(x1), 2) + f64::powi(f64::sin(x2), 2))
}

/// ## Eggholder function
/// 2-dimensional only \
/// Global minimum: \
/// f(512, 404.2319) = -959.6407
pub fn eggholder(x: &[f64]) -> f64 {
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

/// # Exponential function
/// Global minimum: \
/// f(0, ..., 0) = 1
///
pub fn exponential(x: &[f64]) -> f64 {
    let mut res = 0_f64;
    for arg in x {
        res += f64::powi(*arg, 2)
    }
    -1_f64 * f64::exp(-0.5 * res)
}

/// # Exp 2 function
/// 2-dimensional only \
/// Global minimum: \
/// f(1, 10) = 0
///
pub fn exp2(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Exp 2 function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    let mut res = 0_f64;
    for i in 1..10 {
        res += f64::powi(
            f64::powf(f64::consts::E, -1_f64 * i as f64 * x1 / 10_f64)
                - 5_f64 * f64::powf(f64::consts::E, -1_f64 * i as f64 * x2 / 10_f64)
                - f64::powi(f64::consts::E, -i / 10)
                + 5_f64 * f64::powi(f64::consts::E, -i),
            2,
        )
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

/// # Freudenstein Roth function
/// 2-dimensional only \
/// Global minimum: \
/// f(5,4) = 0
///
pub fn freudenstein_roth(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Freudenstein Roth function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(x1 - 13_f64 + ((5_f64 - x2) * x2 - 2_f64) * x2, 2)
        + f64::powi(x1 - 29_f64 + ((x2 + 1_f64) * x2 - 14_f64) * x2, 2)
}

/// # Giunta Roth function
/// 2-dimensional only \
/// Global minimum: \
/// f(0.45834282, 0.45834282) = 0.060447
///
pub fn giunta(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Giunta function takes only a two dimensional vector as a parameter."
    );
    let mut res = 0_f64;
    for arg in x {
        res += f64::sin((16 / 15) as f64 * arg - 1_f64)
            + f64::powi((16 / 15) as f64 * arg - 1_f64, 2)
            + 0.02 * f64::sin(4_f64 * ((16 / 15) as f64 * arg - 1_f64))
    }
    res
}

/// # Goldstein-Price function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, -1) = 3
pub fn goldstein_price(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Goldstein-Price function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    (1.0 + f64::powi(x1 + x2 + 1.0, 2)
        * (19.0 - 14.0 * x1 + 3.0 * f64::powi(x1, 2) - 14.0 * x2 + 6.0 * x1 * x2 + 3.0 * f64::powi(x2, 2)))
        * (30.0
            + f64::powi(2.0 * x1 - 3.0 * x2, 2)
                * (18.0 - 32.0 * x1 + 12.0 * f64::powi(x1, 2) + 48.0 * x2 - 36.0 * x1 * x2
                    + 27.0 * f64::powi(x2, 2)))
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
pub fn griewank(x: &[f64]) -> f64 {
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

/// # Gulf Research function
/// 99-dimensional only \
/// Global minimum: \
/// f(50, 25, 1.5, .....) = 0///
pub fn gulf_research(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        99,
        "Gulf Research function takes only a 99-dimensional vector as a parameter."
    );
    let x3 = x[2];
    let mut res = 0_f64;
    for (i, _) in x.iter().enumerate() {
        res += f64::powi(
            f64::exp(
                -1_f64
                    * f64::powf(
                        25_f64 + f64::powf(-50_f64 * f64::ln(0.01 * i as f64), 1_f64 / 1.5),
                        x3,
                    )
                    / x[i],
            ),
            2,
        )
    }
    res
}

//TODO: Implement Hansen function

///# Hartmann 3-dimensional function
/// 3-dimensional only\
/// Global minimum:\
/// f(0.114614, 0.555649, 0.852547)

pub fn hartmann_3d(x: &[f64]) -> f64 {
    assert_eq!(
        3,
        x.len(),
        "Hartmann 3-dimensional function takes only a three dimensional vector as a parameter."
    );
    let alfa = [1.0, 1.2, 3.0, 3.2];
    let a = [vec![3.0, 10.0, 30.0],
        vec![0.1, 10.0, 35.0],
        vec![3.0, 10.0, 30.0],
        vec![0.1, 10.0, 35.0]];
    let p = [vec![3.689, 1.17, 2.673],
        vec![4.699, 4.387, 7.47],
        vec![1.091, 8.732, 5.547],
        vec![0.381, 5.743, 8.828]];
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

pub fn hartmann_4d(x: &[f64]) -> f64 {
    assert_eq!(
        4,
        x.len(),
        "Hartmann 4-dimensional function takes only a three dimensional vector as a parameter."
    );
    let alfa = [1.0, 1.2, 3.0, 3.2];
    let a = [vec![10.0, 3.0, 17.0, 3.5, 1.7, 8.0],
        vec![0.05, 10.0, 17.0, 0.1, 8.0, 14.0],
        vec![3.0, 3.5, 1.7, 10.0, 17.0, 8.0],
        vec![17.0, 8.0, 0.05, 10.0, 0.1, 14.0]];
    let p = [vec![1.312, 1.696, 5.569, 1.24, 8.283, 5.886],
        vec![2.329, 4.135, 8.307, 3.736, 1.004, 9.991],
        vec![2.348, 1.451, 3.522, 2.883, 3.047, 6.650],
        vec![4.047, 8.828, 8.732, 5.743, 1.091, 3.81]];
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

pub fn hartmann_6d(x: &[f64]) -> f64 {
    assert_eq!(
        6,
        x.len(),
        "Hartmann 6-dimensional function takes only a six dimensional vector as a parameter."
    );
    let alfa = [1.0, 1.2, 3.0, 3.2];
    let a = [vec![10.0, 3.0, 17.0, 3.5, 1.7, 8.0],
        vec![0.05, 10.0, 17.0, 0.1, 8.0, 14.0],
        vec![3.0, 3.5, 1.7, 10.0, 17.0, 8.0],
        vec![17.0, 8.0, 0.05, 10.0, 0.1, 14.0]];
    let p = [vec![1.312, 1.696, 5.569, 1.24, 8.283, 5.886],
        vec![2.329, 4.135, 8.307, 3.736, 1.004, 9.991],
        vec![2.348, 1.451, 0.522, 2.883, 3.047, 6.650],
        vec![4.047, 8.828, 8.732, 5.743, 1.091, 3.81]];
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

//TODO: Implement Helical Valley function

/// # Himmelblau's function
/// 2-dimensional only \
/// Global minima: \
/// f(3, 2) = 0 \
/// f(-2.805118, 3.131312) = 0 \
/// f(-3.779310, -3.283186) = 0 \
/// f(3.584428, -1.848126) = 0
pub fn himmelblau(x: &[f64]) -> f64 {
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
pub fn holder_table(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Holder table function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    -1.0 * f64::abs(
        f64::sin(x1)
            * f64::cos(x2)
            * f64::exp(f64::abs(
                1.0 - f64::sqrt(f64::powi(x1, 2) + f64::powi(x2, 2)) / f64::consts::PI,
            )),
    )
}

/// # Holder Table 2 function
/// 2-dimensional only \
/// Global minima: \
/// f(+-9.646168, -+9.646168) = −26.920336
///
pub fn holder_table2(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Holder Table function 2 takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    -1.0 * f64::abs(
        f64::cos(x1)
            * f64::cos(x2)
            * f64::exp(f64::abs(
                1.0 - f64::sqrt(f64::powi(x1, 2) + f64::powi(x2, 2)) / f64::consts::PI,
            )),
    )
}

/// # Holder Table 3 function
/// 2-dimensional only \
/// Global minima: \
/// f(f(+-8.055023472141116, +-9.664590028909654) = −19.20850
///
pub fn holder_table3(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Holder Table function 3 takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    -1.0 * f64::abs(
        f64::cos(x1)
            * f64::cos(x2)
            * f64::exp(f64::abs(f64::powi(
                1.0 - f64::sqrt(f64::powi(x1, 2) + f64::powi(x2, 2)) / f64::consts::PI,
                2,
            )))
            / 30_f64,
    )
}

/// # Hosaki Roth function
/// 2-dimensional only \
/// Global minimum: \
/// f(4,2) = ~ −2.3458
///
pub fn hosaki(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Hosaki function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    (1_f64 - 8_f64 * x1 + 7_f64 * f64::powi(x1, 2) - 7_f64 / 3_f64 * f64::powi(x1, 3)
        + 0.25 * f64::powi(x1, 4))
        * f64::powi(x2, 2)
        * f64::powf(f64::consts::E, -1_f64 * x2)
}

/// # Jennrich-Sampson function
/// 2-dimensional only \
/// Global minimum: \
/// f(0.257825, 0.257825) = 124.3612
///
pub fn jennrich_sampson(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Jennrich-Sampson function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    let mut res = 0_f64;
    for i in 1..11 {
        res += f64::powi(
            2_f64 + 2_f64 * i as f64
                - (f64::powf(f64::consts::E, i as f64 * x1) + f64::powf(f64::consts::E, i as f64 * x2)),
            2,
        )
    }
    res
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

//TODO: Implement Langermann-5 function

/// # Levy function
/// n-dimensional \
/// Global minimum: \
/// f(1, ..., 1) = 0
pub fn levy(x: &[f64]) -> f64 {
    assert!(
        !x.is_empty(),
        "Levy function takes an at least one dimensional vector as a parameter."
    );
    let mut result: f64 = f64::powi(f64::sin(f64::consts::PI * (1.0 + (x[0] - 1.0) / 4.0)), 2);
    for x_i in x.iter().take(x.len() - 1) {
        let temp: f64 = 1.0 + (x_i - 1.0) / 4.0;
        result +=
            f64::powi(temp - 1.0, 2) * (1.0 + 10.0 * f64::powi(f64::sin(f64::consts::PI * temp + 1.0), 2));
    }
    let temp: f64 = 1.0 + (x[x.len() - 1] - 1.0) / 4.0;
    result += f64::powi(temp - 1.0, 2) * (1.0 + f64::powi(f64::sin(2.0 * f64::consts::PI * temp), 2));
    result
}

/// # Levy function N.13
/// 2-dimensional only \
/// Global minimum: \
/// f(1, 1) = 0
pub fn levy_n13(x: &[f64]) -> f64 {
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

/// # Keane function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, 1.39325) = f(1.39325, 0) = −0.673668
///
pub fn keane(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Keane function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(f64::sin(x1 - x2), 2) * f64::powi(f64::sin(x1 + x2), 2)
        / f64::sqrt(f64::powi(x1, 2) + f64::powi(x2, 2))
}

/// # Leon function
/// 2-dimensional only \
/// Global minimum: \
/// f(1, 1) = 0
///
pub fn leon(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Leon function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    100_f64 * f64::powi(x2 - f64::powi(x1, 2), 2) + f64::powi(1_f64 - x1, 2)
}

/// # Matyas function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, 0) = 0
pub fn matyas(x: &[f64]) -> f64 {
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
pub fn mcormick(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "McCormick table function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::sin(x1 + x2) + f64::powi(x1 - x2, 2) + -1.5 * x1 + 2.5 * x2 + 1.0
}

/// # Michalewicz function
/// n-dimensional \
/// Global minimum: \
/// depends of number of dimensions \
/// for two-dimensional argument: f(2.2, 1.57) = -1.8013
pub fn michalewicz(x: &[f64]) -> f64 {
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

/// # Miele Cantrell function
/// 4-dimensional only \
/// Global minimum: \
/// f(0, 1, 1, 1) = 0
///
pub fn miele_cantrell(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        4,
        "Miele Cantrell function takes only a four dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];
    let x4 = x[3];
    f64::powi(f64::powf(f64::consts::E, -1_f64 * x1) - x2, 4)
        + 100_f64 * f64::powi(x2 - x3, 6)
        + f64::powi(f64::tan(x3 - x4), 4)
        + f64::powi(x1, 8)
}

//TODO: Implement Mishra functions

/// # Parsopoulos function
/// 2-dimensional only \

pub fn parsopoulos(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Parsopoulos function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(f64::cos(x1), 2) + f64::powi(f64::sin(x2), 2)
}

/// # Pen Holder function
/// 2-dimensional only \
/// Global minimum: \
/// f(+-9.646168, -+9.646168) = −0.96354
///
pub fn pen_holder(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Pen Holder function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    -1_f64
        * f64::exp(f64::powi(
            f64::abs(
                f64::cos(x1)
                    * f64::cos(x2)
                    * f64::powf(
                        f64::consts::E,
                        f64::abs(
                            1_f64 - f64::powf(f64::powi(x1, 2) + f64::powi(x2, 2), 0.5 / f64::consts::PI),
                        ),
                    ),
            ),
            -1,
        ))
}

/// # Pathological function
/// Multidimensional
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn pathological(x: &[f64]) -> f64 {
    let mut res = 0_f64;
    for i in 1..x.len() - 2 {
        res += 0.5
            + (f64::powi(
                f64::sin(f64::sqrt(100_f64 * f64::powi(x[i], 2) + f64::powi(x[i + 1], 2))),
                2,
            ) - 0.5)
                / (1_f64
                    + 0.001
                        * f64::powi(
                            f64::powi(x[i], 2) - 2_f64 * x[i] * x[i + 1] + f64::powi(x[i + 1], 2),
                            2,
                        ));
    }
    res
}

/// # Paviani function
/// 10-dimensional only \
/// Global minimum: \
/// f(9.351, ...., 9.351) = ~−45.778
pub fn paviani(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        10,
        "Paviani function takes only a ten dimensional vector as a parameter."
    );
    let mut res = 0_f64;
    let mut sum = 0_f64;
    for arg in x {
        res += f64::powi(f64::ln(arg - 2_f64), 2) + f64::powi(f64::ln(10_f64 - arg), 2);
        sum *= arg;
    }

    res - f64::powf(sum, 0.2)
}

/// # Pinter function
/// Multidimensional
/// Global minimum: \
/// f(9.351, ...., 9.351) = ~−45.778
pub fn pinter(x: &[f64]) -> f64 {
    let mut res = 0_f64;
    for (i, val) in x.iter().enumerate() {
        res += i as f64 * f64::powi(*val, 2)
            + 20_f64 * i as f64 * f64::powi(f64::sin(x[i - 1] * f64::sin(*val) + f64::sin(x[i + 1])), 2)
            + i as f64
                * f64::log(
                    1_f64
                        + i as f64
                            * f64::powi(
                                f64::powi(x[i - 1], 2) - 2_f64 * val - 3_f64 * x[i - 1] - f64::cos(*val)
                                    + 1_f64,
                                2,
                            ),
                    10_f64,
                )
    }
    res
}

/// # Periodic function
/// Two-dimensional
/// Global minimum: \
/// f(0, 0) = 0.9
pub fn periodic(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Periodic function takes only a ten dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    1_f64 + f64::powi(f64::sin(x1), 2) + f64::powi(f64::sin(x2), 2)
        - 0.1 * f64::powf(f64::consts::E, -1_f64 * (f64::powi(x1, 2) + f64::powi(x2, 2)))
}

///# Perm 0, D, Beta function
/// 2-dimensional only\
/// Global minimum: \
/// f(1, 0.5, ... , 1/d) = 0
pub fn perm_0_d_beta(x: &[f64], beta: &f64) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Perm 0, D, Beta function takes only a two dimensional vector as a parameter."
    );
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

/// # Perm d, beta function
/// D-dimensional \
/// Global minimum:\
/// f(1,2,...,D) = 0

pub fn perm_d_beta(x: &[f64], beta: &f64) -> f64 {
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

pub fn powell(x: &[f64]) -> f64 {
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

/// # Powell Singular 2 function
/// D-dimensional, D>4 \
/// Global minimum: \
/// f(0,..,0) = 0

pub fn powell2(x: &[f64]) -> f64 {
    assert!(
        x.len() > 3,
        "Powell Singular 2 function takes at least a four dimensional vector as a parameter."
    );
    let mut res = 0_f64;
    for i in 1..x.len() - 3 {
        res += f64::powi(x[i - 1] + 10_f64 * x[i], 2)
            + 5_f64 * f64::powi(x[i + 1] - x[i + 2], 2)
            + f64::powi(x[i] - 2_f64 * x[i + 1], 4)
            + 1_f64 * f64::powi(x[i - 1] - x[i + 2], 4)
    }
    res
}

/// # Powell Sum function
/// Multi-dimensional \
/// Global minimum: \
/// f(0,..,0) = 0

pub fn powell_sum(x: &[f64]) -> f64 {
    let mut res = 0_f64;
    for (dim, arg) in x.iter().enumerate() {
        res += f64::powi(f64::abs(*arg), (dim + 1) as i32);
    }
    res
}

/// # Power sum function
/// D-dimensional, where b is a vector of length D \
/// Global minimum: \
/// f(0,...,0) = 0

pub fn power_sum(x: &[f64], b: &[f64]) -> f64 {
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

/// # Price 1 function
/// Two-dimensional
/// Global minimum: \
/// f(+-5, +-5) = 0
pub fn price1(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Price 1 function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(f64::abs(x1) - 5_f64, 2) + f64::powi(f64::abs(x2) - 5_f64, 2)
}

/// # Price 2 function
/// Two-dimensional
/// Global minimum: \
/// f(0, 0) = 0.9
pub fn price2(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Price 2 function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    1_f64 + f64::powi(f64::sin(x1), 2) + f64::powi(f64::sin(x2), 2)
        - 0.1 * f64::powf(f64::consts::E, -1_f64 * f64::powi(x1, 2) - f64::powi(x2, 2))
}

/// # Price 3 function
/// Two-dimensional
/// Global minimum: \
/// f(+-5, +-5) = 0
pub fn price3(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Price 3 function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    100_f64 * f64::powi(x2 - f64::powi(x1, 2), 2)
        + 6_f64 * f64::powi(6.4 * f64::powi(x2 - 0.5, 2) - x1 - 0.6, 2)
}

/// # Price 4 function
/// Two-dimensional
/// Global minimum: \
/// f(0, 0) = 0
pub fn price4(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Price 4 function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(2_f64 * f64::powi(x1, 3) * x2 - f64::powi(x2, 3), 2)
        + f64::powi(6_f64 * x1 - f64::powi(x2, 2) + x2, 2)
}

/// # Qing function
/// Multidimensional
/// Global minimum: \
/// f(+-sqrt(i)
pub fn qing(x: &[f64], i: &f64) -> f64 {
    let mut res = 0_f64;
    for arg in x {
        res += f64::powi(f64::powi(*arg, 2) - i, 2)
    }
    res
}

/// # Quadratic function
/// Two-dimensional
/// Global minimum: \
/// f(0.19388, 0.48513) = −3873.7243.
pub fn quadratic(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Quadratic function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    -3803.84 - 138.08 * x1 - 232.92 * x2
        + 128.08 * f64::powi(x1, 2)
        + 23.64 * f64::powi(x2, 2)
        + 182.25 * x1 * x2
}

// /// # Quartic function
// /// Multidimensional
// /// Global minimum: \
// /// f(0, ..., 0) = 0
// pub fn quartic(x: &[f64], i: &f64) -> f64 {
//   let mut res = 0_f64;
//   for (dim, arg) in x.iter().enumerate() {
//       res += dim*f64::powi(*arg, 4)+thread_rng().gen_range(0.0..1.0);
//     }
//   res
// }

/// # Quintic function
/// Multidimensional
/// Global minimum: \
/// f(-1, ..., -1) = f(2, ..., 2) = 0
pub fn quintic(x: &[f64]) -> f64 {
    let mut res = 0_f64;
    for arg in x {
        res += f64::abs(
            f64::powi(*arg, 5) - 3_f64 * f64::powi(*arg, 4)
                + 4_f64 * f64::powi(*arg, 3)
                + 2_f64 * f64::powi(*arg, 2)
                - 10_f64 * arg
                - 4_f64,
        )
    }
    res
}

/// # Rana function
/// Multidimensional

pub fn rana(x: &[f64]) -> f64 {
    let mut res = 0_f64;
    for (dim, arg) in x.iter().enumerate() {
        let t1 = f64::sqrt(f64::abs(x[dim + 1] + arg + 1_f64));
        let t2 = f64::sqrt(f64::abs(x[dim + 1] - arg + 1_f64));
        res += (x[dim + 1] + 1_f64) * f64::cos(t2) * f64::sin(t1) + arg * f64::cos(t1) * f64::sin(t2);
    }
    res
}

/// # Rastrigin function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn rastrigin(x: &[f64]) -> f64 {
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

/// # Ripple 1 function
/// Two-dimensional

pub fn ripple1(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Ripple 1 function takes only a two dimensional vector as a parameter."
    );
    let mut res = 0_f64;
    for arg in x {
        res += -1_f64
            * f64::powf(
                f64::consts::E,
                -2_f64 * f64::ln(2_f64 * f64::powi((arg - 1_f64) / 0.8, 2)),
            )
            * (f64::powi(f64::sin(5_f64 * f64::consts::PI * arg), 6)
                + 0.1 * f64::powi(f64::cos(500_f64 * f64::consts::PI * arg), 2))
    }
    res
}

/// # Ripple 25 function
/// Two-dimensional

pub fn ripple25(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Ripple 25 function takes only a two dimensional vector as a parameter."
    );
    let mut res = 0_f64;
    for arg in x {
        res += -1_f64
            * f64::powf(
                f64::consts::E,
                -2_f64 * f64::ln(2_f64 * f64::powi((arg - 1_f64) / 0.8, 2)),
            )
            * (f64::powi(f64::sin(5_f64 * f64::consts::PI * arg), 6))
    }
    res
}

/// # Rosenbrock function
/// n-dimensional \
/// Global minimum: \
/// f(1, ..., 1) = 0
#[allow(clippy::ptr_arg)]
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

/// # Rosenbrock modified function
/// Two-dimensional
/// Global minimum: \
/// f(-1, -1) = 0

pub fn rosenbrock_modified(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Ripple 25 function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    74_f64 + 100_f64 * f64::powi(x2 - f64::powi(x1, 2), 2) + f64::powi(1_f64 - x2, 2)
        - 400_f64
            * f64::powf(
                f64::consts::E,
                -1_f64 * (f64::powi(x1 + 1_f64, 2)) + f64::powi(x2 + 1_f64, 2) / 0.1,
            )
}

/// # Rotated Ellipse function
/// Two-dimensional
/// Global minimum: \
/// f(0, 0) = 0

pub fn rotated_ellipse(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Rotated Ellipse function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    7_f64 * f64::powi(x1, 2) - 6_f64 * f64::sqrt(3_f64) * x1 * x2 + 13_f64 * f64::powi(x2, 2)
}

/// # Rotated Ellipse 2 function
/// Two-dimensional
/// Global minimum: \
/// f(0, 0) = 0

pub fn rotated_ellipse2(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Rotated Ellipse 2 function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(x1, 2) - x1 * x2 + f64::powi(x2, 2)
}

/// # Rotated Hyper-Ellipsoid function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn rotated_hyper_ellipsoid(x: &[f64]) -> f64 {
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

/// # Rump function
/// Two-dimensional
/// Global minimum: \
/// f(0, 0) = 0

pub fn rump(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Rump function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    (333.75 - f64::powi(x1, 2)) * f64::powi(x2, 6)
        + f64::powi(x1, 2)
            * (11_f64 * f64::powi(x1, 2) * f64::powi(x2, 2) - 121_f64 * f64::powi(x2, 4) - 2_f64)
        + 5.5 * f64::powi(x2, 8)
        + x1 / 2_f64 * x2
}

/// # Salomon function
/// Multidimensional

pub fn salomon(x: &[f64]) -> f64 {
    let mut res = 0_f64;
    for arg in x {
        res += f64::powi(*arg, 2)
    }
    1_f64 - f64::cos(2_f64 * f64::consts::PI * res) + 0.1 * f64::sqrt(res)
}

/// # Sargan function
/// Multidimensional

pub fn sargan(x: &[f64]) -> f64 {
    let mut res = 0_f64;
    for (dim, val) in x.iter().enumerate() {
        let mut innersum = 0_f64;
        for j in 0..x.len() {
            if dim != j {
                innersum += val * x[j]
            }
            res += x.len() as f64 * (f64::powi(*val, 2) + 0.4 * innersum)
        }
    }
    res
}

/// # Schaffer N.1 function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, 0) = 0
pub fn schaffer_n1(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Schaffer function N.1 takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    0.5 + (f64::powi(f64::sin(f64::powi(x1, 2) + f64::powi(x2, 2)), 2) - 0.5)
        / (1_f64 + 0.001 * (f64::powi(x1, 2) + f64::powi(x2, 2)))
}

/// # Schaffer N.2 function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, 0) = 0
pub fn schaffer_n2(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Schaffer function N.2 takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    0.5 + (f64::powi(f64::sin(f64::powi(x1, 2) - f64::powi(x2, 2)), 2) - 0.5)
        / f64::powi(1.0 + 0.001 * (f64::powi(x1, 2) + f64::powi(x2, 2)), 2)
}

/// # Schaffer N.3 function
/// 2-dimensional only \

pub fn schaffer_n3(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Schaffer function N.3 takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    0.5 + (f64::powi(
        f64::sin(f64::cos(f64::abs(f64::powi(x1, 2) - f64::powi(x2, 2)))),
        2,
    ) - 0.5)
        / (1_f64 + 0.001 * f64::powi(f64::powi(x1, 2) + f64::powi(x2, 2), 2))
}

/// # Schaffer N.4 function
/// 2-dimensional only \
/// Global minima: \
/// f(0, 1.25313) = 0.292579 \
/// f(0, -1.25313) = 0.292579 \
/// f(1.25313, 0) = 0.292579 \
/// f(-1.25313, 0) = 0.292579
pub fn schaffer_n4(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Schaffer function N.4 takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    0.5 + (f64::powi(
        f64::cos(f64::sin(f64::abs(f64::powi(x1, 2) - f64::powi(x2, 2)))),
        2,
    ) - 0.5)
        / f64::powi(1.0 + 0.001 * (f64::powi(x1, 2) + f64::powi(x2, 2)), 2)
}

/// # Schmidt Vetters function
/// 3-dimensional only \
/// Global minimum: \
/// f(0.78547, 0.78547, 0.78547) = 3

pub fn schmidt_vetters(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        3,
        "Schmidt Vetters function takes only a three dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];
    1_f64 / (1_f64 + f64::powi(x1 - x2, 2))
        + f64::sin((f64::consts::PI * x2 + x3) / 2_f64)
        + f64::powf(f64::consts::E, f64::powi((x1 + x2) / 2_f64 - 2_f64, 2))
}

/// # Schumer Steiglitz function
/// Multidimensional
/// Global minimum:\
/// f(0, ..., 0) =

pub fn schumer_steiglitz(x: &[f64]) -> f64 {
    let mut res = 0_f64;
    for arg in x.iter() {
        res += f64::powi(*arg, 4)
    }
    res
}

/// # Schwefel function
/// n-dimensional \
/// Global minimum: \
/// f(420.9687, ..., 420.9687) = 0
pub fn schwefel(x: &[f64]) -> f64 {
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

//TODO implement remaining shwefel functions + refactor existing

/// # Shekel function
/// 4-dimensional only \
/// Global minimum:\
/// at m=5: f(4,4,4,4) = -10.1532 \
/// at m=7: f(4,4,4,4) = -10.4029 \
/// at m=10: f(4,4,4,4) = -10.5364

pub fn shekel(x: &[f64], m: i32, beta: &[f64], c: &[Vec<f64>]) -> f64 {
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

pub fn shekel_default(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        4,
        "Shekel function takes only a four dimensional vector as a parameter."
    );
    let mut res = 0.0;
    let raw = vec![1.0, 2.0, 2.0, 4.0, 4.0, 6.0, 3.0, 7.0, 5.0, 5.0];
    let beta = raw.into_iter().map(|x| 0.1 * x).collect::<Vec<_>>();
    let c = [vec![4.0, 1.0, 8.0, 6.0, 3.0, 2.0, 5.0, 8.0, 6.0, 7.0],
        vec![4.0, 1.0, 8.0, 6.0, 7.0, 9.0, 3.0, 1.0, 2.0, 3.6],
        vec![4.0, 1.0, 8.0, 6.0, 3.0, 2.0, 5.0, 8.0, 6.0, 7.0],
        vec![4.0, 1.0, 8.0, 6.0, 7.0, 9.0, 3.0, 1.0, 2.0, 3.6]];
    for i in 1..11 {
        let mut sum = 0_f64;
        for j in 1..5 {
            sum += f64::powi(x[j] - c[j][j], 2)
        }
        res += f64::powi(sum + beta[i as usize], -1)
    }
    res
}

//TODO implement remaining shekel functions + refactor existing

/// # Shubert function
/// 2-dimensional only \
/// Global minima: 18\
/// f(x*) = -186.7309

pub fn shubert(x: &[f64]) -> f64 {
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

//TODO implement remaining shubert functions + refactor existing

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

/// # Styblinski-Tang function
/// n-dimensional \
/// Global minimum: \
/// f(-2.903534, ..., -2.903534) = -39.16599n, where n - number of dimensions of argument vector
pub fn styblinski_tang(x: &[f64]) -> f64 {
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

/// # Streched V Sine Wave function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn stretched_v_sine_wave(x: &[f64]) -> f64 {
    assert!(
        !x.is_empty(),
        "Streched V Sine Wave function takes an at least one dimensional vector as a parameter."
    );
    let mut result: f64 = 0.0;
    for (index, x_curr) in x.iter().enumerate() {
        result += f64::powf(f64::powi(x[index + 1], 2) + f64::powi(*x_curr, 2), 0.25)
            * (f64::powi(
                f64::sin(50_f64 * f64::powf(f64::powi(x[index + 1], 2) + f64::powi(*x_curr, 2), 0.1)),
                2,
            ) + 0.1)
    }
    result
}

/// # Sum of Different Powers function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn sum_of_powers(x: &[f64]) -> f64 {
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
pub fn sum_squares(x: &[f64]) -> f64 {
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

/// # Testtube Holder function
/// 2-dimensional only \
/// Global minimum: \
/// f(+-pi/2, 0) = −10.872300
///
pub fn testtube_holder(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Testtube Holder function  takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    -4_f64
        * f64::sin(x1)
        * f64::cos(x2)
        * f64::powf(
            f64::consts::E,
            f64::abs(f64::cos(f64::powi(x1, 2) + f64::powi(x2, 2)) / 200_f64),
        )
}

/// # Trecanni function
/// 2-dimensional only \
/// Global minimum: \
/// f(0, 0) = f(-2, 0) = 0
///
pub fn trecanni(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Trecanni function  takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(x1, 4) - 4_f64 * f64::powi(x1, 3) + 4_f64 * x1 + f64::powi(x2, 2)
}

/// # Trid function
/// Alternatively: Trid 6
/// n-dimensional \
/// Global minimum: \
/// f((x_1, ..., x_n)) = -n * ( n + 4 ) * ( n - 1 ) / 6, where x_i = i * (n + 1 - i)
pub fn trid(x: &[f64]) -> f64 {
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

/// # Trid 10 function
/// n-dimensional \
/// Global minimum: \
/// f((x_1, ..., x_n)) = -n * ( n + 4 ) * ( n - 1 ) / 6, where x_i = i * (n + 1 - i)
pub fn trid10(x: &[f64]) -> f64 {
    assert!(
        x.len() >= 2,
        "Trid 10 function takes an at least two dimensional vector as a parameter."
    );
    let mut temp1 = 0_f64;
    let mut temp2 = 0_f64;
    for (dim, val) in x.iter().enumerate() {
        temp1 += f64::powi(val - 1_f64, 2);
        temp2 += val * x[dim - 1]
    }
    temp1 - temp2
}

/// # Trigonometric 1 function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn trigonometric1(x: &[f64]) -> f64 {
    assert!(
        x.len() >= 2,
        "Trigonometric 1 function takes an at least two dimensional vector as a parameter."
    );
    let mut res = 0_f64;
    let mut sum = 0_f64;
    for arg in x {
        sum += f64::cos(*arg);
    }
    for (dim, arg) in x.iter().enumerate() {
        res += f64::powi(
            x.len() as f64 - sum + dim as f64 * (1_f64 - f64::cos(*arg) - f64::sin(*arg)),
            2,
        )
    }
    res
}

/// # Trigonometric 2 function
/// n-dimensional \
/// Global minimum: \
/// f(0.9, ..., 0.9) = 0
pub fn trigonometric2(x: &[f64]) -> f64 {
    assert!(
        x.len() >= 2,
        "Trigonometric 2 function takes an at least two dimensional vector as a parameter."
    );
    let mut sum = 0_f64;
    for arg in x {
        sum += f64::cos(*arg);
    }
    for arg in x.iter() {
        sum += 8_f64 * f64::powi(f64::sin(7_f64 * f64::powi(arg - 0.9, 2)), 2)
            + 6_f64 * f64::powi(f64::sin(14_f64 * f64::powi(x[0] - 0.9, 2)), 2)
            + f64::powi(arg - 0.9, 2)
    }
    1_f64 + sum
}

//TODO: Implement tripod function

/// # Ursem 1 function
/// 2-dimensional only \

pub fn ursem1(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Ursem function N.1 takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    -1_f64 * f64::sin(2_f64 * x1 - 0.5 * f64::consts::PI) - 3_f64 * f64::cos(x2) - 0.5 * x1
}

/// # Ursem 3 function
/// 2-dimensional only \

pub fn ursem3(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Ursem function N.3 takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    -1_f64
        * f64::sin(2.2 * f64::consts::PI * x1 + 0.5 * f64::consts::PI)
        * (2_f64 - f64::abs(x1) / 2_f64)
        * (3_f64 - f64::abs(x1) / 2_f64)
        - 1_f64
            * f64::sin(2.2 * f64::consts::PI * x2 + 0.5 * f64::consts::PI)
            * (2_f64 - f64::abs(x2) / 2_f64)
            * (3_f64 - f64::abs(x2) / 2_f64)
}

/// # Ursem 4 function
/// 2-dimensional only \

pub fn ursem4(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Ursem function N.4 takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    -3_f64
        * f64::sin(0.5 * f64::consts::PI * x1 + f64::consts::PI * 0.5)
        * (2_f64 - f64::sqrt(f64::powi(x1, 2) + f64::powi(x2, 2)))
        / 4_f64
}

/// # Ursem Waves function
/// 2-dimensional only \

pub fn ursem_waves(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Ursem Waves function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    -0.9 * f64::powi(x1, 2)
        + x1 * x2 * (f64::powi(x2, 2) - 4.5 * f64::powi(x2, 2))
        + 4.7 * f64::cos(3_f64 * x1 - f64::powi(x2, 2) * (2_f64 + x1)) * f64::sin(2.5 * f64::consts::PI)
}

/// # Trefethen function
/// Two-dimensional \
/// Global minimum: \
/// f(−0.024403, 0.210612) = −3.30686865.
pub fn trefethen(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Trefethen function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powf(f64::consts::E, f64::sin(50_f64 * x1))
        + f64::sin(60_f64 * f64::powf(f64::consts::E, x2))
        + f64::sin(70_f64 * f64::sin(x1))
        + f64::sin(f64::sin(80_f64 * x2))
        - f64::sin(10_f64 * (x1 + x2))
        + 0.25 * (f64::powi(x1, 2) + f64::powi(x2, 2))
}

/// # Venter Sobiesczanski-Sobieski function
/// Two-dimensional \
/// Global minimum: \
/// f(−0.024403, 0.210612) = −3.30686865.
pub fn vss(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Venter Sobiesczanski-Sobieski function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(x1, 2) - 100_f64 * f64::powi(f64::cos(x1), 2) - 100_f64 * f64::cos(f64::powi(x1, 2) / 30_f64)
        + f64::powi(x2, 2)
        - 100_f64
            * f64::powf(
                f64::cos(x2),
                2_f64 - 100_f64 * f64::cos(f64::powi(x2, 2) / 30_f64),
            )
}

/// # Wayburn Seader 1 function
/// Alt.: Aluffi-Pentini’s function
/// 2-dimensional only \
/// Global minima: 18\
/// f(−0.0299, 0) = −0.003791

pub fn wayburn_seader1(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Wayburn Seader 1 function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(f64::powi(x1, 6) + f64::powi(x2, 4) - 17_f64, 2) + f64::powi(2_f64 * x1 + x2 - 4_f64, 2)
}

/// # Wayburn Seader 2 function
/// Alt.: Aluffi-Pentini’s function
/// 2-dimensional only \
/// Global minima: 18\
/// f(0.2, 1) = 0

pub fn wayburn_seader2(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Wayburn Seader 2 function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(
        1.613 - 4_f64 * f64::powi(x1 - 0.3125, 2) - 4_f64 * f64::powi(x2 - 1.625, 2),
        2,
    ) + f64::powi(x1 - 2_f64, 2)
}

/// # Wayburn Seader 3 function
/// Alt.: Aluffi-Pentini’s function
/// 2-dimensional only \
/// Global minimum:\
/// f(5.611, 6.187) = 21.35

pub fn wayburn_seader3(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Wayburn Seader 3 function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    2_f64 * f64::powi(x1, 3) / 3_f64 - 8_f64 * f64::powi(x1, 2) + 33_f64 * x1 - x1 * x2
        + 5_f64
        + f64::powi(f64::powi(x1 - 4_f64, 2) + f64::powi(x2 - 5_f64, 2) - 4_f64, 2)
}

/// # Wordmax
/// Global maximum in `chromosome.len()`
#[allow(clippy::ptr_arg)]
pub fn wordmax(chromosome: &Vec<bool>) -> f64 {
    chromosome.iter().filter(|gene| **gene).count() as f64
}

/// # Wolfe function
/// 3-dimensional only \
/// Global minimum:\
/// f() = (0, 0, 0) = 0

pub fn wolfe(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        3,
        "Wolfe function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];
    (4 / 3) as f64 * f64::powf(f64::powi(x1, 2) + f64::powi(x2, 2) - x1 * x2, 0.75) + x3
}

// /// # Xin-She Yang 1
// /// n-dimensional \
// /// Global minimum: \
// /// f(0, ..., 0) = 0
// pub fn xin_she_yang_1(x: &[f64]) -> f64 {
//   assert!(
//     !x.is_empty(),
//     "Xin-She Yang (Function 1) takes an at least one dimensional vector as a parameter."
//   );
//   let mut temp1 = 0.0;
//   for (index, x_curr) in x.iter().enumerate() {
//     temp1 += thread_rng().gen_range(0.0..1.0) * f64::powi(f64::abs(*x_curr), index as i32)
//   }
//   temp1 + f64::powi(temp2, 2) + f64::powi(temp2, 4)
// }

/// # Xin-She Yang 2
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn xin_she_yang_2(x: &[f64]) -> f64 {
    assert!(
        !x.is_empty(),
        "Xin-She Yang function 2 takes an at least one dimensional vector as a parameter."
    );
    let mut temp1 = 0.0;
    let mut temp2 = 0.0;
    for x_curr in x.iter() {
        temp1 += f64::abs(*x_curr);
        temp2 += f64::sin(f64::powi(*x_curr, 2))
    }
    temp1 * f64::exp(-temp2)
}

/// # Zakharov function
/// n-dimensional \
/// Global minimum: \
/// f(0, ..., 0) = 0
pub fn zakharov(x: &[f64]) -> f64 {
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

/// # Zettl function
/// Alt.: Aluffi-Pentini’s function
/// 2-dimensional only \
/// Global minima: 18\
/// f(−0.0299, 0) = −0.003791

pub fn zettl(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Zettl function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    f64::powi(f64::powi(x1, 2) + f64::powi(x2, 2) - 2_f64 * x1, 2) + 0.25 * x1
}

/// # Zirilli function
/// Alt.: Aluffi-Pentini’s function
/// 2-dimensional only \
/// Global minima: 18\
/// f(−1.0465, 0) =~ −0.3523

pub fn zirilli(x: &[f64]) -> f64 {
    assert_eq!(
        x.len(),
        2,
        "Zirilli function takes only a two dimensional vector as a parameter."
    );
    let x1 = x[0];
    let x2 = x[1];
    0.25 * f64::powi(x1, 4) - 0.5 * f64::powi(x1, 2) + 0.1 * x1 + 0.5 * f64::powi(x2, 2)
}
