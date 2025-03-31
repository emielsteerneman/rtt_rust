use super::robot_observation::{RobotId, RobotVelocity};

pub struct RobotFeedback {
    id: RobotId,
    ballsensor_sees_ball: bool,
    ballsensor_working: bool,
    dribbler_sees_ball: bool,
    velocity: RobotVelocity,
    orientation: f32,
    imu_calibrated: bool,
    capacitor_charged: bool,
    battery_voltage: f32,
}

pub struct RobotFeedbackCollection {
    
}