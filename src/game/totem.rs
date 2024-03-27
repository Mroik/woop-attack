use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct Totem {
    pub x: i16,
    pub y: i16,
}

impl Totem {
    pub fn new(x: i16, y: i16) -> Totem {
        Totem { x, y }
    }
}
