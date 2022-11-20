use crate::pso::util::print_generic_vector;
use itertools::izip;
use num::{abs, One, Zero};
use rand::distributions::{Distribution, Uniform};
use std::fmt;

#[derive(Clone)]
pub struct Particle {
  pub position: Vec<f64>,
  pub velocity: Vec<f64>,
  pub best_position: Vec<f64>,
  pub value: f64,
  pub best_position_value: f64,
}

impl Particle {
  pub fn generate(
    dimensions: usize,
    lower_bound: f64,
    upper_bound: f64,
    function: &dyn Fn(&Vec<f64>) -> f64,
  ) -> Particle {
    let mut rng = rand::thread_rng();
    let position_distribution = Uniform::from(lower_bound..=upper_bound);
    let velocity_distribution =
      Uniform::from(-abs(upper_bound - lower_bound)..=(abs(upper_bound - lower_bound)));
    let mut position: Vec<f64> = Vec::new();
    let mut velocity: Vec<f64> = Vec::new();

    for _i in 0..dimensions {
      position.push(position_distribution.sample(&mut rng));
      velocity.push(velocity_distribution.sample(&mut rng));
    }
    let best_position: Vec<f64> = position.to_vec();

    Particle {
      position: position.clone(),
      velocity,
      best_position: best_position.clone(),
      value: function(&position),
      best_position_value: function(&best_position),
    }
  }

  pub fn update_velocity(
    &mut self,
    swarm_best_position: &Vec<f64>,
    inertia_weight: &f64,
    cognitive_coefficient: &f64,
    social_coefficient: &f64,
  ) {
    let mut rng = rand::thread_rng();
    let uniform_distribution = Uniform::from((f64::zero())..=(f64::one()));
    let mut updated_velocity: Vec<f64> = Vec::new();

    for (v_i, x_i, p_i, p_d) in izip!(
      &self.velocity,
      &self.position,
      &self.best_position,
      swarm_best_position
    ) {
      let r_1 = uniform_distribution.sample(&mut rng);
      let r_2 = uniform_distribution.sample(&mut rng);
      let updated_v_i = (*inertia_weight * *v_i)
        + (*cognitive_coefficient * r_1 * (*p_i - *x_i))
        + (*social_coefficient * r_2 * (*p_d - *x_i));
      updated_velocity.push(updated_v_i);
    }

    self.velocity = updated_velocity;
  }

  pub fn update_position(&mut self, function: fn(&Vec<f64>) -> f64) {
    let mut updated_position: Vec<f64> = Vec::new();

    for (x_i, v_i) in izip!(&self.position, &self.velocity) {
      let updated_x_i: f64 = *x_i + *v_i;
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
    write!(
      f,
      "Position: {}, velocity: {}, best position: {}, value: {}, best position value: {}",
      print_generic_vector(&self.position),
      print_generic_vector(&self.velocity),
      print_generic_vector(&self.best_position),
      &self.value,
      &self.best_position_value
    )
  }
}
