use ecrs::{
    ga::{self, individual::RealValueIndividual, Individual},
    prelude::population::PopulationGenerator,
};

use super::{individual::JsspIndividual, JsspConfig, JsspInstance, Machine, Operation};

#[derive(Debug)]
pub struct JsspState {
    pub cfg: JsspConfig,
    pub population: Vec<JsspIndividual>,
}

impl JsspState {
    fn build_operations(&self) -> Vec<Operation> {
        // For now I'm implementing just for single case
        assert!(self.cfg.n_jobs == 4 && self.cfg.n_machines == 2);

        let mut operations = Vec::with_capacity(self.cfg.n_jobs + 2);
        operations.push(Operation {
            id: 0,
            finish_time: usize::MAX,
            duration: 0,
            machine: 0,
            preds: Vec::new(),
            // direct_machine_pred: None,
            // direct_job_succ: None,
            // direct_job_pred: None,
            // direct_job_succ:
        });

        operations.push(Operation {
            id: 1,
            finish_time: usize::MAX,
            duration: 4,
            machine: 1,
            // preds: Vec::new(), // or maybe should I put Operation 0 here?
            preds: vec![0],
        });

        operations.push(Operation {
            id: 2,
            finish_time: usize::MAX,
            duration: 2,
            machine: 0,
            preds: vec![0, 1],
        });

        operations.push(Operation {
            id: 3,
            finish_time: usize::MAX,
            duration: 1,
            machine: 0,
            // preds: Vec::new(),
            preds: vec![0],
        });

        operations.push(Operation {
            id: 4,
            finish_time: usize::MAX,
            duration: 3,
            machine: 1,
            preds: vec![0, 3],
        });

        operations.push(Operation {
            id: 5,
            finish_time: usize::MAX,
            duration: 0,
            machine: 0,
            preds: vec![0, 1, 2, 3, 4],
        });

        operations
    }

    fn build_machines(&self) -> Vec<Machine> {
        // To represent machine we could think of some sparse structure, but for now let it be
        // vector

        let machines = vec![
            Machine {
                id: 0,
                rmc: vec![1; 40],
            },
            Machine {
                id: 1,
                rmc: vec![1; 40],
            },
        ];

        machines
    }

    pub fn init_pop(&mut self, size: usize) {
        self.population = ga::population::RandomPoints::with_single_constraint(8, 0.0..1.0)
            .generate(size)
            .into_iter()
            .map(|idv: RealValueIndividual| {
                JsspIndividual::new(
                    idv.chromosome,
                    self.build_operations(),
                    self.build_machines(),
                    usize::MAX,
                )
            })
            .collect();

        // self.population = vec![JsspIndividual {
        //     chromosome: vec![0.20, 0.22, 0.25, 0.90, 0.14, 0.24, 0.25, 0.70],
        //     operations: self.build_operations(),
        //     fitness: usize::MAX,
        //     machines: self.build_machines(),
        // }; size];
    }

    pub fn inject_ecrs_pop(&mut self, population: Vec<Individual<Vec<f64>>>) {
        self.population = population
            .into_iter()
            .map(|idv: RealValueIndividual| {
                JsspIndividual::new(
                    idv.chromosome,
                    self.build_operations(),
                    self.build_machines(),
                    idv.fitness as usize,
                )
            })
            .collect();
    }

    pub fn eval_pop(&mut self) -> usize {
        self.population.iter_mut().map(|idv| idv.eval()).min().unwrap()
    }
}

impl From<JsspInstance> for JsspState {
    fn from(instance: JsspInstance) -> Self {
        todo!()
    }
}
