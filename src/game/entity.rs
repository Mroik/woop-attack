use super::{totem::Totem, zord::Zord};

#[derive(Debug)]
pub enum Entity {
    Zord(Zord),
    Totem(Totem),
}

impl Entity {
    pub fn is_coord(&self, x: i32, y: i32) -> bool {
        match self {
            Entity::Zord(z) => z.x == x && z.y == y,
            Entity::Totem(t) => t.x == x && t.y == y,
        }
    }

    pub fn is_zord(&self) -> bool {
        match self {
            Entity::Zord(_) => true,
            _ => false,
        }
    }

    pub fn get_zord(&self) -> Option<&Zord> {
        match self {
            Entity::Zord(z) => Some(z),
            _ => None,
        }
    }

    pub fn zord_hit(&mut self) -> bool {
        match self {
            Entity::Zord(z) => z.hit(),
            _ => false,
        }
    }
}
