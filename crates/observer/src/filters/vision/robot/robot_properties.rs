#![allow(dead_code)]

use derive_more::{Add, Constructor, Div};
use nalgebra::Vector2;
use std::time::Instant;

#[derive(Default, Constructor)]
pub struct RobotPosition {
    position: Vector2<f32>,
    orientation: f32,
}

#[derive(Default, Constructor, Add, Div)]
pub struct RobotVelocity {
    velocity: Vector2<f32>,
    angular_velocity: f32,
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
    id: u8,
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
    pub fn new(
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

    pub fn get_time_captured(&self) -> Instant {
        self.time_captured
    }

    pub fn get_position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn get_orientation(&self) -> f32 {
        self.orientation
    }

    pub fn get_robot_id(&self) -> RobotId {
        self.id
    }

    pub fn get_team_color(&self) -> TeamColor {
        self.team
    }

    pub fn get_camera_id(&self) -> u32 {
        self.camera_id
    }
}
