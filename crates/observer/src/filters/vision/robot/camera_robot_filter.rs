#![allow(dead_code)]

use std::time::Instant;

use super::{
    robot_orientation_filter::RobotOrientationFilter,
    robot_observation::{RobotId, RobotObservation, RobotPosition, RobotVelocity, TeamColor},
};
use crate::filters::camera_object_filter::CameraObjectFilter;
use common::pos_vel_filter_2d::PosVelFilter2D;
use nalgebra::{Matrix2, Matrix4, Vector2, Vector4};

pub struct CameraRobotFilter {
    filter: CameraObjectFilter,
    position_filter: PosVelFilter2D,
    orientation_filter: RobotOrientationFilter,
    just_updated: bool,

    robot_id: RobotId,
    team_color: TeamColor,
    camera_id: u32,

    previous_position: RobotPosition,
    previous_time: Instant,
}

impl CameraRobotFilter {
    const ROBOT_POSITION_INITIAL_COVARIANCE: f32 = 0.1; //[m] Uncertainty in initial robot position
    const ROBOT_VELOCITY_INITIAL_COVARIANCE: f32 = 4.0; //[m/s] Uncertainty in initial robot velocity (which is 0 for new robots)
    const ROBOT_POSITION_MEASUREMENT_ERROR: f32 = 0.005; //[m] Estimated average position uncertainty in robot detections
    const ROBOT_POSITION_MODEL_ERROR: f32 = 4.0; //[m/s^2] Assumed white noise in acceleration of a robot for process error

    const ROBOT_ANGLE_INITIAL_COVARIANCE: f32 = 0.20; //[rad] 11.5 degrees roughly
    const ROBOT_ANGULAR_VELOCITY_INITIAL_COVARIANCE: f32 = 5.0; //[rad/s] Uncertainty in initial w
    const ROBOT_ANGLE_MEASUREMENT_ERROR: f32 = 0.02; //[rad] 1.1 degrees roughly //TODO measure in practice/tune
    const ROBOT_ANGLE_MODEL_ERROR: f32 = 4.0; //[rad/s^2] Assumed white noise in angular acceleration of a robot

    pub fn new(observation: &RobotObservation, velocity: &RobotVelocity) -> Self {
        let camera_object_filter =
            CameraObjectFilter::new(0.2, 1. / 60., 10., 3., observation.get_time_captured());

        let mut initial_position_covariance: Matrix4<f32> = Matrix4::zeros();
        initial_position_covariance[(0, 0)] = Self::ROBOT_POSITION_INITIAL_COVARIANCE;
        initial_position_covariance[(1, 1)] = Self::ROBOT_POSITION_INITIAL_COVARIANCE;
        initial_position_covariance[(2, 2)] = Self::ROBOT_VELOCITY_INITIAL_COVARIANCE;
        initial_position_covariance[(3, 3)] = Self::ROBOT_VELOCITY_INITIAL_COVARIANCE;

        let initial_position: Vector4<f32> = Vector4::new(
            observation.get_position()[0],
            observation.get_position()[1],
            velocity.get_velocity()[0],
            velocity.get_velocity()[1],
        );

        let position_filter = PosVelFilter2D::new(
            initial_position,
            initial_position_covariance,
            Self::ROBOT_POSITION_MODEL_ERROR,
            Self::ROBOT_POSITION_MEASUREMENT_ERROR,
            observation.get_time_captured(),
        );

        let mut initial_orientation_covariance: Matrix2<f32> = Matrix2::zeros();
        initial_orientation_covariance[(0, 0)] = Self::ROBOT_ANGLE_INITIAL_COVARIANCE;
        initial_orientation_covariance[(1, 1)] = Self::ROBOT_ANGULAR_VELOCITY_INITIAL_COVARIANCE;

        let initial_orientation: Vector2<f32> = Vector2::new(
            observation.get_orientation(),
            velocity.get_angular_velocity(),
        );

        let orientation_filter = RobotOrientationFilter::new(
            initial_orientation,
            initial_orientation_covariance,
            Self::ROBOT_ANGLE_MODEL_ERROR,
            Self::ROBOT_ANGLE_MEASUREMENT_ERROR,
            observation.get_time_captured(),
        );

        Self {
            filter: camera_object_filter,
            position_filter,
            orientation_filter,
            just_updated: true,

            robot_id: observation.get_robot_id(),
            team_color: observation.get_team_color(),
            camera_id: observation.get_camera_id(),

            previous_position: RobotPosition::new(
                observation.get_position(),
                observation.get_orientation(),
            ),
            previous_time: observation.get_time_captured(),
        }
    }

    pub fn from_observation(observation: &RobotObservation) -> Self {
        Self::new(observation, &RobotVelocity::default())
    }

    fn update_previous_info(&mut self) {
        self.previous_position = RobotPosition::new(
            self.position_filter.get_position(),
            self.orientation_filter.get_orientation(),
        );
        self.previous_time = self.filter.get_time_last_update();
    }

    fn velocity_estimate(&self) -> RobotVelocity {
        RobotVelocity::new(
            self.position_filter.get_velocity(),
            self.orientation_filter.get_orientation(),
        )
    }

    fn accept_observation(&self, robot_observation: &RobotObservation) -> bool {
        let orientation_difference: f32 = f32::abs(
            self.orientation_filter.get_orientation() - robot_observation.get_orientation(),
        );
        let position_difference_squared: f32 =
            (robot_observation.get_position() - self.position_filter.get_position()).norm_squared();

        position_difference_squared < 0.4 * 0.4
            && orientation_difference < std::f32::consts::FRAC_PI_2
    }

    // fn estimate(&self, time:Instant) -> FilteredRobot {

    // }

    fn predict(time: Instant) {}
}
