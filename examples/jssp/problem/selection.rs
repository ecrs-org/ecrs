use ecrs::prelude::selection::SelectionOperator;

use super::individual::JsspIndividual;

pub struct EmptySelection;

impl EmptySelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionOperator<JsspIndividual> for EmptySelection {
    fn apply<'a>(
        &mut self,
        _metrics: &ecrs::ga::Metrics,
        _population: &'a [JsspIndividual],
    ) -> Vec<&'a JsspIndividual> {
        Vec::new()
    }
}
