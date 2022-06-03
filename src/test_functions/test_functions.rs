use std::f64;


// Rastrigin function
pub fn rastrigin(x: &Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for x_curr in x.iter() {
        result += f64::powi(*x_curr, 2) - 10.0 * f64::cos(2.0 * f64::consts::PI * *x_curr);
    }
    result += 10.0 * x.len() as f64;
    return result;
}

// Ackley function
pub fn ackley(x: &Vec<f64>) -> f64 {
    let mut temp1 = 0 as f64;
    let mut temp2 = 0 as f64;
    for x_curr in x.iter() {
        temp1 += f64::powi(*x_curr, 2);
        temp2 += f64::cos(2.0 * f64::consts::PI * *x_curr)
    }
    temp1 *= (1.0 / x.len() as f64);
    temp2 *= (1.0 / x.len() as f64);
    return -20.0 * f64::exp(-0.2 * f64::sqrt(temp1)) - f64::exp(temp2) + 20.0 + f64::exp(1.0);
}

// Sphere
pub fn sphere(x: &Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for x_curr in x.iter() {
        result += f64::powi(*x_curr, 2)
    }
    return result;
}

// Rosenbrock
pub fn rosenbrock(x: &Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..x.len()-1 {
        result += 100.0 * f64::powi(x[i+1] - f64::powi(x[i], 2), 2) + f64::powi(1.0 - x[i], 2);
    }
    return result;
}

// Styblinski-Tang
pub fn styblinskitang(x: &Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for param in x.iter() {
        result += f64::powi(*param, 4) - 16.0 * f64::powi(*param, 2) + 5.0 * *param;
    }
    result *= 0.5;
    return result;
}

// Beale
pub fn beale(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("Beale function takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64 = 0.0;
    result += f64::powi(1.5 - x1 + x1 * x2, 2);
    result += f64::powi(2.25 - x1 + x1 * f64::powi(x2, 2), 2);
    result += f64::powi(2.625 - x1 + x1 * f64::powi(x2, 3), 2);
    return result;
}

// Goldstein-Price
pub fn goldsteinprice(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("Goldstein-Price function takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64 = 1.0;
    result *= (1.0 + f64::powi(x1 + x2 + 1.0,2) * (19.0 - 14.0 * x1 + 3.0 * f64::powi(x1,2) - 14.0 * x2 + 6.0 * x1 * x2 + 3.0 * f64::powi(x2,2)));
    result *= (30.0 + f64::powi(2.0 * x1 - 3.0 * x2,2) * (18.0 - 32.0 * x1 + 12.0 * f64::powi(x1, 2) + 48.0 * x2 - 36.0 * x1 * x2 + 27.0 * f64::powi(x2,2)));
    return result;
}

// Booth
pub fn booth(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("Booth function takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64 = 0.0;
    result += f64::powi(x1 + 2.0 * x2 - 7.0, 2);
    result += f64::powi(2.0 * x1 + x2 - 5.0, 2);
    return result;

}

// Bukin N.6
pub fn bukin6(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("Bukin function N.6 takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64 = 0.0;
    result += 100.0 * f64::sqrt(f64::abs(x2 - 0.01 * f64::powi(x1, 2)));
    result += 0.01 * f64::abs(x1 + 10.0);
    return result;
}

// Matyas
pub fn matyas(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("Matyas function takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64 = 0.0;
    result += 0.26 * (f64::powi(x1, 2) + f64::powi(x2, 2));
    result += -0.48 * x1 * x2;
    return result;
}

// Levi N.13
pub fn levi13(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("Levi function N.13 takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64 = 0.0;
    result += f64::powi(f64::sin(3.0 * f64::consts::PI * x1), 2);
    result += f64::powi(x1 - 1.0, 2) * (1.0 + f64::powi(f64::sin(3.0 * f64::consts::PI * x2), 2));
    result += f64::powi(x2 - 1.0, 2) * (1.0 + f64::powi(f64::sin(2.0 * f64::consts::PI * x2), 2));
    return result;
}

// Himmelblau's
pub fn himmelblau(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("Himmelblau's function takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64 = 0.0;
    result += f64::powi(f64::powi(x1, 2) + x2 - 11.0, 2);
    result += f64::powi(x1 + f64::powi(x2, 2) - 7.0, 2);
    return result;
}

// Three-hump camel
pub fn threehumpcamel(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("Three-hump camel function takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64 = 0.0;
    result += 2.0 * f64::powi(x1, 2);
    result += -1.05 * f64::powi(x1, 4);
    result += f64::powi(x1, 6) / 6.0;
    result += x1 * x2;
    result += f64::powi(x2, 2);
    return result;
}

// Easom
pub fn easom(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("Easom function takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64 = 1.0;
    result *= -1.0 * f64::cos(x1);
    result *= f64::cos(x2);
    result *= f64::exp(-1.0 * (f64::powi(x1 - f64::consts::PI, 2) + f64::powi(x2 - f64::consts::PI, 2)));
    return result;
}

// Cross-in-tray
pub fn crossintray(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("Cross-in-tray function takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64;
    result = -0.0001 * f64::powf(f64::abs(f64::sin(x1) * f64::sin(x2) * f64::exp(f64::abs(100.0 - f64::sqrt(f64::powi(x1, 2) + f64::powi(x2, 2)) / f64::consts::PI))) + 1.0, 0.1);
    return result;
}

// Eggholder
pub fn eggholder(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("Eggholder function takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64 = 0.0;
    result += -1.0 * (x2 + 47.0) * f64::sin(f64::sqrt(f64::abs((x1 / 2.0) + x2 + 47.0)));
    result += -1.0 * x1 * f64::sin(f64::sqrt(f64::abs(x1 - x2 - 47.0)));
    return result;
}

// Holder table
pub fn holdertable(x: &Vec<f64>) -> f64{
    if x.len() != 2 {
        panic!("Holder table function takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64;
    result = -1.0 * f64::abs(f64::sin(x1) * f64::cos(x2) * f64::exp(f64::abs(1.0 - f64::sqrt(f64::powi(x1, 2) + f64::powi(x2, 2)) / f64::consts::PI)));
    return result;
}

// McCormick
pub fn mcormick(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("McCormick table function takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64 = 0.0;
    result += f64::sin(x1 + x2);
    result += f64::powi(x1 - x2, 2);
    result += -1.5 * x1;
    result += 2.5 * x2;
    result += 1.0;
    return result;
}

// Schaffer N.2
pub fn schaffer2(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("Schaffer function N.2 takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64;
    result = 0.5 + (f64::powi(f64::sin(f64::powi(x1, 2) - f64::powi(x2, 2)), 2) - 0.5) / f64::powi(1.0 + 0.001 * (f64::powi(x1, 2) + f64::powi(x2, 2)), 2);
    return result;
}

// Schaffer N.4
pub fn schaffer4(x: &Vec<f64>) -> f64 {
    if x.len() != 2 {
        panic!("Schaffer function N.4 takes only a two dimensional vector as a parameter.");
    }
    let x1 = x[0];
    let x2 = x[1];
    let mut result: f64;
    result = 0.5 + (f64::powi(f64::cos(f64::sin(f64::abs(f64::powi(x1, 2) - f64::powi(x2, 2)))), 2) - 0.5) / f64::powi(1.0 + 0.001 * (f64::powi(x1, 2) + f64::powi(x2, 2)), 2);
    return result;
}
