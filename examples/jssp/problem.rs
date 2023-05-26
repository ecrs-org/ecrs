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
    // dummy structure
    rmc: Vec<usize>,
}

impl Machine {
    pub fn new(id: usize, rmc_capacity: usize) -> Self {
        Self {
            id,
            rmc: vec![1; rmc_capacity],
        }
    }
    pub fn is_idle(&self, range: std::ops::RangeInclusive<usize>) -> bool {
        for i in range {
            if self.rmc[i] == 0 {
                return false;
            }
        }
        true
    }

    pub fn reserve(&mut self, range: std::ops::Range<usize>) {
        for i in range {
            self.rmc[i] = 0;
        }
    }

    pub fn reset(&mut self) {
        self.rmc.fill(1);
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
