use super::zord::Entity;

pub struct Board {
    size: u16,
    board: Vec<Entity>,
}
impl Board {
    pub fn new(size: u16) -> Self {
        Board {
            size,
            board: Vec::new(),
        }
    }
}
