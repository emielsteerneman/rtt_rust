use common::pos_vel_filter_1d::PosVelFilter1D;
use nalgebra::{Matrix2, Vector2};
use std::{f32::consts::PI, time::Instant};

#[derive(Default)]
pub struct RobotOrientationFilter {
    filter: PosVelFilter1D,
    last_observation: f32,
    orientation_turns: u32,
}

impl RobotOrientationFilter {
    pub fn new(
        initial_state: Vector2<f32>,
        initial_covariance: Matrix2<f32>,
        model_error: f32,
        measurement_error: f32,
        time: Instant,
    ) -> Self {
        Self {
            filter: PosVelFilter1D::new(
                initial_state,
                initial_covariance,
                model_error,
                measurement_error,
                time,
            ),
            last_observation: 0.,
            orientation_turns: 0,
        }
    }

    pub fn limit_angle(mut orientation: f32) -> f32 {
        orientation += PI;
        orientation %= 2. * PI;
        orientation -= PI;
        orientation
    }

    pub fn update(&mut self, observation: f32) {
        // We need to do something about the rotation's discontinuities at -pi/pi so it works correctly.
        // We allow the state to go outside of bounds [-PI,PI) in between updates, but then simply make sure the observation difference is correct

        if observation < -0.5 * PI && 0.5 * PI < self.last_observation {
            self.orientation_turns += 1;
        }
        if -0.5 * PI < observation && self.last_observation < 0.5 * PI {
            self.orientation_turns -= 1;
        }
        self.last_observation = observation;

        let corrected_observation: f32 = observation + self.orientation_turns as f32 * 2. * PI;

        self.filter.update(corrected_observation);
    }

    pub fn get_orientation(&self) -> f32 {
        Self::limit_angle(self.filter.get_position())
    }

    pub fn get_orientation_estimate(&self, time: Instant) -> f32 {
        Self::limit_angle(self.filter.get_position_estimate(time))
    }
}
