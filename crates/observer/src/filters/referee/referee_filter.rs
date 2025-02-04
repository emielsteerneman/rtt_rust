use protos::messages::Referee;

#[derive(Default)]
pub struct RefereeFilter {
    first_message_received:bool,
    last_message:Referee,
}

impl RefereeFilter {

    pub fn update(&mut self, message: &Referee) {
        self.first_message_received = true;
        self.last_message = message.clone();
    }

    pub fn get_last_referee_message(&self) -> Option<Referee> {
        match self.first_message_received {
            true => Some(self.last_message.clone()),
            false => None,
        }
    }

}