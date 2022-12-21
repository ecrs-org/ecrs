use crate::aco::util::into_vec;
use crate::aco::FMatrix;
use itertools::Itertools;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::cmp::Ordering;

/// Struct with matrix representing path and its cost
#[derive(Clone)]
pub struct Solution {
  pub matrix: FMatrix,
  pub path: Vec<usize>,
  pub cost: f64,
  pub fitness: f64,
}

impl Default for Solution {
  fn default() -> Self {
    Self {
      matrix: FMatrix::zeros(0, 0),
      path: vec![],
      cost: f64::MAX,
      fitness: 0.0,
    }
  }
}

impl Solution {
  pub fn from_path(path: Vec<usize>) -> Self {
    let mut matrix = FMatrix::zeros(path.len(), path.len());
    for (i, j) in path.iter().circular_tuple_windows::<(&usize, &usize)>() {
      matrix[(*i, *j)] = 1.0;
      matrix[(*j, *i)] = 1.0;
    }

    Self {
      matrix,
      path,
      ..Self::default()
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
