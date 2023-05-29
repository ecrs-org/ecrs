use ecrs::prelude::{population::PopulationGenerator, replacement::ReplacementOperator};

use super::{individual::JsspIndividual, population::JsspPopProvider};

pub struct JsspReplacement {
    pop_gen: JsspPopProvider,
    elite_rate: f64,
    sample_rate: f64,
}

impl JsspReplacement {
    pub fn new(pop_gen: JsspPopProvider, elite_rate: f64, sample_rate: f64) -> Self {
        Self {
            pop_gen,
            elite_rate,
            sample_rate,
        }
    }
}

impl ReplacementOperator<JsspIndividual> for JsspReplacement {
    fn apply(
        &mut self,
        mut population: Vec<JsspIndividual>,
        mut children: Vec<JsspIndividual>,
    ) -> Vec<JsspIndividual> {
        let elite_size: usize = (self.elite_rate * population.len() as f64) as usize;
        let sample_size: usize = (self.sample_rate * population.len() as f64) as usize;
        let crossover_size: usize = population.len() - elite_size - sample_size;

        assert!((0..population.len()).contains(&crossover_size));
        assert_eq!(elite_size + sample_size + crossover_size, population.len());

        if elite_size > 0 {
            population.sort();
        }

        for i in elite_size..(elite_size + crossover_size) {
            std::mem::swap(&mut population[i], &mut children[i - elite_size]);
        }

        if sample_size > 0 {
            population.splice(
                (elite_size + crossover_size)..population.len(),
                self.pop_gen.generate(sample_size),
            );
        }

        population
    }

    fn requires_children_fitness(&self) -> bool {
        false
    }
}
