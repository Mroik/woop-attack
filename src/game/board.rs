use super::entity::Entity;

#[derive(Debug)]
pub struct Board {
    pub size: i16,
    pub board: Vec<Entity>,
}
impl Board {
    pub fn new(size: i16) -> Self {
        Board {
            size,
            board: Vec::new(),
        }
    }
}
