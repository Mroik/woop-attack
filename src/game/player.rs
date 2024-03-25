use serde::Serialize;

pub const BASE_ACTIONS: u8 = 20;

#[derive(PartialEq, Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Player {
    pub name: String,
    pub actions: u8,
    pub points: u16,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            actions: BASE_ACTIONS,
            points: 0,
        }
    }

    pub fn spend_action(&mut self, amount: u8) {
        self.actions -= amount;
    }
}
