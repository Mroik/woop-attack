use super::player::Player;

pub const BASE_RANGE: u8 = 5;
const BASE_HP: u8 = 2;

#[derive(Debug)]
pub struct Zord {
    pub x: i16,
    pub y: i16,
    pub hp: u8,
    pub shields: u8,
    pub range: u8,
    pub owner: String,
}

// Since move requires for the board to be passed it is implemented in board
impl Zord {
    pub fn new(owner: &Player, x: i16, y: i16) -> Self {
        Zord {
            x,
            y,
            hp: BASE_HP,
            shields: 0,
            range: BASE_RANGE,
            owner: owner.name.clone(),
        }
    }

    pub fn hit(&mut self) -> bool {
        if self.shields > 0 {
            self.shields -= 1;
        } else {
            self.hp -= 1;
        }
        self.hp == 0
    }

    pub fn increase_range(&mut self) {
        self.range += 1;
    }

    pub fn generate_shield(&mut self) {
        self.shields += 1;
    }

    pub fn set_coord(&mut self, x: i16, y: i16) {
        self.x = x;
        self.y = y;
    }
}

#[cfg(test)]
mod tests {
    use crate::game::player::Player;

    use super::Zord;

    #[test]
    fn survive_hit() {
        let player = Player::new("ciao");
        let mut z = Zord::new(&player, 0, 0);
        assert!(!z.hit());
        assert_eq!(z.hp, 1);
    }

    #[test]
    fn die_on_hit() {
        let player = Player::new("ciao");
        let mut z = Zord::new(&player, 0, 0);
        z.hit();
        assert!(z.hit());
        assert_eq!(z.hp, 0);
    }

    #[test]
    fn increase_range() {
        let player = Player::new("ciao");
        let mut z = Zord::new(&player, 0, 0);
        z.increase_range();
        assert_eq!(z.range, 6);
    }

    #[test]
    fn generate_shield() {
        let player = Player::new("ciao");
        let mut z = Zord::new(&player, 0, 0);
        z.generate_shield();
        assert_eq!(z.shields, 1);
    }
}
