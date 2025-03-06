use super::parameters::robot_parameter_database::RobotParameterDatabase;
use super::filters::{referee::referee_filter::RefereeFilter, vision::vision_filter::VisionFilter};

use protos::sslvision::SslWrapperPacket;
use protos::gamecontroller::Referee;
use protos::roboteam::RobotFeedback;

pub struct Observer {
    robot_parameter_database: RobotParameterDatabase,
    referee_filter: RefereeFilter,
    vision_flter: VisionFilter,
}

impl Observer {
    
    pub fn process(
        &self,
        vision_packets: &Vec<SslWrapperPacket>,
        referee_packets: &Vec<Referee>,
        feedback_packets: &Vec<RobotFeedback>,
        camera_ids: &Vec<u32>,
    ) {
        todo!("Implement Observer::process");
    }

    fn update_robot_parameters(&self, referee_packets: &Vec<Referee>) {
        todo!("Implement Observer::update_robot_parameters");
    }

    fn update_referee(&self, referee_packets: &Vec<Referee>) {
        todo!("Implement Observer::update_referee");
    }
}