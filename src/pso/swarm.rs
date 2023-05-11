use crate::pso::particle::Particle;
use crate::pso::util::print_generic_vector;
use rand::distributions::Uniform;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::fmt;

#[derive(Clone)]
pub struct Swarm {
    pub particles: Vec<Particle>,
    pub best_position: Vec<f64>,
    pub best_position_value: f64,
    distribution: Uniform<f64>,
}

impl Swarm {
    pub fn generate(
        particle_count: usize,
        dimensions: usize,
        lower_bound: f64,
        upper_bound: f64,
        function: fn(&Vec<f64>) -> f64,
    ) -> Swarm {
        let distribution = Uniform::new_inclusive(0.0, 1.0);

        let mut particles: Vec<Particle> = Vec::new();
        for _i in 0..particle_count {
            particles.push(Particle::generate(
                dimensions,
                lower_bound,
                upper_bound,
                function,
                &distribution,
            ));
        }

        let mut best_position = particles[0].clone().best_position;
        for particle in &particles {
            if function(&particle.position) < function(&best_position) {
                best_position = particle.position.clone();
            }
        }

        Swarm {
            particles,
            best_position: best_position.clone(),
            best_position_value: function(&best_position),
            distribution,
        }
    }

    pub fn update_velocities(
        &mut self,
        inertia_weight: &f64,
        cognitive_coefficient: &f64,
        social_coefficient: &f64,
    ) {
        self.particles.par_iter_mut().for_each(|particle| {
            particle.update_velocity(
                &self.best_position,
                inertia_weight,
                cognitive_coefficient,
                social_coefficient,
                &self.distribution,
            )
        });
    }

    pub fn update_positions(&mut self, function: fn(&Vec<f64>) -> f64) {
        self.particles
            .par_iter_mut()
            .for_each(|particle| particle.update_position(function));
    }

    pub fn update_best_position(&mut self, function: fn(&Vec<f64>) -> f64) {
        self.particles.iter_mut().for_each(|particle| {
            if function(&particle.best_position) < function(&self.best_position) {
                self.best_position = particle.best_position.clone();
                self.best_position_value = particle.best_position_value;
            }
        });
    }
}

impl fmt::Display for Swarm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Particles: {}\nBest swarm position: {}\nBest position value: {}",
            print_generic_vector(&self.particles),
            print_generic_vector(&self.best_position),
            self.best_position_value
        )
    }
}
