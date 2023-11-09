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
        _metadata: &ecrs::ga::GAMetadata,
        _population: &'a [JsspIndividual],
        _count: usize,
    ) -> Vec<&'a JsspIndividual> {
        Vec::new()
    }
}
