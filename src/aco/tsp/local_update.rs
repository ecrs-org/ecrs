//! Local update rules mainly for [AntColonySystemAB]
use crate::aco::FMatrix;

/// # Local Update Rule
///
/// Trait defines common interface for local update rules.
pub trait LocalUpdate {
    fn apply(&mut self, pheromone: &mut FMatrix, partial_paths: &[Vec<usize>]);
}

/// Decay Rule
///
/// Implements [LocalUpdate].
///
/// Multiplies pheromone on last traveled edges by provided constant.
pub struct Decay {
    decay_rate: f64,
}

impl Decay {
    /// Creates a new instance of [Decay].
    ///
    ///  ## Arguments
    /// * `decay_rate` - Constant to multiply the pheromone by. Must be in range (0.0..1.0).
    pub fn new(decay_rate: f64) -> Self {
        assert!(
            (0.0..1.0).contains(&decay_rate),
            "Decay rate must be in range (0.0..1.0)"
        );
        Self { decay_rate }
    }
}

impl LocalUpdate for Decay {
    fn apply(&mut self, pheromone: &mut FMatrix, partial_paths: &[Vec<usize>]) {
        for p_path in partial_paths {
            let l = p_path.len();
            assert!(l > 1);
            let s = p_path[l - 2];
            let r = p_path[l - 1];

            pheromone[(s, r)] *= self.decay_rate;
        }
    }
}

/// Decay to Rule
///
/// Implements [LocalUpdate].
///
/// Multiplies pheromone on last traveled edges by provided constant (decay_rate) and adds another constant ((1-decay_rate) * stable_constant).
pub struct DecayTo {
    decay_rate: f64,
    stable_constant: f64,
}

impl DecayTo {
    /// Creates a new instance of [DecayTo].
    ///
    ///  ## Arguments
    /// * `decay_rate` - Constant to multiply the pheromone by. Must be in range (0.0..1.0).
    /// * `stable_constant` - to which value the pheromone should decay
    pub fn new(decay_rate: f64, stable_constant: f64) -> Self {
        assert!(
            (0.0..1.0).contains(&decay_rate),
            "Decay rate must be in range (0.0..1.0)"
        );
        Self {
            decay_rate,
            stable_constant,
        }
    }
}

impl LocalUpdate for DecayTo {
    fn apply(&mut self, pheromone: &mut FMatrix, partial_paths: &[Vec<usize>]) {
        for p_path in partial_paths {
            let l = p_path.len();
            let s = p_path[l - 2];
            let r = p_path[l - 1];

            pheromone[(s, r)] *= self.decay_rate;
            pheromone[(s, r)] += self.stable_constant * (1.0 - self.decay_rate);
        }
    }
}
