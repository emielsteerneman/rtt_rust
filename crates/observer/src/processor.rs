pub struct Processor {
    hz: f32,
}

impl Processor {
    pub fn new(hz:f32) -> Self {
        Processor { hz }
    }

    pub fn run(self) {
        let cycle_duration = std::time::Duration::from_secs_f32(1.0 / self.hz);
        let mut time_next_cycle = std::time::Instant::now();

        loop {
            // do some work..
            tracing::info!("Tick!");

            // sleep till next cycle
            let time_to_sleep = time_next_cycle - std::time::Instant::now();
            time_next_cycle += cycle_duration;
            
            // Check if sleep time is negative
            if 0 < time_to_sleep.as_micros() {
                std::thread::sleep(time_to_sleep);
            } else {
                tracing::warn!("Processor is running behind! Can't handle {} Hz", self.hz);
            }

        }
    }
}

impl Default for Processor {
    fn default() -> Self {
        Processor::new(60.0)
    }
}