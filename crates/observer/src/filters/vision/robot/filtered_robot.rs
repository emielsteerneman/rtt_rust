use derive_more::Constructor;
use protos::roboteam::WorldRobot;

use super::robot_observation::{RobotId, RobotPosition, RobotVelocity, TeamColor};

#[derive(Constructor)]
pub struct FilteredRobot {
    id: RobotId,
    team: TeamColor,
    position: RobotPosition,
    velocity: RobotVelocity,
    health: f32,
    position_uncertainty: f32,
    velocity_uncertainty: f32,
    yaw_uncertainty: f32,
    angular_velocity_uncertainty: f32,
}

impl FilteredRobot {
    pub fn as_world_robot(self) -> WorldRobot {
        let mut robot = WorldRobot::default();

        robot.pos = Some(protos::roboteam::Vector2f {
            x: self.position.position.x,
            y: self.position.position.y,
        });

        robot.vel = Some(protos::roboteam::Vector2f {
            x: self.velocity.velocity.x,
            y: self.velocity.velocity.y,
        });

        robot.yaw = self.position.orientation;

        robot.w = self.velocity.angular_velocity;
        robot.id = self.id.id as u32;

        robot
    }
}