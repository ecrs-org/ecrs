use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Individual {
  pub chromosome: Vec<f64>,
  pub fitness: f64,
}

impl Eq for Individual {}

impl PartialEq<Self> for Individual {
  fn eq(&self, other: &Self) -> bool {
    if let Some(result) = self.fitness.partial_cmp(&other.fitness) {
      return result == Ordering::Equal;
    }
    unimplemented!();
  }
}

impl PartialOrd<Self> for Individual {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.fitness.partial_cmp(&other.fitness)
  }
}

impl Ord for Individual {
  fn cmp(&self, other: &Self) -> Ordering {
    if let Some(result) = self.fitness.partial_cmp(&other.fitness) {
      return result;
    }
    unimplemented!();
  }
}
//
// impl Display for Individual {
//   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//     todo!()
//
//   }
// }
