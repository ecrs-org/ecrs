use std::fmt;
use rand::distributions::{Distribution, Uniform};
use num::{abs, One, Zero};
use itertools::izip;
use crate::particle_swarm_optimization::util::print_generic_vector;

#[derive(Clone)]
pub struct Particle<T> {
    pub position: Vec<T>,
    pub velocity: Vec<T>,
    pub best_position: Vec<T>
}

impl<T: rand::distributions::uniform::SampleUniform + std::clone::Clone + num::Signed + std::cmp::PartialOrd> Particle<T> {
    pub fn generate(dimensions: usize, lower_bound: T, upper_bound: T) -> Particle<T> {
        let mut rng = rand::thread_rng();
        let position_distribution = Uniform::from((lower_bound.clone())..=(upper_bound.clone()));
        let velocity_distribution = Uniform::from(-abs(upper_bound.clone() - lower_bound.clone())..=(abs(upper_bound.clone() - lower_bound.clone())));
        let mut position: Vec<T> = Vec::new();
        let mut velocity: Vec<T> = Vec::new();
        let best_position: Vec<T>;

        for _i in 0..dimensions {
            position.push(position_distribution.sample(&mut rng));
            velocity.push(velocity_distribution.sample(&mut rng));
        }
        best_position = position.to_vec();

        let particle = Particle {
            position,
            velocity,
            best_position
        };

        return particle;
    }

    pub fn update_velocity(&mut self, swarm_best_position: &Vec<T>, inertia_weight: &T, cognitive_coefficient: &T, social_coefficient: &T) {
        let mut rng = rand::thread_rng();
        let uniform_distribution = Uniform::from((T::zero())..=(T::one()));
        let mut updated_velocity: Vec<T> = Vec::new();

        for (v_i, x_i, p_i, p_d) in izip!(&self.velocity, &self.position, &self.best_position, swarm_best_position) {
            let r_1 = uniform_distribution.sample(&mut rng);
            let r_2 = uniform_distribution.sample(&mut rng);
            let updated_v_i = (inertia_weight.clone() * v_i.clone()) + (cognitive_coefficient.clone() * r_1.clone() * (p_i.clone() - x_i.clone())) + (social_coefficient.clone() * r_2.clone() * (p_d.clone() - x_i.clone()));
            updated_velocity.push(updated_v_i);
        }

        self.velocity = updated_velocity;
    }

    pub fn update_position(&mut self, function: &dyn Fn(Vec<T>) -> T) {
        let mut updated_position: Vec<T> = Vec::new();

        for (x_i, v_i) in izip!(&self.position, &self.velocity) {
            let updated_x_i: T = x_i.clone() + v_i.clone();
            updated_position.push(updated_x_i);
        }

        self.position = updated_position.clone();

        let position = self.position.clone();
        let best_position = self.best_position.clone();

        if function(position) < function(best_position) {
            self.best_position = updated_position;
        }
    }
}

impl<T: std::fmt::Display> fmt::Display for Particle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position: {}, velocity: {}, best position: {}", print_generic_vector(&self.position), print_generic_vector(&self.velocity), print_generic_vector(&self.best_position))
    }
}
