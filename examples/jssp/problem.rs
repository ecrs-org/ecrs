pub mod fitness;
pub mod individual;
pub mod state;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeKind {
    JobSucc,
    MachineSucc,
}

#[derive(Debug, Clone)]
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
    critical_distance: Option<usize>,
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
            critical_distance: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Machine {
    id: usize,
    // dummy structure
    rmc: Vec<usize>,
}

impl Machine {
    pub fn is_idle(&self, range: std::ops::RangeInclusive<usize>) -> bool {
        for i in range {
            if self.rmc[i] == 0 {
                return false;
            }
        }
        true
    }

    pub fn reserve(&mut self, range: std::ops::Range<usize>) {
        for i in range.clone() {
            self.rmc[i] = 0;
        }
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
    pub ops: Vec<Operation>,
    pub cfg: JsspConfig,
    pub metadata: JsspInstanceMetadata,
}
