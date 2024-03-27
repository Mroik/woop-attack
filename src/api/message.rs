use crate::game::{entity::Entity, log::PlayerEvent, player::Player};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, Clone, ToSchema, IntoParams)]
pub struct DoubleCoord {
    /// Coordinates of your own zord
    pub from: (i16, i16),
    /// Coordinates of target zord
    pub to: (i16, i16),
}

#[derive(Deserialize, Serialize, Clone, ToSchema, IntoParams)]
pub struct SingleCoord {
    pub coord: (i16, i16),
}

#[derive(Deserialize, Serialize, Clone, ToSchema, IntoParams)]
pub struct Donate {
    pub receiver: String,
    pub amount: u16,
}

#[derive(Serialize, ToSchema)]
pub enum Empty {
    Error(String),
    Ok,
}

#[derive(Serialize, ToSchema)]
pub struct WoopMap<'a> {
    pub map: &'a Vec<Entity>,
}

#[derive(Serialize, ToSchema)]
pub struct Leaderboard<'a> {
    /// List of players sorted by points
    pub leaderboard: &'a Vec<Player>,
}

#[derive(Serialize, ToSchema)]
pub struct GameInfo {
    /// Current game day
    pub day: u8,
    /// Unix timestamp of the start of the day
    pub start_of_day: u64,
}

#[derive(Serialize, ToSchema)]
pub struct Activity {
    /// List of the last 100 actions
    pub activity: Vec<PlayerEvent>,
}
