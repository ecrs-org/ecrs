#![allow(dead_code)]
#![allow(clippy::new_without_default)]
#![allow(clippy::type_complexity)]

extern crate core;

pub mod aco;
pub mod ff;
pub mod ga;
pub mod pso;

#[cfg(feature = "test_functions")]
pub mod test_functions;
