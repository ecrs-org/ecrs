use std::fmt;
use crate::particle_swarm_optimization::particle::Particle;
use crate::particle_swarm_optimization::util::print_generic_vector;

#[derive(Clone)]
pub struct Swarm<T> {
    pub particles: Vec<Particle<T>>,
    pub best_position: Vec<T>
}

impl<T: rand::distributions::uniform::SampleUniform + std::clone::Clone + std::marker::Copy + std::cmp::PartialOrd + num::Signed> Swarm<T> {
    pub fn generate(particle_count: usize, dimensions: usize, lower_bound: T, upper_bound: T, function: &dyn Fn(Vec<T>) -> T) -> Swarm<T> {
        let mut particles: Vec<Particle<T>> = Vec::new();
        for _i in 0..particle_count {
            particles.push(Particle::generate(dimensions, lower_bound, upper_bound));
        }
        let mut best_position = particles[0].clone().best_position;
        for particle in &particles {
            if function(particle.clone().position) < function(best_position.clone()) {
                best_position = particle.position.clone();
            }
        }
        let swarm = Swarm {
            particles,
            best_position: best_position.clone()
        };
        return swarm;
    }

    pub fn update_velocities(&mut self, inertia_weight: &T, cognitive_coefficient: &T, social_coefficient: &T) {
        for i in 0..self.particles.len() {
            self.particles[i].update_velocity(&self.best_position, inertia_weight, cognitive_coefficient, social_coefficient);
        }
    }

    pub fn update_positions(&mut self, function: &dyn Fn(Vec<T>) -> T) {
        for i in 0..self.particles.len() {
            self.particles[i].update_position(function);
        }
    }

    pub fn update_best_position(&mut self, function: &dyn Fn(Vec<T>) -> T) {
        for i in 0..self.particles.len() {
            if function(self.particles[i].best_position.clone()) < function(self.best_position.clone()) {
                self.best_position = self.particles[i].best_position.clone();
            }
        }
    }
}

impl<T: std::fmt::Display> fmt::Display for Swarm<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Best swarm position: {}, Particles: {}", print_generic_vector(&self.best_position), print_generic_vector(&self.particles))
    }
}
