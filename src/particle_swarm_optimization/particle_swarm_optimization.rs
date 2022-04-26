use std::borrow::Borrow;
use std::fmt;
use itertools::izip;
use rand::distributions::Uniform;
use rand::prelude::*;
use num::{abs, NumCast, One};

use crate::particle_swarm_optimization::util::print_generic_vector;
use crate::particle_swarm_optimization::particle::Particle;
use crate::particle_swarm_optimization::swarm::Swarm;



struct PSOAlgorithm<T> {
    dimensions: usize,
    lower_bound: T,
    upper_bound: T,
    particle_count: usize,
    inertia_weight: T,
    cognitive_coefficient: T,
    social_coefficient: T,
    function: Box<dyn Fn(Vec<T>) -> T>,
    iterations: usize,
    swarm: Swarm<T>
}

impl<T: rand::distributions::uniform::SampleUniform + std::marker::Copy + std::cmp::PartialOrd + num::Signed + std::default::Default + std::ops::AddAssign> PSOAlgorithm<T> {
    fn new(dimensions: usize, lower_bound: T, upper_bound: T, particle_count: usize, inertia_weight: T, cognitive_coefficient: T, social_coefficient: T, function: &'static dyn Fn(Vec<T>) -> T, iterations: usize) -> PSOAlgorithm<T> {
        return PSOAlgorithm {
            dimensions,
            lower_bound: lower_bound.clone(),
            upper_bound: upper_bound.clone(),
            particle_count,
            inertia_weight,
            cognitive_coefficient,
            social_coefficient,
            function: Box::new(function),
            iterations,
            swarm: Swarm::generate(particle_count, dimensions, lower_bound, upper_bound, function.borrow())
        };
    }

    fn run(&mut self) {
        for _i in 0..self.iterations {
            self.swarm.update_velocities(&self.inertia_weight, &self.cognitive_coefficient, &self.social_coefficient);
            self.swarm.update_positions(&self.function);
            self.swarm.update_best_position(&self.function);
        }
    }
}

// fn simple_function<T: Default + std::ops::Mul<Output = T> + std::ops::AddAssign + Copy>(x: Vec<T>) -> T {
//     let mut value: T = T::default();
//     for x_i in x {
//         value += x_i * x_i;
//     }
//     return value;
// }

fn rosenbrock<T: Copy + Default + One + NumCast + std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + std::ops::AddAssign>(x: Vec<T>) -> T {
    let _100: T = NumCast::from(100).unwrap();
    let mut value: T = T::default();
    for i in 0..x.len() {
        if i == x.len() - 1 {
            break;
        }
        let x_curr: T = x[i];
        let x_next: T = x[i+1];
        value += _100 * (x_next - (x_curr * x_curr)) * (x_next - (x_curr * x_curr)) + (T::one() - x_curr)*(T::one() - x_curr);
    }
    return value;
}

pub fn pso_demo() {
    let mut algorithm =
        PSOAlgorithm::new(3,
                          -10.0,
                          10.0,
                          30,
                          0.5,
                          1.0,
                          3.0,
                          &rosenbrock,
                          5000);

    println!("Swarm at the start:\n{}", &algorithm.swarm);
    println!("Function value at best positon: {}", rosenbrock(algorithm.swarm.best_position.clone()));

    algorithm.run();

    println!("\nSwarm at the end:\n{}", &algorithm.swarm);
    println!("Function value at best positon: {}", rosenbrock(algorithm.swarm.best_position.clone()));

    println!("\nRosenbrock function value at [1, 1, 1]: {}", rosenbrock(vec![1, 1, 1]));
}