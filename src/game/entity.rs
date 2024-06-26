use super::{totem::Totem, zord::Zord};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Entity {
    Zord(Zord),
    Totem(Totem),
}

impl Entity {
    pub fn is_coord(&self, x: i16, y: i16) -> bool {
        match self {
            Entity::Zord(z) => z.x == x && z.y == y,
            Entity::Totem(t) => t.x == x && t.y == y,
        }
    }

    pub fn is_zord(&self) -> bool {
        matches!(self, Entity::Zord(_))
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

    pub fn zord_generate_shield(&mut self) {
        if let Entity::Zord(z) = self {
            z.generate_shield();
        }
    }

    pub fn move_zord(&mut self, x: i16, y: i16) {
        if let Entity::Zord(z) = self {
            z.set_coord(x, y);
        }
    }

    pub fn distance(&self, x: i16, y: i16) -> i16 {
        match self {
            Entity::Zord(z) => (z.x - x).abs().max((z.y - y).abs()),
            Entity::Totem(t) => (t.x - x).abs().max((t.y - y).abs()),
        }
    }

    pub fn get_totem(&self) -> Option<&Totem> {
        match self {
            Entity::Totem(t) => Some(t),
            _ => None,
        }
    }
}
