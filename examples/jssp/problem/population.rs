use std::path::PathBuf;

use ecrs::prelude::population::{PopulationGenerator, self};

use super::{individual::JsspIndividual, JsspInstance};

pub struct JsspPopProvider {
    path: PathBuf,
}

impl JsspPopProvider {
    fn new(path: PathBuf) -> Self {
        assert!(path.is_file());
        Self { path }
    }
}

impl PopulationGenerator<JsspIndividual> for JsspPopProvider {
    fn generate(&mut self, count: usize) -> Vec<JsspIndividual> {
        let instance = JsspInstance::try_from(self.path.clone());

        let Ok(instance) = instance else {
            panic!("Failed to load problem instance from file {:?}", self.path);
        };

        // Finding dimension of the chromosome
        let dim: usize = instance.jobs.iter().map(|job| job.len()).sum();

        let points_gen = population::RandomPoints::new(dim);

        points_gen.generate(count);

        vec![]
    }
}
