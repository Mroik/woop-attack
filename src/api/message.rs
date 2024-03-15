use serde::{Deserialize, Serialize};

use crate::game::{entity::Entity, player::Player};

#[derive(Deserialize, Clone)]
pub enum Request {
    // Actions
    Shoot(String, (i16, i16), (i16, i16)),
    Move(String, (i16, i16), (i16, i16)),
    IncreaseRange(String, (i16, i16)),
    GenerateShield(String, (i16, i16)),
    Donate(String, String),
    BuildZord(String, (i16, i16)),

    // Other stuff
    Map,
    Leaderboard,
    Day,
}

#[derive(Serialize)]
pub enum Reply {
    Map(Vec<Entity>),
    Leaderboard(Vec<Player>),
    Day(u8),
    Ok,
}
