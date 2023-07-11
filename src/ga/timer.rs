pub(crate) struct Timer {
    start_time: std::time::Instant,
}

impl Timer {
    pub(crate) fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }

    #[inline(always)]
    pub(crate) fn start(&mut self) {
        self.start_time = std::time::Instant::now()
    }
    pub(crate) fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }
}
