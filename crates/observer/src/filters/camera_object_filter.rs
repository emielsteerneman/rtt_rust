use std::time::Instant;

pub struct CameraObjectFilter {
    frames_total: u32,
    frames_not_seen_for: u32,
    time_last_seen: Instant,
    time_last_update: Instant,
    health: f32,

    INCREMENT: f32,
    DECREMENT_SLOPE: f32,
    HEALTHY_LIMIT: f32,
}

impl CameraObjectFilter {
    const MAXIMUM: f32 = 100.;

    pub fn new(
        full_health_to_unhealty_time: f32,
        tick_rate: f32,
        full_health_ticks: f32,
        is_healthy_after: f32,
        time: Instant,
    ) -> Self {
        let healthy_limit: f32 = Self::MAXIMUM * is_healthy_after / full_health_ticks;
        let decrement_slope: f32 = Self::MAXIMUM * (1. - is_healthy_after / full_health_ticks)
            / full_health_to_unhealty_time;
        let increment: f32 = Self::MAXIMUM / full_health_ticks + tick_rate * decrement_slope;

        Self {
            frames_total: 1,
            frames_not_seen_for: 0,
            time_last_seen: time,
            time_last_update: time,
            HEALTHY_LIMIT: healthy_limit,
            DECREMENT_SLOPE: decrement_slope,
            INCREMENT: increment,
            health: increment,
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.HEALTHY_LIMIT <= self.health
    }

    pub fn object_seen(&mut self, time: Instant) {
        if time < self.time_last_seen {
            return;
        }
        let new_health: f32 = self.health + self.INCREMENT
            - (time - self.time_last_update).as_secs_f32() * self.DECREMENT_SLOPE;
        self.health = f32::clamp(new_health, 0., Self::MAXIMUM);
        self.frames_total += 1;
        self.time_last_seen = time;
        self.time_last_update = time;
        self.frames_not_seen_for = 0;
    }

    pub fn object_invisible(&mut self, time: Instant) {
        if time < self.time_last_seen {
            return;
        }
        self.health = f32::max(
            0.,
            self.health - (time - self.time_last_update).as_secs_f32() * self.DECREMENT_SLOPE,
        );
    }
}

/* Getters and Setters */
impl CameraObjectFilter {
    pub fn get_frames_total(&self) -> u32 {
        self.frames_total
    }
    pub fn get_frames_not_seen_for(&self) -> u32 {
        self.frames_not_seen_for
    }
    pub fn get_time_last_seen(&self) -> Instant {
        self.time_last_seen
    }
    pub fn get_time_last_update(&self) -> Instant {
        self.time_last_update
    }
    pub fn get_health(&self) -> f32 {
        self.health
    }
}
