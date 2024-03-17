use crate::game::{entity::Entity, player::Player};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiReply {
    Error(String),
    Data(Reply),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Request {
    // Actions
    Shoot {
        player: String,
        from: (i16, i16),
        to: (i16, i16),
    },
    Move {
        player: String,
        from: (i16, i16),
        to: (i16, i16),
    },
    IncreaseRange {
        player: String,
        coord: (i16, i16),
    },
    GenerateShield {
        player: String,
        coord: (i16, i16),
    },
    Donate {
        donator: String,
        receiver: String,
        amount: u16,
    },
    BuildZord {
        player: String,
        coord: (i16, i16),
    },

    // Other stuff
    Map,
    Leaderboard,
    Day,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Reply {
    Map(Vec<Entity>),
    Leaderboard(Vec<Player>),
    Day(u8),
    Ok,
}
