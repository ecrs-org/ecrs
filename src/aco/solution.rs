use crate::aco::FMatrix;
use itertools::Itertools;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

/// Struct with path vector and its fitness
#[derive(Clone)]
pub struct Solution {
  pub path: Vec<usize>,
  pub fitness: f64,
}

impl Default for Solution {
  fn default() -> Self {
    Self {
      path: vec![],
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
      path,
      ..Self::default()
    }
  }

  pub fn matrix(&self) -> FMatrix {
    let mut m = FMatrix::zeros(self.path.len(), self.path.len());
    for (i, j) in self.path.iter().circular_tuple_windows::<(&usize, &usize)>() {
      m[(*i, *j)] = 1.0;
      m[(*j, *i)] = 1.0;
    }
    m
  }
}

impl Serialize for Solution {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s_struct = serializer.serialize_struct("solution", 2)?;
    s_struct.serialize_field("path", &self.path)?;
    s_struct.serialize_field("fitness", &self.fitness)?;

    s_struct.end()
  }
}

impl PartialEq<Self> for Solution {
  fn eq(&self, other: &Self) -> bool {
    self.fitness == other.fitness && self.matrix() == other.matrix()
  }
}
