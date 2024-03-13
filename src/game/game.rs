use std::time::Instant;

use super::{
    board::Board,
    entity::Entity,
    player::{Player, BASE_ACTIONS},
    zord::{Zord, BASE_RANGE},
};

const BASE_BOARD_SIZE: i16 = 10000;
const GRACE_PERIOD: u64 = 60 * 60 * 3;
const NEW_ZORD_COST: u16 = 10;

#[derive(Debug)]
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

    fn generate_shield(&mut self, x_f: i16, y_f: i16) -> bool {
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

    fn donate_points(&mut self, from: &str, to: &str, amount: u16) -> bool {
        // Target exist
        if !self.players.iter().any(|p| p.name == to) {
            return false;
        }

        // Source exist
        let from_p = self.players.iter_mut().find(|p| p.name == from);
        if from_p.is_none() {
            return false;
        }

        // Has enough points
        let pf = from_p.unwrap();
        if amount > pf.points {
            return false;
        }

        // Has enough actions
        if pf.actions == 0 {
            return false;
        }
        pf.spend_action();

        pf.points -= amount;
        let pt = self.players.iter_mut().find(|p| p.name == to).unwrap();
        pt.points += amount;
        true
    }

    fn move_zord(&mut self, x_f: i16, y_f: i16, x_t: i16, y_t: i16) -> bool {
        // Check if empty
        if self.board.board.iter().any(|e| e.is_coord(x_t, y_t)) {
            return false;
        }

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

        // Check if within range
        let distance = (x_f - x_t).abs().max((y_f - y_t).abs());
        if distance > 1 {
            return false;
        }

        // Check if in bounds
        if x_t < 0 || x_t >= self.board.size || y_t < 0 || y_t >= self.board.size {
            return false;
        }

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
        zord.move_zord(x_t, y_t)
    }

    fn player_shoot(&mut self, x_f: i16, y_f: i16, x_t: i16, y_t: i16) -> bool {
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
        if distance > range as i16 {
            return false;
        }

        // Check grace period
        let delta_t = self.start_of_day.elapsed();
        if delta_t.as_secs() <= GRACE_PERIOD {
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
    fn create_zord(&mut self, player: &Player, x: i16, y: i16) {
        let z = Entity::Zord(Zord::new(player, x, y));
        self.board.board.push(z);
    }

    fn new_day(&mut self) {
        // Set new day
        self.start_of_day = Instant::now();
        self.day += 1;

        // Reset actions
        self.players
            .iter_mut()
            .for_each(|player| player.actions = BASE_ACTIONS);

        // Remove shields and reset range
        self.board.board.iter_mut().for_each(|entity| {
            if let Entity::Zord(z) = entity {
                z.range = BASE_RANGE;
                z.shields = 0;
            }
        });
    }

    fn increase_range(&mut self, x: i16, y: i16) -> bool {
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

    fn build_zord(&mut self, player: &str, x: i16, y: i16) -> bool {
        // Check if (x, y) is nearby another zord
        if !self
            .board
            .board
            .iter()
            .any(|z| z.is_zord() && z.distance(x, y) <= 1 && z.get_zord().unwrap().owner == player)
        {
            return false;
        }

        // Check if enough actions and points
        if self
            .players
            .iter()
            .any(|p| p.name == player && (p.actions == 0 || p.points < NEW_ZORD_COST))
        {
            return false;
        }

        let p = self.players.iter_mut().find(|p| p.name == player).unwrap();
        p.spend_action();
        p.points -= NEW_ZORD_COST;
        self.board.board.push(Entity::Zord(Zord::new(p, x, y)));
        true
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::game::{game::GRACE_PERIOD, player::BASE_ACTIONS, zord::BASE_RANGE};

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
        game.start_of_day = game
            .start_of_day
            .checked_sub(Duration::from_secs(GRACE_PERIOD + 1))
            .unwrap();
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
    fn shoot_during_grace_period() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(&p, 0, 0);
        let p = game.players.get(1).cloned().unwrap();
        game.create_zord(&p, 1, 1);
        game.player_shoot(0, 0, 1, 1);
        let success = game.player_shoot(0, 0, 1, 1);
        assert!(!success);
        assert_eq!(game.board.board.len(), 2);
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
        assert_eq!(
            game.board.board.first().unwrap().get_zord().unwrap().range,
            6
        );
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
        let zord = game.board.board.first().unwrap().get_zord().unwrap();
        assert_eq!(game.day, 2);
        assert_eq!(game.players.get(0).unwrap().actions, BASE_ACTIONS);
        assert_eq!(zord.range, BASE_RANGE);
        assert_eq!(zord.shields, 0);
    }

    #[test]
    fn move_zord() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(&p, 0, 0);
        let success = game.move_zord(0, 0, 1, 1);
        assert!(success);
        let z = game.board.board.first().unwrap().get_zord().unwrap();
        assert_eq!(z.x, 1);
        assert_eq!(z.y, 1);
    }

    #[test]
    fn move_zord_out_of_bounds() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(&p, 0, 0);
        let success = game.move_zord(0, 0, -1, -1);
        assert!(!success);
        let z = game.board.board.first().unwrap().get_zord().unwrap();
        assert_eq!(z.x, 0);
        assert_eq!(z.y, 0);
    }

    #[test]
    fn move_zord_out_of_range() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(&p, 0, 0);
        let success = game.move_zord(0, 0, 2, 2);
        assert!(!success);
        let z = game.board.board.first().unwrap().get_zord().unwrap();
        assert_eq!(z.x, 0);
        assert_eq!(z.y, 0);
    }

    #[test]
    fn move_zord_cell_occupied() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(&p, 0, 0);
        let p = game.players.get(1).cloned().unwrap();
        game.create_zord(&p, 1, 1);
        let success = game.move_zord(0, 0, 1, 1);
        assert!(!success);
        let z = game.board.board.first().unwrap().get_zord().unwrap();
        assert_eq!(z.x, 0);
        assert_eq!(z.y, 0);
    }

    #[test]
    fn donate_points() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        game.players
            .iter_mut()
            .find(|p| p.name == "mroik")
            .unwrap()
            .points = 100;
        let success = game.donate_points("mroik", "fin", 30);
        assert!(success);
        assert_eq!(
            game.players
                .iter()
                .find(|p| p.name == "mroik")
                .unwrap()
                .points,
            70
        );
        assert_eq!(
            game.players
                .iter()
                .find(|p| p.name == "fin")
                .unwrap()
                .points,
            30
        );
    }

    #[test]
    fn donate_points_amount_too_big() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        game.players
            .iter_mut()
            .find(|p| p.name == "mroik")
            .unwrap()
            .points = 10;
        let success = game.donate_points("mroik", "fin", 30);
        assert!(!success);
        assert_eq!(
            game.players
                .iter()
                .find(|p| p.name == "mroik")
                .unwrap()
                .points,
            10
        );
        assert_eq!(
            game.players
                .iter()
                .find(|p| p.name == "fin")
                .unwrap()
                .points,
            0
        );
    }

    #[test]
    fn build_zord() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        {
            let p = game.players.get(0).cloned().unwrap();
            game.create_zord(&p, 0, 0);
        }
        game.players
            .iter_mut()
            .find(|p| p.name == "mroik")
            .unwrap()
            .points = 10;

        let success = game.build_zord("mroik", 1, 1);
        let z = game.board.board.iter().find(|z| z.is_zord() && z.is_coord(1, 1)).unwrap().get_zord().unwrap();
        let p = game.players.get(0).unwrap();
        assert!(success);
        assert_eq!(p.actions, 4);
        assert_eq!(p.points, 0);
        assert_eq!(z.owner, "mroik");
    }

    #[test]
    fn build_zord_out_of_range() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        {
            let p = game.players.get(0).cloned().unwrap();
            game.create_zord(&p, 0, 0);
        }
        game.players
            .iter_mut()
            .find(|p| p.name == "mroik")
            .unwrap()
            .points = 10;

        let success = game.build_zord("mroik", 3, 3);
        let p = game.players.get(0).unwrap();
        assert!(!success);
        assert_eq!(p.actions, 5);
        assert_eq!(p.points, 10);
        assert_eq!(game.board.board.iter().len(), 1);
    }

    #[test]
    fn build_zord_not_enough_points() {
        let names = vec!["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        {
            let p = game.players.get(0).cloned().unwrap();
            game.create_zord(&p, 0, 0);
        }
        game.players
            .iter_mut()
            .find(|p| p.name == "mroik")
            .unwrap()
            .points = 9;

        let success = game.build_zord("mroik", 1, 1);
        let p = game.players.get(0).unwrap();
        assert!(!success);
        assert_eq!(p.actions, 5);
        assert_eq!(p.points, 9);
        assert_eq!(game.board.board.iter().len(), 1);
    }
}
