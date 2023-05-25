use std::path::PathBuf;

use ecrs::prelude::population::{self, PopulationGenerator};
use itertools::Itertools;
use push_trait::PushFront;

use super::{individual::JsspIndividual, Edge, EdgeKind, JsspInstance, Machine, Operation};

pub struct JsspPopProvider {
    path: PathBuf,
}

impl JsspPopProvider {
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_file());
        Self { path }
    }
}

impl PopulationGenerator<JsspIndividual> for JsspPopProvider {
    fn generate(&mut self, count: usize) -> Vec<JsspIndividual> {
        let instance = JsspInstance::try_from(self.path.clone());

        let Ok(mut instance) = instance else {
            panic!("Failed to load problem instance from file {:?}", self.path);
        };

        // Finding dimension of the chromosome
        let mut point_gen = population::tools::PointGenerator::new();
        let dim: usize = instance.jobs.iter().map(|job| job.len()).sum();

        // Shift all ids by 1 && and job 0 & n + 1
        let mut zero_op = Operation::new(0, usize::MAX, 0, 0, Vec::new());
        let sink_op = Operation::new(dim + 1, usize::MAX, 0, 0, Vec::from_iter(0..=dim));
        instance.jobs.iter_mut().for_each(|job| {
            job.iter_mut().for_each(|op| {
                op.id += 1;
                op.preds.iter_mut().for_each(|pred_id| *pred_id += 1);
                op.preds.push(0);
                op.edges_out.push(Edge {
                    neigh_id: op.id + 1,
                    kind: EdgeKind::JobSucc,
                })
            });
            job.last_mut().unwrap().edges_out.last_mut().unwrap().neigh_id = dim + 1;
            zero_op.edges_out.push(Edge {
                neigh_id: job.first().unwrap().id,
                kind: EdgeKind::JobSucc,
            });
        });

        let operations = [zero_op]
            .into_iter()
            .chain(instance.jobs.clone().into_iter().flatten())
            .chain([sink_op])
            .collect_vec();

        point_gen
            .generate_with_single_constraint(2 * dim, count, 0.0..1.0)
            .into_iter()
            .map(|chromosome| {
                JsspIndividual::new(
                    chromosome,
                    operations.clone(),
                    Vec::from_iter((0..instance.cfg.n_machines).map(|i| Machine::new(i, 50))),
                    usize::MAX,
                )
            })
            .collect()
    }
}
