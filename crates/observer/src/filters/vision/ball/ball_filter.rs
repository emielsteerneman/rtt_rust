use std::{collections::HashMap, time::Instant};

use derive_more::Constructor;

use crate::filters::vision::robot::filtered_robot::FilteredRobot;

use super::{ball_observation::BallObservation, camera_ground_ball_filter::{CameraGroundBallFilter, CameraGroundBallPrediction, CameraGroundBallPredictionObservationPair}, filtered_ball::FilteredBall};

#[derive(Constructor)]
struct GroundBallPrediction {
    prediction: CameraGroundBallPrediction,
    has_requested_camera: bool,
}

pub struct BallFilter {
    ground_filters: HashMap<usize, CameraGroundBallFilter>
}

impl BallFilter {
    pub fn new(observation: &BallObservation) -> Self {
        Self {
            ground_filters: HashMap::from(
                [( observation.camera_id, CameraGroundBallFilter::from_observation(observation) )]
            )
        }   
    }

    /**
     * Predicts the filter until Time to get a ground prediction for camera ID.
     * If the camera is not tracked by the current filter, information from other camera's is used.
     * @param cameraID to predict the ball for
     * @param until Time to predict the ball at
     * @return The prediction
     */
    fn predict_cam(&mut self, camera_id: usize, until: Instant, robots_yellow: &Vec<FilteredRobot>, robots_blue: &Vec<FilteredRobot>) -> GroundBallPrediction{
        let camera_filter = self.ground_filters.get(&camera_id);

        match camera_filter {
            Some(filter) => {
                let prediction = filter.predict(until, robots_yellow, robots_blue);
                GroundBallPrediction::new(prediction, true)
            },
            None => {
                let estimate = self.merge_balls(until);
                let prediction = CameraGroundBallPrediction::new(estimate.position, estimate.velocity, until);
                GroundBallPrediction::new(prediction, false)
            }
        }
    }

    /**
     * Updates the ground filter with a given camera ID with Prediction Observation pairs.
     * If not yet tracked, this camera is added to the list of tracked camera's.
     * @return true if the filter can be removed e.g. it's components are empty for a while.
     */
    fn process_detections(&mut self, detections: CameraGroundBallPredictionObservationPair, camera_id: usize) -> bool {
        let camera_filter = self.ground_filters.get(&camera_id);
        
        if let Some(camera_filter) = camera_filter {
            let remove_filter = camera_filter.process_detections(&detections);
            if remove_filter {
                self.ground_filters.remove(&camera_id);
            }
            self.ground_filters.is_empty()
        } else {
            if let Some(observation) = &detections.observation {
                self.ground_filters.insert(
                    camera_id,
                    CameraGroundBallFilter::new(observation, detections.prediction.velocity)
                );
            }
            false
        }
    }

    /**
     * Predicts and merges the ball filter at given time
     * @param time
     * @return The Predicted/Filtered ball
     */
    fn merge_balls(&self, time:Instant) -> FilteredBall {
        const MERGE_FACTOR:f32 = 1.5;
        
        let mut ball = FilteredBall::default();

        for filter in self.ground_filters.values() {
            let estimate = filter.get_estimate(time);
            let inverse_weight:f32 = 100.0 / estimate.health;
            let position_weight: f32 = (estimate.position_uncertainty * inverse_weight).powf(-MERGE_FACTOR);
            let velocity_weight: f32 = (estimate.velocity_uncertainty * inverse_weight).powf(-MERGE_FACTOR);
            ball.position += estimate.position * position_weight;
            ball.velocity += estimate.velocity * velocity_weight;
            ball.position_uncertainty += position_weight;
            ball.velocity_uncertainty += velocity_weight;
        }
        
        // Guard against division by zero
        let limit:f32 = 1e-10;
        
        if limit <= ball.position_uncertainty {
            ball.position /= ball.position_uncertainty;
        }
        if limit <= ball.velocity_uncertainty {
            ball.velocity /= ball.velocity_uncertainty;
        }

        ball
    }

    fn get_health(&self) -> f32 {
        let mut max_health: f32 = 0.0;
        for filter in self.ground_filters.values() {
            max_health = max_health.max(filter.get_health());
        }
        max_health
    }

    fn get_num_observations(&self) -> usize {
        let mut max_num_observations: usize = 0;
        for filter in self.ground_filters.values() {
            max_num_observations = max_num_observations.max(filter.get_num_observations());
        }
        max_num_observations
    }

}

