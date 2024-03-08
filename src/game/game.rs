use std::time::Instant;

use super::{board::Board, entity::Entity, player::{Player, BASE_ACTIONS}, zord::{Zord, BASE_RANGE}};

const BASE_BOARD_SIZE: u16 = 10000;

struct Game {
    players: Vec<Player>,
    board: Board,
    start_of_day: Instant,
    day: u8,
}

impl Game {
    fn new(names: Vec<String>) -> Self {
        let players = names.iter().map(|name| Player::new(name)).collect();
        Game {
            players,
            board: Board::new(BASE_BOARD_SIZE),
            start_of_day: Instant::now(),
            day: 0,
        }
    }

    fn generate_shield(&mut self, x_f: i32, y_f: i32) -> bool {
        // Check if zord in cell
        let zord = self
            .board
            .board
            .iter_mut()
            .find(|entity| entity.is_coord(x_f, y_f) && entity.is_zord());
        if zord.is_none() {
            return false;
        }

        let zord = zord.unwrap();

        // Check if enough actions
        let name = zord.get_zord().unwrap().owner.as_str();
        let owner = self
            .players
            .iter_mut()
            .find(|o| name == o.name.as_str())
            .unwrap();
        if owner.actions == 0 {
            return false;
        }
        owner.spend_action();

        zord.zord_generate_shield()
    }

    fn player_shoot(&mut self, x_f: i32, y_f: i32, x_t: i32, y_t: i32) -> bool {
        // Check if zord in cell
        let zord = self
            .board
            .board
            .iter_mut()
            .find(|entity| entity.is_coord(x_f, y_f) && entity.is_zord());
        if zord.is_none() {
            return false;
        }

        // Check if within range
        let zord = zord.unwrap().get_zord().unwrap();
        let range = zord.range;
        let distance = (x_f - x_t).abs().max((y_f - y_t).abs());
        if distance > range as i32 {
            return false;
        }

        // Check if enough actions
        let owner = self
            .players
            .iter_mut()
            .find(|o| zord.owner == o.name.as_str())
            .unwrap();
        if owner.actions == 0 {
            return false;
        }
        owner.spend_action();

        // Shoot and cleanup
        self.board
            .board
            .iter_mut()
            .find(|entity| entity.is_coord(x_t, y_t) && entity.is_zord())
            .unwrap()
            .zord_hit();
        self.clear_dead();

        true
    }

    // Clean up dead zords
    fn clear_dead(&mut self) {
        self.board.board.retain(|entity| {
            if !entity.is_zord() {
                return true;
            }
            let zord = entity.get_zord().unwrap();
            zord.hp > 0
        });
    }

    // Add zord to the board
    fn create_zord(&mut self, player: &Player, x: i32, y: i32) {
        let z = Entity::Zord(Zord::new(player, x, y));
        self.board.board.push(z);
    }

    fn new_day(&mut self) {
        // Set new day
        self.start_of_day = Instant::now();
        self.day += 1;

        // Reset actions
        self.players.iter_mut().for_each(|player| player.actions = BASE_ACTIONS);

        // Remove shields and reset range
        self.board.board.iter_mut().for_each(|entity| {
            match entity {
                Entity::Zord(z) => {
                    z.range = BASE_RANGE;
                    z.shields = 0;
                },
                _ => (),
            }
        });
    }

    fn increase_range(&mut self, x: i32, y: i32) -> bool {
        // Check if zord in cell
        let zord = self
            .board
            .board
            .iter_mut()
            .find(|entity| entity.is_coord(x, y) && entity.is_zord());
        if zord.is_none() {
            return false;
        }

        let zord = zord.unwrap();

        // Check if enough actions
        let name = zord.get_zord().unwrap().owner.as_str();
        let owner = self
            .players
            .iter_mut()
            .find(|o| name == o.name.as_str())
            .unwrap();
        if owner.actions == 0 {
            return false;
        }
        owner.spend_action();

        zord.zord_increase_range()
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{player::BASE_ACTIONS, zord::BASE_RANGE};

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

    #[test]
    fn generate_shield() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(&p, 0, 0);
        let success = game.generate_shield(0, 0);
        let zord = game.board.board.first().unwrap().get_zord().unwrap();
        assert!(success);
        assert_eq!(zord.shields, 1);
    }

    #[test]
    fn generate_shield_no_actions() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(&p, 0, 0);
        for _ in 0..5 {
            game.generate_shield(0, 0);
        }
        let success = game.generate_shield(0, 0);
        let zord = game.board.board.first().unwrap().get_zord().unwrap();
        assert!(!success);
        assert_eq!(zord.shields, 5);
    }

    #[test]
    fn increase_range() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(&p, 0, 0);
        game.increase_range(0, 0);
        assert_eq!(game.board.board.get(0).unwrap().get_zord().unwrap().range, 6);
        assert_eq!(game.players.get(0).unwrap().actions, 4);
    }

    #[test]
    fn new_day() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(&p, 0, 0);
        game.new_day();
        game.generate_shield(0, 0);
        game.increase_range(0, 0);
        game.new_day();
        let zord = game.board.board.get(0).unwrap().get_zord().unwrap();
        assert_eq!(game.day, 2);
        assert_eq!(game.players.get(0).unwrap().actions, BASE_ACTIONS);
        assert_eq!(zord.range, BASE_RANGE);
        assert_eq!(zord.shields, 0);
    }
}
