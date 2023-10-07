use std::ops::{Range, RangeInclusive};

use itertools::Itertools;
use log::{debug, info};

pub mod crossover;
pub mod fitness;
pub mod individual;
pub mod population;
pub mod probe;
pub mod replacement;

/// Describes relation between two operations
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum EdgeKind {
    /// Operation that the edge points to is from the same job (ops are on different machines)
    JobSucc,
    /// Operation that the edge points to is on the same machine (ops are from different jobs)
    MachineSucc,
}

/// Models the edge in neighbourhood graph where operations are nodes
#[derive(Debug, Clone, Copy)]
pub struct Edge {
    /// Unique id of the neighbour operation
    pub neigh_id: usize,
    /// Describes the relation between the operations
    pub kind: EdgeKind,
}

impl Edge {
    pub fn new(neigh_id: usize, kind: EdgeKind) -> Self {
        Self { neigh_id, kind }
    }
}

/// Models Operation that is a part of some job
///
/// TODO: Cleanup this struct. Move all data non-intrinsic to the Operation model
/// to separate structs
#[derive(Debug, Clone)]
pub struct Operation {
    /// Unique id of this operation
    id: usize,
    /// Duration of the operation
    duration: usize,
    /// Machine this operation is assigned to
    machine: usize,
    /// Finish time tick of this operation as determined by the solver. The value of this field
    /// is modified during the algorithm run
    finish_time: Option<usize>,
    /// Ids of all ops that this op depends on. TODO: Was the order guaranteed?
    preds: Vec<usize>,
    /// Edges describing relations to other ops in neighbourhood graph. It contains *at most* two elements
    /// as each op might have at most two successors: next operation in the job or next operation on the same machine
    /// this op is executed on. The value of this field is modified as the algorithm runs
    edges_out: Vec<Edge>,
    /// Operation id of direct machine predecessor of this op. This might be `None` in following scenarios:
    /// 1. Op is the first op on particular machine TODO: I'm not sure now, whether I set op no. 0 as machine predecessor
    /// of every first op on given machine or not, so please verify it before using this fact.
    /// 2. This is op with id 0
    ///
    /// The value of this field is modified as the algorithm runs.
    machine_pred: Option<usize>,
    /// If this operation lies on critical path in neighbourhood graph (as defined in paper by Nowicki & Smutnicki)
    /// this is the edge pointing to next op on critical path, if there is one - this might be the last operation
    /// or simply not on the path. The value of this field is modified as the algorithm runs.
    critical_path_edge: Option<Edge>,
    /// If this operation lies on critical path this field is used by the local search algorithm to store
    /// distance from this op to the sink node. The value of this field is modified as the algorithm runs.
    critical_distance: usize,
}

impl Operation {
    pub fn new(
        id: usize,
        duration: usize,
        machine: usize,
        finish_time: Option<usize>,
        preds: Vec<usize>,
    ) -> Self {
        Self {
            id,
            duration,
            machine,
            finish_time,
            preds,
            edges_out: Vec::new(),
            machine_pred: None,
            critical_path_edge: None,
            critical_distance: usize::MIN,
        }
    }

    pub fn reset(&mut self) {
        self.finish_time = None;
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

    /// Removes all ranges from the machine state
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
