use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
enum WoopError {
    OutOfActions,
    ZordNotFound(i16, i16),
    NotEnoughPoints(u16, u16),
    CellOccupied(i16, i16),
    OutOfBounds(i16, i16),
    NotInRange(i16, i16, i16, i16),
}

impl Error for WoopError {}

impl Display for WoopError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
        }
    }
}
