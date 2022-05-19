use std::fmt;
use rand::distributions::{Distribution, Uniform};
use num::{abs, One, Zero};
use itertools::izip;
use crate::pso::util::print_generic_vector;

#[derive(Clone)]
pub struct Particle {
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub best_position: Vec<f64>,
    pub value: f64,
    pub best_position_value: f64
}

impl Particle {
    pub fn generate(dimensions: usize, lower_bound: f64, upper_bound: f64, function: &dyn Fn(&Vec<f64>) -> f64) -> Particle {
        let mut rng = rand::thread_rng();
        let position_distribution = Uniform::from((lower_bound.clone())..=(upper_bound.clone()));
        let velocity_distribution = Uniform::from(-abs(upper_bound.clone() - lower_bound.clone())..=(abs(upper_bound.clone() - lower_bound.clone())));
        let mut position: Vec<f64> = Vec::new();
        let mut velocity: Vec<f64> = Vec::new();
        let best_position: Vec<f64>;

        for _i in 0..dimensions {
            position.push(position_distribution.sample(&mut rng));
            velocity.push(velocity_distribution.sample(&mut rng));
        }
        best_position = position.to_vec();

        let particle = Particle {
            position: position.clone(),
            velocity,
            best_position: best_position.clone(),
            value: function(&position),
            best_position_value: function(&best_position)
        };

        return particle;
    }

    pub fn update_velocity(&mut self, swarm_best_position: &Vec<f64>, inertia_weight: &f64, cognitive_coefficient: &f64, social_coefficient: &f64) {
        let mut rng = rand::thread_rng();
        let uniform_distribution = Uniform::from((f64::zero())..=(f64::one()));
        let mut updated_velocity: Vec<f64> = Vec::new();

        for (v_i, x_i, p_i, p_d) in izip!(&self.velocity, &self.position, &self.best_position, swarm_best_position) {
            let r_1 = uniform_distribution.sample(&mut rng);
            let r_2 = uniform_distribution.sample(&mut rng);
            let updated_v_i = (inertia_weight.clone() * v_i.clone()) + (cognitive_coefficient.clone() * r_1.clone() * (p_i.clone() - x_i.clone())) + (social_coefficient.clone() * r_2.clone() * (p_d.clone() - x_i.clone()));
            updated_velocity.push(updated_v_i);
        }

        self.velocity = updated_velocity;
    }

    pub fn update_position(&mut self, function: &dyn Fn(&Vec<f64>) -> f64) {
        let mut updated_position: Vec<f64> = Vec::new();

        for (x_i, v_i) in izip!(&self.position, &self.velocity) {
            let updated_x_i: f64 = x_i.clone() + v_i.clone();
            updated_position.push(updated_x_i);
        }

        self.position = updated_position.clone();
        self.value = function(&self.position);

        if self.value < self.best_position_value {
            self.best_position = updated_position;
            self.best_position_value = self.value;
        }
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position: {}, velocity: {}, best position: {}, value: {}, best position value: {}", print_generic_vector(&self.position), print_generic_vector(&self.velocity), print_generic_vector(&self.best_position), &self.value, &self.best_position_value)
    }
}
