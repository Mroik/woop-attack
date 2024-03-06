use super::entity::Entity;

pub struct Board {
    size: u16,
    pub board: Vec<Entity>,
}
impl Board {
    pub fn new(size: u16) -> Self {
        Board {
            size,
            board: Vec::new(),
        }
    }
}
