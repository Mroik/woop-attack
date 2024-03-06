use super::totem::Totem;

const BASE_RANGE: u8 = 5;
const BASE_HP: u8 = 2;

pub struct Zord {
    x: i32,
    y: i32,
    hp: u8,
    shields: u8,
    range: u8,
}

pub enum Entity {
    Zord(Zord),
    Totem(Totem),
}

// Since move requires for the board to be passed it is implemented in board
impl Zord {
    fn hit(&mut self) -> bool {
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
    use super::Zord;

    #[test]
    fn survive_hit() {
        let mut z = Zord {
            x: 0,
            y: 0,
            hp: 2,
            shields: 0,
            range: 5,
        };
        assert!(!z.hit());
        assert_eq!(z.hp, 1);
    }

    #[test]
    fn die_on_hit() {
        let mut z = Zord {
            x: 0,
            y: 0,
            hp: 1,
            shields: 0,
            range: 5,
        };
        assert!(z.hit());
        assert_eq!(z.hp, 0);
    }

    #[test]
    fn increase_range() {
        let mut z = Zord {
            x: 0,
            y: 0,
            hp: 2,
            shields: 0,
            range: 5,
        };
        z.increase_range();
        assert_eq!(z.range, 6);
    }

    #[test]
    fn generate_shield() {
        let mut z = Zord {
            x: 0,
            y: 0,
            hp: 2,
            shields: 0,
            range: 5,
        };
        z.generate_shield();
        assert_eq!(z.shields, 1);
    }
}
