#[allow(unused_imports)]
use ecrs::prelude::*;

// ECRS nakłada zbyt duże ograniczenia.
//
// 1. Postać osobnika jest sprowadzona jedynie do chormosomu -- w przypadku rozwiązania
// które próbuję zaimplementować posiadanie stanu jest krytyczne!
//

fn decode_chromosome(chromosome: &Vec<f64>) {
    // We deduce the problem size from the chromosome
    let n: usize = chromosome.len() / 2;

    let mut active_schedule = std::collections::HashSet::new();
    let mut finish_times = std::collections::HashSet::new();
    let mut scheduled = std::collections::HashSet::new();
    let mut e_set = std::collections::HashSet::<i32>::new();

    active_schedule.insert(0);
    finish_times.insert(0);
    scheduled.insert(0);

    let mut g = 1;
    let mut t = 0;

    while scheduled.len() < n + 1 {

    }
}

fn main() -> Result<(), ()> {

    Ok(())
}
