use std::time::Instant;

use derive_more::Constructor;
use nalgebra::Vector2;

use crate::filters::{camera_object_filter::CameraObjectFilter, vision::robot::filtered_robot::FilteredRobot};

use super::{ball_observation::BallObservation, filtered_ball::FilteredBall};

#[derive(Constructor)]
pub struct CameraGroundBallPrediction {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub time: Instant,
}

pub struct CameraGroundBallPredictionObservationPair {
    pub prediction: CameraGroundBallPrediction,
    pub observation: Option<BallObservation>,
}

pub struct CameraGroundBallFilter {
    // ekf: GroundBallExtendedKalmanFilter,
    camera_object_filter: CameraObjectFilter,
}

/// Public methods for the CameraGroundBallFilter
impl CameraGroundBallFilter {
    pub fn new(observation: &BallObservation, velocity_estimate: Vector2<f32>) -> Self {
        todo!()
    }

    pub fn from_observation(observation: &BallObservation) -> Self {
        todo!()
    }

    pub fn get_estimate(&self, time: Instant) -> FilteredBall {
        todo!()
    }

    pub fn get_velocity_estimate(&self, time: Instant) -> Vector2<f32> {
        todo!()
    }

    pub fn predict(&self, time:Instant, robots_yellow: &Vec<FilteredRobot>, robots_blue: &Vec<FilteredRobot>) -> CameraGroundBallPrediction {
        todo!()
    }

    pub fn process_detections(&self, prediction_observation_pair: &CameraGroundBallPredictionObservationPair) -> bool {
        todo!()
    }

    pub fn get_health(&self) -> f32 {
        self.camera_object_filter.health
    }

    pub fn get_num_observations(&self) -> usize {
        self.camera_object_filter.frames_total
    }
}

/// Private methods for the CameraGroundBallFilter
impl CameraGroundBallFilter {
    fn check_robots(&self, robots: &Vec<FilteredRobot>, position_estimate: Vector2<f32>, velocity_estimate: Vector2<f32>) -> bool {
        todo!()
    }

    fn check_robot_collision(&self, robot: &FilteredRobot, position_estimate: Vector2<f32>, velocity_estimate: Vector2<f32>) -> bool {
        todo!()
    }

    fn predict_filter(&self, prediction: &CameraGroundBallPrediction) {
        todo!()
    }

    fn update(&self, observation: &BallObservation) {
        todo!()
    }

    fn update_not_seen(&self, time: Instant) -> bool {
        todo!()
    }
}