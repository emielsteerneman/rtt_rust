use protos::messages::Referee;

use super::robot_parameters::RobotParameters;

#[derive(Default)]
pub struct TwoTeamRobotParameters {
    blue_changed: bool,
    yellow_changed: bool,

    blue_parameters: RobotParameters,
    yellow_parameters: RobotParameters,
}

#[derive(Default)]
pub struct RobotParameterDatabase {
    blue_name: String,
    blue_parameters: RobotParameters,
    yellow_name: String,
    yellow_parameters: RobotParameters,
}

impl RobotParameterDatabase {
    pub fn update(&mut self, referee_message: &Referee) -> TwoTeamRobotParameters {
        let mut parameters = TwoTeamRobotParameters::default();

        if referee_message.blue.name != self.blue_name {
            parameters.blue_changed = true;
            parameters.blue_parameters =
                RobotParameterDatabase::get_team_parameters(&referee_message.blue.name);
        }

        if referee_message.yellow.name != self.yellow_name {
            parameters.yellow_changed = true;
            parameters.yellow_parameters =
                RobotParameterDatabase::get_team_parameters(&referee_message.yellow.name);
        }

        self.blue_parameters = parameters.blue_parameters.clone();
        self.yellow_parameters = parameters.yellow_parameters.clone();

        parameters
    }

    pub fn get_parameters(&self) -> TwoTeamRobotParameters {
        TwoTeamRobotParameters {
            blue_changed: false,
            yellow_changed: false,
            blue_parameters: self.blue_parameters.clone(),
            yellow_parameters: self.yellow_parameters.clone(),
        }
    }

    fn get_team_parameters(team_name: &String) -> RobotParameters {
        if team_name == "RoboTeam Twente" {
            return RobotParameters::settings_2020();
        } else {
            return RobotParameters::default();
        }
    }

}