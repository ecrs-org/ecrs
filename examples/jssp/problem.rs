use std::ops::{Range, RangeInclusive};

use itertools::Itertools;
use log::{debug, info};

pub mod crossover;
pub mod fitness;
pub mod individual;
pub mod population;
pub mod probe;
pub mod replacement;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum EdgeKind {
    JobSucc,
    MachineSucc,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub neigh_id: usize,
    pub kind: EdgeKind,
}

impl Edge {
    pub fn new(neigh_id: usize, kind: EdgeKind) -> Self {
        Self { neigh_id, kind }
    }
}

#[derive(Debug, Clone)]
pub struct Operation {
    id: usize,
    finish_time: usize,
    duration: usize,
    machine: usize,

    preds: Vec<usize>,
    edges_out: Vec<Edge>,
    machine_pred: Option<usize>,
    critical_path_edge: Option<Edge>,
    critical_distance: usize,
}

impl Operation {
    pub fn new(id: usize, finish_time: usize, duration: usize, machine: usize, preds: Vec<usize>) -> Self {
        Self {
            id,
            finish_time,
            duration,
            machine,
            preds,
            edges_out: Vec::new(),
            machine_pred: None,
            critical_path_edge: None,
            critical_distance: usize::MIN,
        }
    }

    pub fn reset(&mut self) {
        self.finish_time = usize::MAX;
        self.machine_pred = None;
        if let Some(edge_to_rm) = self
            .edges_out
            .iter()
            .find_position(|edge| edge.kind == EdgeKind::MachineSucc)
        {
            self.edges_out.swap_remove(edge_to_rm.0);
        }
        debug_assert_eq!(
            self.edges_out
                .iter()
                .filter(|e| e.kind == EdgeKind::MachineSucc)
                .count(),
            0
        );
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Machine {
    id: usize,

    // For naive implementation
    // rmc: Vec<usize>,

    // For "possibly better implementation"
    rmc: Vec<Range<usize>>,
    pub last_scheduled_op: Option<usize>,
}

impl Machine {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            // rmc: vec![1; rmc_capacity],
            rmc: Vec::new(),
            last_scheduled_op: None,
        }
    }
}

// Possibly better implementation
// Best one should be balanced interval BST (e.g. BTreeMap) with simple interval intersection
// finding algorithm.
// Unfortunately the API that would allow implementation of such algorithm is not stabilized yet:
// https://github.com/rust-lang/libs-team/issues/141
// Example implementation: https://github.com/Amanieu/rangetree
//
// Here we use just a vector of intervals. This is most likely slower that naive solution, but it
// does not require so much memory.
impl Machine {
    pub fn is_idle(&self, range: std::ops::RangeInclusive<usize>) -> bool {
        !self
            .rmc
            .iter()
            .any(|busy_range| busy_range.start < *range.end() && busy_range.end > *range.start())
    }

    /// DOES NOT PERFORM VALIDATION!
    pub fn reserve(&mut self, range: std::ops::Range<usize>, op: usize) {
        self.rmc.push(range);
        self.last_scheduled_op = Some(op);
    }

    pub fn reset(&mut self) {
        self.rmc.clear();
        self.last_scheduled_op = None;
    }
}

/// Basic information (metadata) about the jssp instance.
#[derive(Debug, Clone)]
pub struct JsspConfig {
    /// Number of
    pub n_jobs: usize,
    pub n_machines: usize,
    pub n_ops: usize,
}

#[derive(Debug, Clone)]
pub struct JsspInstanceMetadata {
    /// Name of the instance. In case the instance was loaded from the disk,
    /// the `name` should be related to the data file name.
    pub name: String,
}

/// Describes single JSSP problem instance.
/// Instance is modeled as a set of jobs.
/// Each job is modeled as a set of operations.
/// Operations have precedency relation estabilished
/// and each operation is assigned to a particular machine.
#[derive(Debug, Clone)]
pub struct JsspInstance {
    pub jobs: Vec<Vec<Operation>>,
    pub cfg: JsspConfig,
    pub metadata: JsspInstanceMetadata,
}
