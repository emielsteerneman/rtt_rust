use std::time::Instant;

use crate::parameters::robot_parameter_database::TwoTeamRobotParameters;

use super::geometry_filter::GeometryFilter;
use super::world_filter::WorldFilter;

use protos::{roboteam::{RobotFeedback, World}, sslvision::{SslGeometryData, SslWrapperPacket}};

/**
 * @author Rolf
 * @class VisionFilter Filters and processes all information received from SSL-Vision into coherent information.
 *  This primarily involves filters to track the robots and the ball and estimate their position and speed, and some processing of the geometry
 *  Additionally, it also filters feedback information from the robots into a coherent picture.
 *  At some point in the future, it is also intended that sent robots commands are processed.
 */
pub struct VisionFilter {
    geometry_filter: GeometryFilter,
    world_filter: WorldFilter,
    last_packet_time: std::time::Instant,
    time_extrapolation_policy: TimeExtrapolationPolicy,
}

impl VisionFilter {
    /**
     *  This function updates the vision filter with vision packets and returns the world state as estimated at time.
     *  Note time needs to be at some point in the future!
     * @param packets the received packets to update visionfilter with
     * @param time the wanted time to estimate the world state at (can extrapolate in the future)
     * @param feedback the feedback packets received for all robots
     * @param camera_ids the camera ids to process, if empty all cameras are processed
     * @return a world state, extrapolated to the given time
     */
    pub fn process(packets: &Vec<SslWrapperPacket>, feedback: &Vec<RobotFeedback>, camera_ids: &Vec<i64>) -> World {
        todo!{}
    }

    pub fn update_robot_parameters(parameters: &TwoTeamRobotParameters) {
        todo!{}
    }

    pub fn get_geometry() -> Option<SslGeometryData> {
        todo!{}
    }

    pub fn set_extrapolation_policy(policy: TimeExtrapolationPolicy) {
        todo!{}
    }
}

impl VisionFilter {
    fn get_extrapolation_time_for_policy() -> Instant {
        todo!{}
    }

    fn process_geometry(packets: &Vec<SslWrapperPacket>){
        todo!{}
    }

    fn process_detections(packets: &Vec<SslWrapperPacket>, feedback: &Vec<RobotFeedback>, camera_ids: &Vec<i64>) {
        todo!{}
    }
}

pub enum TimeExtrapolationPolicy {
    REALTIME,
    LAST_RECEIVED_PACKET_TIME,
}
