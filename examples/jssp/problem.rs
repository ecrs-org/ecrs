use std::ops::{Range, RangeInclusive};

use log::info;

pub mod crossover;
pub mod fitness;
pub mod individual;
pub mod population;

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

#[derive(Debug, Clone)]
pub struct Operation {
    id: usize,
    finish_time: usize,
    duration: usize,
    machine: usize,

    // Should I hold references to other operations or just their ids
    preds: Vec<usize>,
    edges_out: Vec<Edge>,
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
            critical_path_edge: None,
            critical_distance: usize::MIN,
        }
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
}

impl Machine {
    pub fn new(id: usize, _rmc_capacity: usize) -> Self {
        Self {
            id,
            // rmc: vec![1; rmc_capacity],
            rmc: Vec::new(),
        }
    }
}

// Naive implementation
// impl Machine {
//     pub fn is_idle(&self, range: std::ops::RangeInclusive<usize>) -> bool {
//         for i in range {
//             if self.rmc[i] == 0 {
//                 return false;
//             }
//         }
//         true
//     }
//
//     pub fn reserve(&mut self, range: std::ops::Range<usize>) {
//         for i in range {
//             self.rmc[i] = 0;
//         }
//     }
//
//     pub fn reset(&mut self) {
//         self.rmc.fill(1);
//     }
// }

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
        if let Some(_) = self
            .rmc
            .iter()
            .find(|busy_range| busy_range.start < *range.end() && busy_range.end > *range.start())
        {
            false
        } else {
            true
        }
    }

    /// DOES NOT PERFORM VALIDATION!
    pub fn reserve(&mut self, range: std::ops::Range<usize>) {
        self.rmc.push(range);
    }

    pub fn reset(&mut self) {
        self.rmc.clear();
    }
}

#[derive(Debug)]
pub struct JsspConfig {
    pub n_jobs: usize,
    pub n_machines: usize,
}

#[derive(Debug)]
pub struct JsspInstanceMetadata {
    pub name: String,
}

#[derive(Debug)]
pub struct JsspInstance {
    pub jobs: Vec<Vec<Operation>>,
    pub cfg: JsspConfig,
    pub metadata: JsspInstanceMetadata,
}
