//! Convinence reexports for common used types & functions
//!
//! ## Usage
//!
//! ```
//! use rand::prelude::*;
//! ```

#[cfg(feature = "aco")]
pub use crate::aco;
#[cfg(feature = "ff")]
pub use crate::ff;
#[cfg(feature = "ga")]
pub use crate::ga;
#[cfg(feature = "ga")]
pub use crate::ga::operators as ops;
#[cfg(feature = "ga")]
pub use crate::ga::operators::{crossover, fitness, mutation, replacement, selection};
#[cfg(feature = "ga")]
pub use crate::ga::population;
#[cfg(feature = "pso")]
pub use crate::pso;
pub use crate::test_functions as tf;
