use crate::game::{entity::Entity, log::PlayerEvent, player::Player};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiReply<'a> {
    Error(String),
    Data(Reply<'a>),
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum Request {
    // Actions
    DoubleCoord { from: (i16, i16), to: (i16, i16) },
    SingleCoord { coord: (i16, i16) },
    Donate { receiver: String, amount: u16 },
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Reply<'a> {
    Map(&'a Vec<Entity>),
    Leaderboard(&'a Vec<Player>),
    GameInfo { day: u8, start_of_day: u64 },
    Ok,
    Activity(Vec<PlayerEvent>),
}
