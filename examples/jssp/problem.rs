pub mod fitness;
pub mod individual;
pub mod state;

use crate::util::print_slice;

#[derive(Debug, Clone)]
pub struct Operation {
    id: usize,
    finish_time: usize,
    duration: usize,
    machine: usize,

    // Should I hold references to other operations or just their ids
    preds: Vec<usize>,
    // direct_machine_pred: Option<usize>,
    // direct_machine_succ: Option<usize>,
    // direct_job_pred: Option<usize>,
    // direct_job_succ: Option<usize>,
}

impl Operation {
    pub fn new(id: usize, finish_time: usize, duration: usize, machine: usize, preds: Vec<usize>) -> Self {
        Self {
            id,
            finish_time,
            duration,
            machine,
            preds,
        }
    }
}

// impl std::fmt::Display for Operation {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

#[derive(Debug, Clone)]
pub struct Machine {
    id: usize,
    // dummy structure
    rmc: Vec<usize>,
}

impl Machine {
    pub fn is_idle(&self, range: std::ops::RangeInclusive<usize>) -> bool {
        assert!(*range.end() < 40);

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
        println!("Reserved {}..{} in machine {}: ", range.start, range.end, self.id);
        print_slice(&self.rmc);
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
