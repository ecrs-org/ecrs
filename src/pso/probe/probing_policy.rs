use std::time::Instant;
use crate::pso::swarm::Swarm;
use super::ProbingPolicy;

pub struct GenerationInterval {
    last_log_generation: usize,
    log_interval: usize,
}

impl GenerationInterval {
    pub fn new(log_interval: usize) -> GenerationInterval {
        GenerationInterval {
            last_log_generation: 0,
            log_interval,
        }
    }
}

impl ProbingPolicy for GenerationInterval {
    fn on_begin(&mut self) -> bool {
        true
    }

    fn on_end(&mut self, generation: usize) -> bool {
        generation > self.last_log_generation
    }

    fn on_new_generation(&mut self, generation: usize) -> bool {
        if generation % self.log_interval == 0 {
            self.last_log_generation = generation;
            true
        }
        else {
            false
        }
    }
}


pub struct ElapsedTime {
    last_log_time: Instant,
    log_interval: usize,
    last_log_generation: usize,
}

impl ElapsedTime {
    pub fn new(log_interval: usize) -> ElapsedTime {
        ElapsedTime {
            last_log_time: Instant::now(),
            log_interval,
            last_log_generation: 0,
        }
    }
}

impl ProbingPolicy for ElapsedTime {
    fn on_begin(&mut self) -> bool {
        self.last_log_time = Instant::now();
        true
    }

    fn on_end(&mut self, generation: usize) -> bool {
        generation > self.last_log_generation
    }

    fn on_new_generation(&mut self, generation: usize) -> bool {
        if self.last_log_time.elapsed().as_secs() >= self.log_interval as u64 {
            self.last_log_time = Instant::now();
            self.last_log_generation = generation;
            true
        }
        else {
            false
        }
    }
}