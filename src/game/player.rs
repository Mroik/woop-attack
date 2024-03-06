const BASE_ACTIONS: u8 = 5;

pub struct Player {
    pub name: String,
    actions: u8,
    points: u16,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            actions: BASE_ACTIONS,
            points: 0,
        }
    }
}
