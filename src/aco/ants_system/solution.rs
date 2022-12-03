use std::cmp::Ordering;

use crate::aco::into_vec;
use nalgebra::{Dynamic, OMatrix, RealField};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

type FMatrix = OMatrix<f64, Dynamic, Dynamic>;

/// Struct with matrix representing path and its cost
#[derive(Clone)]
pub struct Solution {
  pub matrix: FMatrix,
  pub cost: f64,
}

impl Default for Solution {
  fn default() -> Self {
    Self {
      matrix: FMatrix::zeros(0, 0),
      cost: f64::max_value().unwrap(),
    }
  }
}

impl Serialize for Solution {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s_struct = serializer.serialize_struct("solution", 2)?;
    let solution = into_vec(&self.matrix);
    s_struct.serialize_field("matrix", &solution)?;
    s_struct.serialize_field("cost", &self.cost)?;

    s_struct.end()
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
