use crate::ga::individual::Chromosome;

use super::Probe;

pub struct EmptyProbe;

impl EmptyProbe {
    pub fn new() -> Self {
        EmptyProbe
    }
}

impl<T: Chromosome> Probe<T> for EmptyProbe {}
