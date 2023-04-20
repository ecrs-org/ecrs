#[allow(unused_imports)]
use ecrs::prelude::*;
use ecrs::prelude::population::PopulationGenerator;

// ECRS nakłada zbyt duże ograniczenia.
//
// 1. Postać osobnika jest sprowadzona jedynie do chormosomu -- w przypadku rozwiązania
// które próbuję zaimplementować posiadanie stanu jest krytyczne!
//

fn decode_chromosome(chromosome: &Vec<f64>, rmc: &mut Vec<Vec<i32>>) {
    // We deduce the problem size from the chromosome
    let n: usize = chromosome.len() / 2;

    let mut priorities = vec![0; n];

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
       // Update e_set 
       // e_set = ???
       while !e_set.is_empty() {
        // Select operation with highest priority
        let j = e_set.iter().enumerate().max_by_key(|(_, &val)| val).map(|(idx, _)| idx);

        // Calculate earliset finish time (in terms of precedence only)
        earliset_finish_j =
       }
    }
}

struct JsspIndividual {
    chromosome: Vec<f64>,
    job_id: i32,
    fitness: i32,
    preds: Vec<i32>
}

impl JsspIndividual {
    fn calculate_fitness(&self) -> i32 {
        // We deduce the problem size from the chromosome
        let n: usize = self.chromosome.len() / 2;

        let mut priorities = vec![0; n];

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
           // Update e_set 
           // e_set = ???
           while !e_set.is_empty() {
            // Select operation with highest priority
            let j = e_set.iter().enumerate().max_by_key(|(_, &val)| val).map(|(idx, _)| idx);

            // Calculate earliset finish time (in terms of precedence only)
            earliset_finish_j =
           }
        }
        0
    }

}

fn run() -> () {
    const POPULATION_SIZE: usize = 2;

    // Generate initial population
    let mut counter = -1;
    let mut population: Vec<JsspIndividual> = ga::population::RandomPoints::with_constraints(2, vec![(0.0..1.0), (0.0..1.0)])
        .generate(POPULATION_SIZE)
        .into_iter()
        .map(|idv| {
            counter += 1;
            JsspIndividual {
                chromosome: idv.chromosome,
                job_id: counter,
                fitness: i32::MAX,
            }
        })
        .collect();
    // Evaluate population
    //
    //
    
    // For bounded number of iterations run evolution:
    // 1. Select with elitism 
    // 2. Uniform crossover or chromosomes (not decoded solutions)
    // 3. Instead of mutation 
}

fn main() -> Result<(), ()> {

    Ok(())
}
