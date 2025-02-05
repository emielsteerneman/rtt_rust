#![allow(dead_code)]

use super::kalman_filter::KalmanFilter;

use nalgebra::{max, SMatrix, SVector};

struct PosVelFilter1D {
    filter: KalmanFilter<2, 1>,
    last_updated_time: std::time::Instant,
    model_error: f32,
}

impl Default for PosVelFilter1D {
    fn default() -> Self {
        Self {
            filter: KalmanFilter::default(),
            last_updated_time: std::time::Instant::now(),
            model_error: f32::default(),
        }
    }
}

impl PosVelFilter1D {
    pub fn new(
        initial_state: SVector<f32, 2>,
        initial_covariance: SMatrix<f32, 2, 2>,
        model_error: f32,
        measurement_error: f32,
        timestamp: std::time::Instant,
    ) -> Self {
        let mut filter = KalmanFilter::new(initial_state, initial_covariance);

        filter.R[(0, 0)] = measurement_error;

        Self {
            filter,
            last_updated_time: timestamp,
            model_error,
        }
    }

    pub fn predict(&mut self, time: std::time::Instant) -> bool {
        let dt = (time - self.last_updated_time).as_secs_f32();

        if dt < 0. {
            return false;
        }

        self.last_updated_time = time;
        self.set_transition_matrix_dt(dt);
        self.set_process_noise_from_random_acceleration(dt);
        self.filter.predict();

        true
    }

    pub fn update(&mut self, position: f32) {
        self.filter.update(SVector::from_vec(vec![position]));
    }

    pub fn set_measurement_error(&mut self, measurement_error: f32) {
        self.filter.R[(0, 0)] = measurement_error;
    }

    pub fn set_transition_matrix_dt(&mut self, dt: f32) {
        self.filter.F[(0, 1)] = dt;
    }

    // http://www.robots.ox.ac.uk/~ian/Teaching/Estimation/LectureNotes2.pdf
    // See page 13 ish for more explanation
    pub fn set_process_noise_from_random_acceleration(&mut self, dt: f32) {
        let sigma_squared = self.model_error * self.model_error;

        let dt3: f32 = (1.0 / 3.0) * dt * dt * dt * sigma_squared;
        let dt2: f32 = (1.0 / 2.0) * dt * dt * sigma_squared;
        let dt1: f32 = dt * sigma_squared;

        self.filter.Q[(0, 0)] = dt3;
        self.filter.Q[(0, 1)] = dt2;
        self.filter.Q[(1, 0)] = dt2;
        self.filter.Q[(1, 1)] = dt1;
    }

    pub fn get_state(&self) -> &SVector<f32, 2> {
        self.filter.state()
    }

    pub fn set_state(&mut self, state: SVector<f32, 2>) {
        self.filter.set_state(state);
    }

    pub fn get_covariance(&self) -> &SMatrix<f32, 2, 2> {
        self.filter.covariance()
    }

    pub fn set_covariance(&mut self, covariance: SMatrix<f32, 2, 2>) {
        self.filter.set_covariance(covariance);
    }

    pub fn get_innovation(&self) -> f32 {
        self.filter.y[(0, 0)]
    }

    pub fn last_updated_time(&self) -> std::time::Instant {
        self.last_updated_time
    }
}

/* Position implementation */
impl PosVelFilter1D {
    pub fn get_position(&self) -> f32 {
        self.filter.state()[0]
    }

    pub fn get_position_estimate(&self, time: std::time::Instant) -> f32 {
        let dt = max(
            time - self.last_updated_time,
            std::time::Duration::from_secs(0),
        );
        self.get_position() + self.get_velocity() * dt.as_secs_f32()
    }

    pub fn get_position_uncertainty(&self) -> f32 {
        self.filter.covariance()[(0, 0)].sqrt()
    }

    pub fn set_position(&mut self, position: f32) {
        self.filter.modify_state(0, position);
    }
}

/* Velocity implementation */
impl PosVelFilter1D {
    pub fn get_velocity(&self) -> f32 {
        self.filter.state()[1]
    }

    pub fn get_velocity_uncertainty(&self) -> f32 {
        self.filter.covariance()[(1, 1)].sqrt()
    }

    pub fn set_velocity(&mut self, velocity: f32) {
        self.filter.modify_state(1, velocity);
    }
}
