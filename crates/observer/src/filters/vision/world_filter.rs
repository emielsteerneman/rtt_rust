use std::collections::HashMap;

use protos::{gamecontroller::RobotId, sslvision::SslDetectionFrame};

use super::{ball::ball_filter::BallFilter, robot::robot_filter::RobotFilter};

pub struct WorldFilter {
    map_blue: HashMap<RobotId, Vec<RobotFilter>>,
    map_yellow: HashMap<RobotId, Vec<RobotFilter>>,
    balls: Vec<BallFilter>,
}

impl WorldFilter {
    pub fn process(frames: &Vec<SslDetectionFrame>, feedback: &Vec<RobotFeedback>) -> World {
        todo! {}
    }
}