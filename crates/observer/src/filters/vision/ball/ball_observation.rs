use std::time::Instant;
use derive_more::Constructor;
use nalgebra::Vector2;
use protos::sslvision::SslDetectionBall;

#[derive(Constructor)]
pub struct BallObservation {
    pub camera_id: usize,
    pub time_captured: Instant,
    pub time_sent: Instant,
    pub position: Vector2<f32>,
    pub pixel_position: Vector2<f32>,
    pub area: u32,
    pub confidence: f32,
    pub height: f32,
}

impl BallObservation {
    pub fn from_observation(camera_id: usize, time_captured: Instant, time_sent: Instant, observation: &SslDetectionBall) -> Self {
        todo!()
    }
}