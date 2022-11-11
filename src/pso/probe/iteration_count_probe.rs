use crate::pso::probe::probe::Probe;
use crate::pso::swarm::Swarm;

pub struct IterationCountProbe{
    probe: Box<dyn Probe>,
    last_log_generation: usize,
    log_interval: usize,
    total_generations: usize
}

impl IterationCountProbe {
    pub fn new(probe: Box<dyn Probe>, log_interval: usize, total_generations: usize, ) -> IterationCountProbe {
        IterationCountProbe {
            probe,
            last_log_generation: 0,
            log_interval,
            total_generations
        }
    }
}

impl Probe for IterationCountProbe {
    fn on_begin(&mut self, swarm: &Swarm) {
        self.probe.on_begin(swarm);
    }

    fn on_end(&mut self, swarm: &Swarm) {
        if self.total_generations > self.last_log_generation as usize {
            self.on_new_generation(swarm, self.total_generations);
        }
        self.probe.on_end(swarm);
    }

    fn on_new_generation(&mut self, swarm: &Swarm, generation: usize) {
        if generation % self.log_interval == 0 {
            self.probe.on_new_generation(swarm, generation);
            self.last_log_generation = generation;
        }
    }
}
