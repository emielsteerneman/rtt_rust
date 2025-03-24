use std::{collections::HashMap, time::Instant};

use derive_more::Constructor;

use crate::filters::vision::{camera, robot::filtered_robot::FilteredRobot};

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
        todo!()
    }

    /**
     * Predicts the filter until Time to get a ground prediction for camera ID.
     * If the camera is not tracked by the current filter, information from other camera's is used.
     * @param cameraID to predict the ball for
     * @param until Time to predict the ball at
     * @return The prediction
     */
    fn predict_cam(self, camera_id: usize, until: Instant, robots_yellow: &Vec<FilteredRobot>, robots_blue: &Vec<FilteredRobot>) -> GroundBallPrediction{
        todo!()
    }

    /**
     * Updates the ground filter with a given camera ID with Prediction Observation pairs.
     * If not yet tracked, this camera is added to the list of tracked camera's.
     * @return true if the filter can be removed e.g. it's components are empty for a while.
     */
    fn process_detections(self, detections: CameraGroundBallPredictionObservationPair, camera_id: usize) -> bool {
        todo!()
    }

    /**
     * Predicts and merges the ball filter at given time
     * @param time
     * @return The Predicted/Filtered ball
     */
    fn merge_balls(self, time:Instant) -> FilteredBall {
        todo!()
    }

    /**
     * @return Gets the health of the ball filter
     */
    fn get_health(self) -> f32 {
        todo!()
    }

    /**
     * @return The number of observations the filter has seen
     */
    fn get_num_observations(self) -> usize {
        todo!()
    }

}

