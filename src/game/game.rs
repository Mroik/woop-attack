use super::{board::Board, player::Player};

const BASE_BOARD_SIZE: u16 = 10000;

struct Game {
    players: Vec<Player>,
    board: Board,
}

impl Game {
    fn new(names: Vec<String>) -> Self {
        let players = names.iter().map(|name| Player::new(name)).collect();
        Game {
            players,
            board: Board::new(BASE_BOARD_SIZE),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn new_game() {
        let names = vec!["mroik", "fin", "warden"];
        let game = Game::new(names.iter().map(|name| name.to_string()).collect());
        for x in game.players {
            assert!(names.contains(&x.name.as_str()));
        }
    }
}
