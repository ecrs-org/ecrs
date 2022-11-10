use std::time::{Duration, Instant};

use crate::pso::probe::probe::Probe;
use crate::pso::swarm::Swarm;

pub struct TimedProbe{
    probe: Box<dyn Probe>,
    last_log_time: Instant,
    log_interval: usize,
    last_log_generation: usize,
    total_generations: usize
}

impl TimedProbe {
    pub fn new(probe: Box<dyn Probe>, log_interval: usize, total_generations: usize) -> TimedProbe {
        TimedProbe {
            probe,
            last_log_time: Instant::now(),
            log_interval,
            last_log_generation: 0,
            total_generations
        }
    }
}

impl Probe for TimedProbe {
    fn on_begin(&mut self, swarm: &Swarm) {
        self.last_log_time = Instant::now();
        self.probe.on_begin(swarm);
    }

    fn on_end(&mut self, swarm: &Swarm) {
        if self.total_generations > self.last_log_generation {
            self.on_new_generation(swarm, self.total_generations);
        }
        self.probe.on_end(swarm);
    }

    fn on_new_generation(&mut self, swarm: &Swarm, generation: usize) {
        if self.last_log_time.elapsed().as_secs() >= self.log_interval as u64 {
            self.probe.on_new_generation(swarm, generation);
            self.last_log_time = Instant::now();
            self.last_log_generation = generation;
        }
    }
}