use std::{collections::HashMap, time::Instant};

use super::{
    camera_robot_filter::CameraRobotFilter,
    robot_observation::{RobotId, RobotObservation, TeamColor},
};

pub struct RobotFilter {
    id: RobotId,
    team: TeamColor,
    camera_filters: HashMap<usize, CameraRobotFilter>,
}

impl RobotFilter {
    pub fn new(observation: &RobotObservation) -> Self {
        Self {
            id: observation.get_robot_id(),
            team: observation.get_team_color(),
            camera_filters: HashMap::from([(0, CameraRobotFilter::from_observation(observation))]),
        }
    }

    /**
     * Optionally processes the given detection, returns true if the detection is accepted by one of the camera filters
     * @param observation
     * @return true if the observation is accepted
     */
    pub fn process_detection(observation: &RobotObservation) {
        todo!()
    }

    /**
     * Predicts the robot on the given cameraID (if a filter for this camera exists)
     * @param cameraID the id of the camera
     * @param untilTime time to predict to
     */
    pub fn predict_cam(camera_id: usize, time_until: Instant) {
        todo!()
    }

    /**
     * Checks if a camera has not seen, and if so, processes the fact that an object was not seen by a camera frame taken at time
     * @param cameraID
     * @param untilTime
     * @return true if this filter can be removed (e.g. is empty), false otherwise
     */
    pub fn process_not_seen(camera_id: usize, time: Instant) {
        todo!()
    }

    /**
     * @brief Merges robot estimates from different cameras into a single robot estimate
     * @param time to estimate the robot position at
     * @return the estimated robot position
     */
    pub fn merge_robots(time: Instant) {
        todo!()
    }

    /**
     * @param cameraID
     * @param time
     * @return the estimate of the robot on the given camera at time t, if a filter which tracks the robot on that camera exists
     */
    pub fn get_robot(camera_id: usize, time: Instant) {
        todo!()
    }

    pub fn get_health() -> f64 {
        todo!()
    }

    pub fn get_num_observations() -> usize {
        todo!()
    }

}
