#[derive(Clone)]
pub struct RobotParameters {
    radius: f32,
    height: f32,
    front_width: f32,
    dribbler_width: f32,
    yaw_offset: f32,
}

impl RobotParameters {
    pub fn settings_2020() -> RobotParameters {
        RobotParameters {
            radius: 0.09,
            height: 0.15,
            front_width: 0.10,
            dribbler_width: 0.10,
            yaw_offset: 0.0,
        }
    }
}

impl Default for RobotParameters {
    fn default() -> Self {
        RobotParameters {
            radius: 0.09,
            height: 0.15,
            front_width: 0.10,
            dribbler_width: 0.10,
            yaw_offset: 0.0,
        }
    }
}
