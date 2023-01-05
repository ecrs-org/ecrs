//! # ECRS - Evolutionary Computation for Rust
//!
//! Evolutionary computation tools & algorithms (also some bioinspired ones)
//!
//! ## Disclaimer
//!
//! Please note that this library is in early development phase
//! and breaking changes may occur without nay notice.

#![allow(dead_code)]
#![allow(clippy::new_without_default)]
#![allow(clippy::type_complexity)]

extern crate core;

pub mod aco;
pub mod ff;
pub mod ga;
pub mod prelude;
pub mod pso;
pub mod test_functions;
