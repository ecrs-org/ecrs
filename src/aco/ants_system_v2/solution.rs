use std::cmp::Ordering;
use nalgebra::{Dynamic, OMatrix, RealField};

type FMatrix = OMatrix<f64, Dynamic, Dynamic>;

#[derive(Clone)]
pub struct Solution {
    pub matrix: FMatrix,
    pub cost: f64,
}

impl Default for Solution {
    fn default() -> Self {
        Self {
            matrix: FMatrix::zeros(0,0),
            cost: f64::max_value().unwrap()
        }
    }
}

impl PartialEq<Self> for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.matrix == other.matrix
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }

    fn lt(&self, other: &Self) -> bool {
        self.cost.lt(&other.cost)
    }

    fn le(&self, other: &Self) -> bool {
        self.cost.le(&other.cost)
    }

    fn gt(&self, other: &Self) -> bool {
        self.cost.gt(&other.cost)
    }

    fn ge(&self, other: &Self) -> bool {
        self.cost.ge(&other.cost)
    }
}