pub mod builder;
pub mod individual;
pub mod operators;
pub mod population;
pub mod probe;

pub use builder::*;
pub use individual::Individual;
pub use probe::csv_probe::CsvProbe;
pub use probe::json_probe::JsonProbe;
pub use probe::stdout_probe::StdoutProbe;
pub use probe::Probe;

use self::{
    individual::Chromosome,
    operators::{
        crossover::CrossoverOperator, mutation::MutationOperator, selection::SelectionOperator,
    },
    population::PopulationGenerator,
};

// trait FitnessFn

type FitnessFn<S> = fn(&S) -> f64;

pub struct GAParams {
    pub selection_rate: f64,
    pub generation_upper_bound: usize,
    pub population_size: usize,
    pub max_duration: Option<std::time::Duration>,
}

impl Default for GAParams {
    fn default() -> Self {
        Self {
            selection_rate: 0.5f64,
            generation_upper_bound: 200,
            population_size: 100,
            max_duration: None,
        }
    }
}

pub struct GAConfig<T: Chromosome> {
    pub params: GAParams,
    // pub ops: GAOps<S>,
    pub fitness_fn: FitnessFn<Individual<T>>,
    pub mutation_operator: Box<dyn MutationOperator<T>>,
    pub crossover_operator: Box<dyn CrossoverOperator<T>>,
    pub selection_operator: Box<dyn SelectionOperator<T>>,
    pub population_factory: Box<dyn PopulationGenerator<T>>,
    pub probe: Box<dyn Probe<T>>,
}

#[derive(Default)]
pub struct GAMetadata {
    start_time: Option<std::time::Instant>,
    duration: Option<std::time::Duration>,
    generation: Option<usize>,
}

impl GAMetadata {
    pub fn new() -> Self {
        GAMetadata {
            start_time: None,
            duration: None,
            generation: None,
        }
    }
}
pub struct GeneticAlgorithm<T: Chromosome> {
    config: GAConfig<T>,
    metadata: GAMetadata,
}

impl<T: Chromosome> GeneticAlgorithm<T> {
    pub fn new(config: GAConfig<T>) -> Self {
        GeneticAlgorithm {
            config,
            metadata: GAMetadata::new(),
        }
    }

    fn find_best_individual(population: &Vec<Individual<T>>) -> &Individual<T> {
        debug_assert!(!population.is_empty());
        let mut best_individual = &population[0];
        for idv in population.iter().skip(1) {
            if *idv > *best_individual {
                best_individual = idv;
            }
        }
        best_individual
    }

    fn evaluate_fitness_in_population(&self, population: &mut Vec<Individual<T>>) {
        for idv in population {
            let fitness = (self.config.fitness_fn)(idv);
            idv.fitness = fitness;
        }
    }

    pub fn run(&mut self) -> Option<Individual<T>> {
        self.metadata.start_time = Some(std::time::Instant::now());
        self.config.probe.on_start(&self.metadata);

        // 1. Create initial random population.
        let mut population = self
            .config
            .population_factory
            .generate(self.config.params.population_size);
        self.config.probe.on_initial_population_created(&population);

        // 2. Evaluate fitness for each individual.
        self.evaluate_fitness_in_population(&mut population);

        // 3. Store best individual.
        let mut best_individual_all_time =
            GeneticAlgorithm::find_best_individual(&population).clone();
        // self.config.probe.on_new_best(&self.metadata, best_individual);

        for generation_no in 1..=self.config.params.generation_upper_bound {
            self.metadata.generation = Some(generation_no);
            self.metadata.duration = Some(self.metadata.start_time.unwrap().elapsed());

            // 2. Evaluate fitness for each individual.
            self.evaluate_fitness_in_population(&mut population);

            // 4. Create mating pool by applying selection operator.
            let mating_pool: Vec<&Individual<T>> =
                self.config
                    .selection_operator
                    .apply(&self.metadata, &population, population.len());

            // 5. From mating pool create new generation (apply crossover & mutation).
            let mut children: Vec<Individual<T>> =
                Vec::with_capacity(self.config.params.population_size);

            // FIXME: Do not assume that population size is an even number.
            for i in (0..mating_pool.len()).step_by(2) {
                let crt_children = self
                    .config
                    .crossover_operator
                    .apply(mating_pool[i], mating_pool[i + 1]);

                children.push(crt_children.0);
                children.push(crt_children.1);
            }

            // 5.1 Here we should apply the mutations on children?
            (0..children.len()).for_each(|i| self.config.mutation_operator.apply(&mut children[i]));

            // TODO
            // 6. Replacement - merge new generation with old one
            // As for now I'm replacing old population with the new one, but this must be
            // reimplemented. See p. 58 Introduction to Genetic Algorithms.
            population = children;

            // 6. Check for stop condition (Is good enough individual found)? If not goto 2.
            self.evaluate_fitness_in_population(&mut population);

            let best_individual = GeneticAlgorithm::find_best_individual(&population);

            if *best_individual > best_individual_all_time {
                best_individual_all_time = best_individual.clone()
            }

            self.config
                .probe
                .on_new_best(&self.metadata, &best_individual_all_time);

            if let Some(duration) = self.config.params.max_duration {
                if self.metadata.start_time.unwrap().elapsed() >= duration {
                    break;
                }
            }
        }

        Some(best_individual_all_time)
    }
}
