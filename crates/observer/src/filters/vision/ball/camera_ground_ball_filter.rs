use std::time::Instant;

use derive_more::Constructor;
use nalgebra::Vector2;

use super::ball_observation::BallObservation;

#[derive(Constructor)]
struct CameraGroundBallPrediction {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    time: Instant,
}

struct CameraGroundBallPredictionObservationPair {
    prediction: CameraGroundBallPrediction,
    observation: Option<BallObservation>,
}

pub struct CameraGroundBallFilter {
    ekf: GroundBallExtendedKalmanFilter,
}

/// Public methods for the CameraGroundBallFilter
impl CameraGroundBallFilter {
    pub fn new(observation: &BallObservation, velocity_estimate: Vector2<f32>) -> Self {
        todo!()
    }

    pub fn from_observation(observation: &BallObservation) -> Self {
        todo!()
    }

    pub fn get_estimate(time: Instant) -> FilteredBall {
        todo!()
    }

    pub fn get_velocity_estimate(time: Instant) -> Vector2<f32> {
        todo!()
    }

    pub fn predict(time:Instant, robots_yellow: &Vec<FilteredRobot>, robots_blue: &Vec<FilteredRobot>) {
        todo!()
    }

    pub fn process_detections(prediction_observation_pair: &CameraGroundBallPredictionObservationPair) {
        todo!()
    }
}

/// Private methods for the CameraGroundBallFilter
impl CameraGroundBallFilter {
    fn check_robots(robots: &Vec<FilteredRobot>, position_estimate: Vector2<f32>, velocity_estimate: Vector2<f32>) -> bool {
        todo!()
    }

    fn check_robot_collision(robot: &FilteredRobot, position_estimate: Vector2<f32>, velocity_estimate: Vector2<f32>) -> bool {
        todo!()
    }

    fn predict_filter(prediction: &CameraGroundBallPrediction) {
        todo!()
    }

    fn update(observation: &BallObservation) {
        todo!()
    }

    fn update_not_seen(time: Instant) -> bool {
        todo!()
    }
}