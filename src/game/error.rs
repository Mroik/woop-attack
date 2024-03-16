use serde::Serialize;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Serialize)]
pub enum WoopError {
    OutOfActions,
    ZordNotFound(i16, i16),
    NotEnoughPoints(u16, u16),
    CellOccupied(i16, i16),
    OutOfBounds(i16, i16),
    NotInRange(i16, i16, i16, i16),
    Generic,
    PlayerNotFound(String),
    WithinGracePeriod,
    NoZordNearby(i16, i16),
    NotOwned(i16, i16),
}

impl WoopError {
    pub fn out_of_actions() -> Result<(), WoopError> {
        Err(WoopError::OutOfActions)
    }

    pub fn zord_not_found(x: i16, y: i16) -> Result<(), WoopError> {
        Err(WoopError::ZordNotFound(x, y))
    }

    pub fn generic() -> Result<(), WoopError> {
        Err(WoopError::Generic)
    }

    pub fn player_not_found(player: &str) -> Result<(), WoopError> {
        Err(WoopError::PlayerNotFound(String::from(player)))
    }

    pub fn not_enough_points(current: u16, required: u16) -> Result<(), WoopError> {
        Err(WoopError::NotEnoughPoints(current, required))
    }

    pub fn cell_occupied(x: i16, y: i16) -> Result<(), WoopError> {
        Err(WoopError::CellOccupied(x, y))
    }

    pub fn not_in_range(x_f: i16, y_f: i16, x_t: i16, y_t: i16) -> Result<(), WoopError> {
        Err(WoopError::NotInRange(x_f, y_f, x_t, y_t))
    }

    pub fn out_of_bounds(x: i16, y: i16) -> Result<(), WoopError> {
        Err(WoopError::OutOfBounds(x, y))
    }

    pub fn within_grace_period() -> Result<(), WoopError> {
        Err(WoopError::WithinGracePeriod)
    }

    pub fn no_zord_nearby(x: i16, y: i16) -> Result<(), WoopError> {
        Err(WoopError::NoZordNearby(x, y))
    }

    pub fn not_owned(x: i16, y: i16) -> Result<(), WoopError> {
        Err(WoopError::NotOwned(x, y))
    }
}

impl Error for WoopError {}

impl Display for WoopError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfActions => write!(f, "Not enough actions"),
            Self::ZordNotFound(x, y) => write!(f, "Couldn't find zord at ({}, {})", x, y),
            Self::NotEnoughPoints(current, required) => {
                write!(f, "You have {} points but need {}", current, required)
            }
            Self::CellOccupied(x, y) => write!(f, "The cell ({}, {}) is occupied", x, y),
            Self::OutOfBounds(x, y) => write!(f, "({}, {}) is out of bounds", x, y),
            Self::NotInRange(x_f, y_f, x_t, y_t) => write!(
                f,
                "({}, {}) is not in range of ({}, {})",
                x_t, y_t, x_f, y_f
            ),
            Self::Generic => write!(f, "Internal error"),
            Self::PlayerNotFound(player) => write!(f, "Couldn't find player named {}", player),
            Self::WithinGracePeriod => write!(f, "You tried shooting within the grace period"),
            Self::NoZordNearby(x, y) => write!(f, "There's no zord nearby ({}, {})", x, y),
            Self::NotOwned(x, y) => write!(f, "You don't own the zord in ({}, {})", x, y),
        }
    }
}
