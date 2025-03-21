use std::time::Instant;
use derive_more::Constructor;
use nalgebra::Vector2;
use protos::sslvision::SslDetectionBall;

#[derive(Constructor)]
pub struct BallObservation {
    camera_id: usize,
    time_captured: Instant,
    time_sent: Instant,
    position: Vector2<f32>,
    pixel_position: Vector2<f32>,
    area: u32,
    confidence: f32,
    height: f32,
}

impl BallObservation {
    pub fn from_observation(camera_id: usize, time_captured: Instant, time_sent: Instant, observation: &SslDetectionBall) -> Self {
        todo!()
    }
}