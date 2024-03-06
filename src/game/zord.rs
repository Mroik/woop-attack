use super::player::Player;

const BASE_RANGE: u8 = 5;
const BASE_HP: u8 = 2;

#[derive(Debug)]
pub struct Zord {
    pub x: i32,
    pub y: i32,
    pub hp: u8,
    shields: u8,
    pub range: u8,
    pub owner: String,
}

// Since move requires for the board to be passed it is implemented in board
impl Zord {
    pub fn new(owner: &Player, x: i32, y: i32) -> Self {
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

    fn increase_range(&mut self) {
        self.range += 1;
    }

    fn generate_shield(&mut self) {
        self.shields += 1;
    }

    fn day_reset(&mut self) {
        self.shields = 0;
        self.range = BASE_RANGE;
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
