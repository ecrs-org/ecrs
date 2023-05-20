use crate::ga::individual::IndividualTrait;

use super::Probe;

pub struct EmptyProbe;

impl EmptyProbe {
    pub fn new() -> Self {
        EmptyProbe
    }
}

impl<IndividualT: IndividualTrait> Probe<IndividualT> for EmptyProbe {}
