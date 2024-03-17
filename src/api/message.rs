use crate::game::{entity::Entity, player::Player};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiReply {
    Error(String),
    Data(Reply),
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum Request {
    // Actions
    DoubleCoord {
        player: String,
        from: (i16, i16),
        to: (i16, i16),
    },
    SingleCoord {
        player: String,
        coord: (i16, i16),
    },
    Donate {
        donator: String,
        receiver: String,
        amount: u16,
    },

    // Data retrival
    Info {
        requesting: String,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Reply {
    Map(Vec<Entity>),
    Leaderboard(Vec<Player>),
    Day(u8),
    Ok,
}
