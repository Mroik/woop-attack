use super::{board::Board, entity::Entity, player::Player, zord::Zord};

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

    // Check if any in cell, check player actions, check range, shoot
    fn player_shoot(&mut self, x_f: i32, y_f: i32, x_t: i32, y_t: i32) -> bool {
        let zord = self
            .board
            .board
            .iter_mut()
            .find(|entity| entity.is_coord(x_f, y_f) && entity.is_zord());
        if zord.is_none() {
            return false;
        }

        let zord = zord.unwrap().get_zord().unwrap();
        let range = zord.range;
        let distance = (x_f - x_t).abs().max((y_f - y_t).abs());
        if distance > range as i32 {
            return false;
        }

        let owner = self.players.iter_mut().find(|o| zord.owner == o.name.as_str()).unwrap();
        if owner.actions == 0 {
            return false;
        }
        owner.spend_action();

        self.board
            .board
            .iter_mut()
            .find(|entity| entity.is_coord(x_t, y_t) && entity.is_zord())
            .unwrap()
            .zord_hit();
        self.clear_dead();

        true
    }

    fn clear_dead(&mut self) {
        self.board.board.retain(|entity| {
            if !entity.is_zord() {
                return true;
            }
            let zord = entity.get_zord().unwrap();
            zord.hp > 0
        });
    }

    fn create_zord(&mut self, player: &Player, x: i32, y: i32) {
        let z = Entity::Zord(Zord::new(player, x, y));
        self.board.board.push(z);
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

    #[test]
    fn shoot_and_kill() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(&p, 0, 0);
        let p = game.players.get(1).cloned().unwrap();
        game.create_zord(&p, 1, 1);
        game.player_shoot(0, 0, 1, 1);
        let success = game.player_shoot(0, 0, 1, 1);
        assert!(success);
        assert_eq!(game.board.board.len(), 1);
    }

    #[test]
    fn shoot_and_out_of_range() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(&p, 0, 0);
        let p = game.players.get(1).cloned().unwrap();
        game.create_zord(&p, 10, 10);
        let success = game.player_shoot(0, 0, 10, 10);
        assert!(!success);
    }

    #[test]
    fn shoot_but_not_found() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(&p, 0, 0);
        let p = game.players.get(1).cloned().unwrap();
        game.create_zord(&p, 1, 1);
        let success = game.player_shoot(2, 2, 0, 0);
        assert!(!success);
    }
}
