use std::time::Duration;

use crate::ga::{individual::IndividualTrait, GAMetadata};

use super::ProbingPolicy;

pub struct GenerationInterval {
    interval: usize,
    threshold: usize,
    should_log: bool,
}

impl GenerationInterval {
    /// Returns new instance of [GenerationInverval] policy
    ///
    /// ### Arguments
    ///
    /// * `interval` - how many iteration should be skipped between logs
    /// * `first_threshold` - number of first iteration to log
    pub fn new(interval: usize, first_threshold: usize) -> Self {
        Self {
            interval,
            threshold: first_threshold,
            should_log: false,
        }
    }
}

impl<IndividualT: IndividualTrait> ProbingPolicy<IndividualT> for GenerationInterval {
    #[inline(always)]
    fn on_start(&mut self, _metadata: &crate::ga::GAMetadata) -> bool {
        true
    }

    #[inline(always)]
    fn on_initial_population_created(&mut self, _metadata: &GAMetadata, _population: &[IndividualT]) -> bool {
        true
    }

    #[inline(always)]
    fn on_new_best(&mut self, _metadata: &crate::ga::GAMetadata, _individual: &IndividualT) -> bool {
        true
    }

    #[inline(always)]
    fn on_new_generation(&mut self, _metadata: &GAMetadata, _generation: &[IndividualT]) -> bool {
        self.should_log
    }

    #[inline(always)]
    fn on_best_fit_in_generation(
        &mut self,
        _metadata: &crate::ga::GAMetadata,
        _individual: &IndividualT,
    ) -> bool {
        self.should_log
    }

    #[inline]
    fn on_iteration_start(&mut self, metadata: &GAMetadata) -> bool {
        if metadata.generation >= self.threshold {
            self.threshold += self.interval;
            self.should_log = true;
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn on_iteration_end(&mut self, _metadata: &GAMetadata) -> bool {
        let prev = self.should_log;
        self.should_log = false;
        prev
    }

    #[inline(always)]
    fn on_end(
        &mut self,
        _metadata: &crate::ga::GAMetadata,
        _population: &[IndividualT],
        _best_individual: &IndividualT,
    ) -> bool {
        true
    }
}

pub struct ElapsedTime {
    interval: Duration,
    threshold: Duration,
    should_log: bool,
}

impl ElapsedTime {
    /// Returns a new instance of [ElapsedTime] policy
    ///
    /// ### Arguments
    ///
    /// * `interval` - time between logging iterations
    /// * `threshold` - time of first logging iteration
    pub fn new(interval: Duration, threshold: Duration) -> Self {
        Self {
            interval,
            threshold,
            should_log: false,
        }
    }
}

impl<IndividualT: IndividualTrait> ProbingPolicy<IndividualT> for ElapsedTime {
    #[inline(always)]
    fn on_start(&mut self, _metadata: &crate::ga::GAMetadata) -> bool {
        true
    }

    #[inline(always)]
    fn on_initial_population_created(&mut self, _metadata: &GAMetadata, _population: &[IndividualT]) -> bool {
        true
    }

    #[inline(always)]
    fn on_new_best(&mut self, _metadata: &crate::ga::GAMetadata, _individual: &IndividualT) -> bool {
        true
    }

    #[inline(always)]
    fn on_new_generation(&mut self, _metadata: &GAMetadata, _generation: &[IndividualT]) -> bool {
        self.should_log
    }

    #[inline(always)]
    fn on_best_fit_in_generation(
        &mut self,
        _metadata: &crate::ga::GAMetadata,
        _individual: &IndividualT,
    ) -> bool {
        self.should_log
    }

    fn on_iteration_start(&mut self, metadata: &GAMetadata) -> bool {
        if metadata.duration.unwrap() >= self.threshold {
            self.should_log = true;
            self.threshold += self.interval;
            true
        } else {
            false
        }
    }

    #[inline]
    fn on_iteration_end(&mut self, _metadata: &GAMetadata) -> bool {
        let prev = self.should_log;
        self.should_log = false;
        prev
    }

    #[inline(always)]
    fn on_end(
        &mut self,
        _metadata: &crate::ga::GAMetadata,
        _population: &[IndividualT],
        _best_individual: &IndividualT,
    ) -> bool {
        true
    }
}
