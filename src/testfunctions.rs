use std::f64;

//----------------------
//Rastrigin
pub fn rastrigin(params:&Vec<f64>) -> f64{
    let mut res = 0 as f64;
    for param in params.iter(){
        res += param * param - 10 as f64 * f64::cos(2 as f64 * f64::consts::PI*param);
    }
    res + 10 as f64 * params.len() as f64
}
//----------------------

//Ackley

//Wymagamy parametrów, wpisane są polecane
const A:u8 = 20;
const B:f32 = 0.2;
const C:f64 = 2 as f64*f64::consts::PI;

fn ackley(params: &Vec<f64>) -> f64 {
    let mut temp1 = 0 as f64;
    let mut temp2 = 0 as f64;
    for param in params.iter(){
        temp1 += f64::powi(*param, 2);
        temp2 += f64::cos(C**param)
    }
    -1 as f64 * A as f64 * f64::exp(-1 as f64 * B as f64* f64::sqrt((1 as f64/params.len() as f64) * temp1)) - f64::exp((1 as f64/params.len() as f64) * temp2) + A as f64 + f64::exp(1 as f64)
}

//----------------------
//Sphere

fn sphere(params:&Vec<f64>) -> f64{
    let mut res = 0 as f64;
    for param in params.iter(){
        res += f64::powi(*param, 2)
    }
    res
}

//----------------------
//Rosenbrock

fn rosenbrock(params:&Vec<f64>) -> f64{
    let mut res = 0 as f64;
    for i in 1 as usize..params.len() -1 as usize{
        res += 100 as f64 * f64::powi(params[i+1 as usize] - f64::powi(params[i],2),2) + f64::powi(1 as f64 - params[i], 2)
    }
    res
}

//----------------------
//Beale
//TODO DWUWYMIAROWA, DODAĆ OBSŁUGĘ

fn beale(params:&Vec<f64>) -> f64{
    f64::powi(1.5 as f64 - params[0] + params[0]*params[1],2) + f64::powi(2.25 - params[0] *params[0]*f64::powi(params[1],2),2) + f64::powi(2.625 - params[0] + params[0] * f64::powi(params[1],3),2)
}

//----------------------
//Goldstein-Price
//TODO DWUWYMIAROWA
//TODO MOGŁEM SIĘ TU WALNĄĆ
fn goldsteinprice(params:&Vec<f64>) -> f64{
    let x1 = params[0];
    let x2 = params[1];
    (1 as f64 + f64::powi(x1 + x2 + 1 as f64,2) * (19 as f64 - 14 as f64 * x1 + 3 as f64* f64::powi(x1,2) -14 as f64*x2 + 6 as f64*x1*x2 +3 as f64 * f64::powi(x2,3)))*(30 as f64 + f64::powi(2 as f64 * x1 - 3 as f64 * x2,2)*(18 as f64 - 32 as f64 * x1 + 12 as f64 * f64::powi(x1, 2) + 48 as f64 * x2 -36 as f64*x1*x2 + 27 as f64 * f64::powi(x2,2)))
}

//----------------------
//Booth
//TODO DWUWYMIAROWA
fn booth(params:&Vec<f64>) -> f64{
    f64::powi(params[0] + 2 as f64 * params[1] - 7 as f64, 2) + f64::powi(2 as f64 * params[0] + params[1] - 5 as f64, 2)
}

//----------------------
//Bukin n. 6
//TODO DWUWYMIAROWA
fn bukin6(params:&Vec<f64>) -> f64{
    100 as f64 * f64::sqrt(f64::abs(params[1] - 0.01 * f64::powi(params[0], 2))) + 0.01*f64::abs(params[0] + 10 as f64)
}

//----------------------
//Matyas
//TODO DWUWYMIAROWA
fn matyas(params:&Vec<f64>) -> f64{
    0.26 * (f64::powi(params[0], 2) + f64::powi(params[1], 2)) - 0.48*params[0]*params[1]
}

//----------------------
//Levi n. 13
//TODO DWUWYMIAROWA
fn levi13(params:&Vec<f64>) -> f64{
    f64::powi(f64::sin(3 as f64 * params[0] * f64::consts::PI),2) + f64::powi(params[0] - 1 as f64, 2)*(1 as f64 + f64::powi(f64::sin(3 as f64 * f64::consts::PI * params[1]),2)) + f64::powi(params[1] - 1 as f64, 2)*(1 as f64 + f64::powi(f64::sin(2 as f64* f64::consts::PI * params[1]),2))
}

//----------------------
//Himmelblau's

fn himmelblau(params:&Vec<f64>) -> f64{
    f64::powi(f64::powi(params[0], 2) + params[1] - 11 as f64, 2) + f64::powi(params[0] + f64::powi(params[1],2) - 7 as f64, 2)
}

//----------------------
//Three-hump camel
//TODO DWUWYMIAROWA
fn threehumpcamel(params:&Vec<f64>) -> f64{
    2 as f64 * f64::powi(params[0], 2) - 1.05 * f64::powi(params[0], 4) + f64::powi(params[0], 6)/6 as f64 + params[0]+params[1] + f64::powi(params[1],2)
}

//----------------------
//Easom
//TODO DWUWYMIAROWA
fn easom(params:&Vec<f64>) -> f64{
    -1 as f64 * f64::cos(params[0]) * f64::cos(params[1]) * f64::exp(-1 as f64 * f64::powi(params[0] - f64::consts::PI,2) - f64::powi(params[1] - f64::consts::PI, 2))
}

//----------------------
//Cross-in-tray
//TODO DWUWYMIAROWA
fn crossintray(params:&Vec<f64>) -> f64{
    -0.0001 * f64::powf(f64::abs(f64::sin(params[0])*f64::sin(params[1]) * f64::exp(f64::abs(100 as f64 - f64::sqrt(f64::powi(params[0],2)+f64::powi(params[1],2))/f64::consts::PI))) + 1 as f64, 0.1)
}

//----------------------
//Eggholder
//TODO DWUWYMIAROWA
fn eggholder(params:&Vec<f64>) -> f64{
    -1 as f64 * (params[1] + 47 as f64) * f64::sin(f64::sqrt(f64::abs(params[1] + params[0]/2 as f64 + 47 as f64))) - params[0]*f64::sin(f64::sqrt(f64::abs(params[0] - params[1] - 47 as f64)))
}

//----------------------
//Holder table
//TODO DWUWYMIAROWA
fn holdertable(params:&Vec<f64>) -> f64{
    -1 as f64 *f64::abs(f64::sin(params[0]) * f64::cos(params[1])*f64::exp(f64::abs(1 as f64 - f64::sqrt(f64::powi(params[0],2)+ f64::powi(params[1],2))/f64::consts::PI)))
}

//----------------------
//McCormick
//TODO DWUWYMIAROWA
fn mcormick(params:&Vec<f64>) -> f64{
    f64::sin(params[0] + params[1]) + f64::powi(params[0] - params[1], 2) - 1.5 * params[0] + 2.5 * params[1] + 1 as f64
}

//----------------------
//Schaffer N.2
//TODO DWUWYMIAROWA
fn schaffer2(params:&Vec<f64>) -> f64{
    0.5 + (f64::powi(f64::powi(params[0], 2) + f64::powi(params[1], 2),2) - 0.5)/f64::powi(1 as f64 + 0.001 * (f64::powi(params[0],2) + f64::powi(params[1],2)), 2)
}

//----------------------
//Schaffer N.4
//TODO DWUWYMIAROWA
fn schaffer4(params:&Vec<f64>) -> f64{
    0.5 + (f64::powi(f64::cos(f64::sin(f64::abs(f64::powi(params[0],2) + f64::powi(params[1],2)))) ,2) - 0.5)/f64::powi(1 as f64 + 0.001 * (f64::powi(params[0], 2) + f64::powi(params[1] ,2)),2)
}

//----------------------
//Styblinski-Tang

fn styblinskitang(params:&Vec<f64>) -> f64{
    let mut res = 0 as f64;
    for param in params.iter(){
        res += 5 as f64*param - 16 as f64 * f64::powi(*param, 2) + f64::powi(*param, 4)
    }
    0.5 * res
}