use std::path::PathBuf;

use ecrs::prelude::population::{self, PopulationGenerator};
use itertools::Itertools;
use push_trait::PushFront;

use crate::parse::JsspInstanceLoadingError;

use super::{individual::JsspIndividual, Edge, EdgeKind, JsspInstance, Machine, Operation};

pub struct JsspPopProvider {
    instance: JsspInstance,
    operations: Vec<Operation>,
}

impl JsspPopProvider {
    pub fn new(mut instance: JsspInstance) -> Self {
        // Finding dimension of the chromosome -- total number of operations (later multiplied)
        let dim: usize = instance.jobs.iter().map(|job| job.len()).sum();

        // Prepare mock operations
        // TODO: Shouldn't the duration be set to 0?
        let mut zero_op = Operation::new(0, usize::MAX, 0, None, Vec::new());
        let sink_op = Operation::new(dim + 1, usize::MAX, 0, None, Vec::from_iter(0..=dim));

        // Shift all ids by 1 && and job 0 & n + 1
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

        assert_eq!(operations.len(), dim + 2);

        Self { instance, operations }
    }
}

impl PopulationGenerator<JsspIndividual> for JsspPopProvider {
    fn generate(&mut self, count: usize) -> Vec<JsspIndividual> {
        population::tools::PointGenerator::new()
            .generate_with_single_constraint(2 * (self.operations.len() - 2), count, 0.0..1.0)
            .into_iter()
            .map(|chromosome| {
                JsspIndividual::new(
                    chromosome,
                    self.operations.clone(),
                    Vec::from_iter((0..self.instance.cfg.n_machines).map(Machine::new)),
                    usize::MAX,
                )
            })
            .collect()
    }
}

impl TryFrom<PathBuf> for JsspPopProvider {
    type Error = JsspInstanceLoadingError;

    fn try_from(file: PathBuf) -> Result<Self, Self::Error> {
        assert!(file.is_file(), "Received path does not point to a file!");
        let instance = JsspInstance::try_from(&file)?;
        Ok(JsspPopProvider::new(instance))
    }
}
