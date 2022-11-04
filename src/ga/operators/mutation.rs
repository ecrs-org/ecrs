use crate::ga::individual::{ChromosomeWrapper, Gene};

pub fn range_compliment<T: Gene, S: ChromosomeWrapper<T>>(individual: &mut S) -> S {
	individual.to_owned()
}
