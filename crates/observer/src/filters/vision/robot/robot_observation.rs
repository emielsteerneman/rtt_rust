#![allow(dead_code)]

use derive_more::{Add, Constructor, Div};
use nalgebra::Vector2;
use std::time::Instant;

#[derive(Default, Constructor)]
pub struct RobotPosition {  
    pub position: Vector2<f32>,
    pub orientation: f32,
}

#[derive(Default, Constructor, Add, Div)]
pub struct RobotVelocity {
    pub velocity: Vector2<f32>,
    pub angular_velocity: f32,
}

impl RobotVelocity {
    pub fn get_velocity(&self) -> &Vector2<f32> {
        &self.velocity
    }

    pub fn get_angular_velocity(&self) -> f32 {
        self.angular_velocity
    }
}

#[derive(Default, Constructor, PartialOrd, PartialEq, Clone, Copy)]
pub struct RobotId {
    pub id: u8,
}

impl RobotId {
    pub fn is_valid(&self) -> bool {
        self.id < 16
    }
}

#[derive(Clone, Copy)]
pub enum TeamColor {
    BLUE,
    YELLOW,
}

pub struct RobotObservation {
    pub camera_id: u32,
    pub time_captured: Instant,
    pub time_sent: Instant,
    pub id: RobotId,
    pub team: TeamColor,
    pub position: Vector2<f32>, // Why does this not use the RobotPosition struct?
    pub pixel_position: Vector2<f32>,
    pub orientation: f32,
    pub confidence: f32,
    pub height: f32,
}

impl RobotObservation {
    pub fn new(
        camera_id: u32,
        time_captured: Instant,
        time_sent: Instant,
        team_color: TeamColor,
        detection: protos::sslvision::SslDetectionRobot,
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
