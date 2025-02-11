use derive_more::{Add, Constructor, Div};
use nalgebra::Vector2;
use std::time::Instant;

#[derive(Default, Constructor)]
struct RobotPosition {
    position: Vector2<f32>,
    orientation: f32,
}

#[derive(Default, Constructor, Add, Div)]
struct RobotVelocity {
    velocity: Vector2<f32>,
    angular_velocity: f32,
}

#[derive(Default, Constructor, PartialOrd, PartialEq)]
struct RobotId {
    id: u8,
}

impl RobotId {
    fn is_valid(&self) -> bool {
        self.id < 16
    }
}

enum TeamColor {
    BLUE,
    YELLOW,
}

pub struct RobotObservation {
    camera_id: u32,
    time_captured: Instant,
    time_sent: Instant,
    id: RobotId,
    team: TeamColor,
    position: Vector2<f32>,
    pixel_position: Vector2<f32>,
    orientation: f32,
    confidence: f32,
    height: f32,
}

impl RobotObservation {
    fn new(
        camera_id: u32,
        time_captured: Instant,
        time_sent: Instant,
        team_color: TeamColor,
        detection: protos::messages::SslDetectionRobot,
        
    ) -> Self {
        
        Self {
            camera_id,
            time_captured,
            time_sent,
            id: RobotId::new(detection.robot_id.unwrap() as u8),
            team: team_color,
            position: Vector2::new(detection.x / 1000., detection.y / 1000.),
            pixel_position: Vector2::new(detection.pixel_x, detection.pixel_y),
            orientation: detection.orientation.unwrap(),
            confidence: detection.confidence,
            height: detection.height.unwrap(),
        }
    }

}