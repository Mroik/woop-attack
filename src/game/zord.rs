use serde::Serialize;
use utoipa::ToSchema;

pub const BASE_RANGE: u8 = 5;
const BASE_HP: u8 = 2;

#[derive(Debug, Serialize, ToSchema)]
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
    pub fn new(owner: &str, x: i16, y: i16) -> Self {
        Zord {
            x,
            y,
            hp: BASE_HP,
            shields: 0,
            range: BASE_RANGE,
            owner: String::from(owner),
        }
    }

    pub fn hit(&mut self) -> bool {
        match self.shields {
            0 => self.hp -= 1,
            _ => self.shields -= 1,
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
    use super::Zord;

    #[test]
    fn survive_hit() {
        let mut z = Zord::new("ciao", 0, 0);
        assert!(!z.hit());
        assert_eq!(z.hp, 1);
    }

    #[test]
    fn die_on_hit() {
        let mut z = Zord::new("ciao", 0, 0);
        z.hit();
        assert!(z.hit());
        assert_eq!(z.hp, 0);
    }

    #[test]
    fn increase_range() {
        let mut z = Zord::new("ciao", 0, 0);
        z.increase_range();
        assert_eq!(z.range, 6);
    }

    #[test]
    fn generate_shield() {
        let mut z = Zord::new("ciao", 0, 0);
        z.generate_shield();
        assert_eq!(z.shields, 1);
    }
}
