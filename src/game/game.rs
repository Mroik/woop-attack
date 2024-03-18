use super::{
    board::Board,
    entity::Entity,
    error::WoopError,
    player::{Player, BASE_ACTIONS},
    totem::Totem,
    zord::{Zord, BASE_RANGE},
};
use std::{collections::HashMap, time::SystemTime};

const BASE_BOARD_SIZE: i16 = 70;
const GRACE_PERIOD: u64 = 60 * 60 * 3;
const NEW_ZORD_COST: u16 = 10;
const KILL_REWARD: u16 = 3;
const TOTEM_AURA: u16 = 2;
const TOTEM_REWARD: u16 = 50;

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub board: Board,
    pub start_of_day: SystemTime,
    pub day: u8,
}

impl Game {
    pub fn new(names: Vec<String>) -> Self {
        let players = names.iter().map(|name| Player::new(name)).collect();
        Game {
            players,
            board: Board::new(BASE_BOARD_SIZE),
            start_of_day: SystemTime::now(),
            day: 0,
        }
    }

    pub fn generate_shield(&mut self, player: &str, x_f: i16, y_f: i16) -> Result<(), WoopError> {
        // Check if zord in cell
        let zord = self
            .board
            .board
            .iter_mut()
            .find(|entity| entity.is_coord(x_f, y_f) && entity.is_zord());
        if zord.is_none() {
            return WoopError::zord_not_found(x_f, y_f);
        }

        let zord = zord.unwrap();

        // Check if own zord
        if zord.get_zord().unwrap().owner.as_str() != player {
            return WoopError::not_owned(x_f, y_f);
        }

        // Check if enough actions
        let name = zord.get_zord().unwrap().owner.as_str();
        let owner = self
            .players
            .iter_mut()
            .find(|o| name == o.name.as_str())
            .unwrap();
        if owner.actions == 0 {
            return WoopError::out_of_actions();
        }
        owner.spend_action();

        if zord.zord_generate_shield() {
            Ok(())
        } else {
            WoopError::generic()
        }
    }

    pub fn donate_points(&mut self, from: &str, to: &str, amount: u16) -> Result<(), WoopError> {
        // Target exist
        if !self.players.iter().any(|p| p.name == to) {
            return WoopError::player_not_found(to);
        }

        // Source exist
        let from_p = self.players.iter_mut().find(|p| p.name == from);
        if from_p.is_none() {
            return WoopError::player_not_found(from);
        }

        // Has enough points
        let pf = from_p.unwrap();
        if amount > pf.points {
            return WoopError::not_enough_points(pf.points, amount);
        }

        // Has enough actions
        if pf.actions == 0 {
            return WoopError::out_of_actions();
        }
        pf.spend_action();

        pf.points -= amount;
        let pt = self.players.iter_mut().find(|p| p.name == to).unwrap();
        pt.points += amount;
        Ok(())
    }

    pub fn move_zord(
        &mut self,
        player: &str,
        x_f: i16,
        y_f: i16,
        x_t: i16,
        y_t: i16,
    ) -> Result<(), WoopError> {
        // Check if empty
        if self.board.board.iter().any(|e| e.is_coord(x_t, y_t)) {
            return WoopError::cell_occupied(x_t, y_t);
        }

        // Check if zord in cell
        let zord = self
            .board
            .board
            .iter_mut()
            .find(|entity| entity.is_coord(x_f, y_f) && entity.is_zord());
        if zord.is_none() {
            return WoopError::zord_not_found(x_f, y_f);
        }
        let zord = zord.unwrap();

        // Check if own zord
        if zord.get_zord().unwrap().owner.as_str() != player {
            return WoopError::not_owned(x_f, y_f);
        }

        // Check if within range
        let distance = (x_f - x_t).abs().max((y_f - y_t).abs());
        if distance > 1 {
            return WoopError::not_in_range(x_f, y_f, x_t, y_t);
        }

        // Check if in bounds
        if x_t < 0 || x_t >= self.board.size || y_t < 0 || y_t >= self.board.size {
            return WoopError::out_of_bounds(x_t, y_t);
        }

        // Check if enough actions
        let name = zord.get_zord().unwrap().owner.as_str();
        let owner = self
            .players
            .iter_mut()
            .find(|o| name == o.name.as_str())
            .unwrap();
        if owner.actions == 0 {
            return WoopError::out_of_actions();
        }
        owner.spend_action();
        if zord.move_zord(x_t, y_t) {
            Ok(())
        } else {
            WoopError::generic()
        }
    }

    pub fn player_shoot(
        &mut self,
        player: &str,
        x_f: i16,
        y_f: i16,
        x_t: i16,
        y_t: i16,
    ) -> Result<(), WoopError> {
        // Check if zord in cell
        let zord = self
            .board
            .board
            .iter_mut()
            .find(|entity| entity.is_coord(x_f, y_f) && entity.is_zord());
        if zord.is_none() {
            return WoopError::zord_not_found(x_f, y_f);
        }

        let zord = zord.unwrap().get_zord().unwrap();

        // Check if own zord
        if zord.owner != player {
            return WoopError::not_owned(x_f, y_f);
        }

        // Check if within range
        let range = zord.range;
        let distance = (x_f - x_t).abs().max((y_f - y_t).abs());
        if distance > range as i16 {
            return WoopError::not_in_range(x_f, y_f, x_t, y_t);
        }

        // Check grace period
        let delta_t = self.start_of_day.elapsed();
        if delta_t.unwrap().as_secs() <= GRACE_PERIOD {
            return WoopError::within_grace_period();
        }

        // Check if enough actions
        let owner = self
            .players
            .iter_mut()
            .find(|o| zord.owner == o.name.as_str())
            .unwrap();
        if owner.actions == 0 {
            return WoopError::out_of_actions();
        }
        owner.spend_action();

        let target = self
            .board
            .board
            .iter_mut()
            .find(|entity| entity.is_coord(x_t, y_t) && entity.is_zord())
            .unwrap();

        // Shoot and cleanup
        let mut t_name = String::new();
        let mut should_sort = false;
        if target.zord_hit() {
            owner.points += KILL_REWARD;
            should_sort = true;
            t_name = target.get_zord().unwrap().owner.clone();
        }
        self.clear_dead();

        let has_zords = self
            .board
            .board
            .iter()
            .filter(|z| z.is_zord() && z.get_zord().unwrap().owner == t_name)
            .count()
            > 0;
        if !t_name.is_empty() && !has_zords {
            let t_player = self.players.iter_mut().find(|p| p.name == t_name).unwrap();
            t_player.points = t_player.points * 2 / 3;
        }

        if should_sort {
            self.players.sort_by_key(|p| p.points);
            self.players.reverse();
        }

        Ok(())
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
    fn create_zord(&mut self, player: &str, x: i16, y: i16) {
        let z = Entity::Zord(Zord::new(player, x, y));
        self.board.board.push(z);
    }

    fn give_out_totem_points(&mut self) {
        let totems = self.board.board.iter().filter(|t| !t.is_zord()).map(|t| {
            let tot = t.get_totem().unwrap();
            (tot.x, tot.y)
        });

        for (x_t, y_t) in totems {
            let mut in_bounds = HashMap::new();
            let mut total = 0;
            self.board
                .board
                .iter()
                .filter(|z| z.is_zord())
                .map(|z| z.get_zord().unwrap())
                .for_each(|z| {
                    if (x_t - z.x).abs().max((y_t - z.y).abs()) <= TOTEM_AURA as i16 {
                        match in_bounds.get(z.owner.as_str()) {
                            None => in_bounds.insert(z.owner.clone(), 1),
                            Some(v) => in_bounds.insert(z.owner.clone(), v + 1),
                        };
                        total += 1;
                    }
                });

            for player in in_bounds.keys() {
                let many = in_bounds.get(player).unwrap();
                let p = self.players.iter_mut().find(|p| p.name == *player).unwrap();
                p.points += TOTEM_REWARD / total * many;
            }
        }
    }

    fn respawn_players(&mut self) {
        let mut players = HashMap::new();
        for player in self.players.iter() {
            players.insert(String::from(player.name.as_str()), 0);
        }

        self.board
            .board
            .iter()
            .filter(|z| z.is_zord())
            .map(|z| z.get_zord().unwrap())
            .for_each(|z| {
                let v = players.get(z.owner.as_str()).unwrap();
                players.insert(z.owner.clone(), v + 1);
            });

        let to_spawn = players.iter().filter(|(_, many)| **many == 0);
        for (player, _) in to_spawn {
            let (x, y) = self.calculate_respawn_coordinates();
            self.create_zord(player.as_str(), x, y);
        }
    }

    pub fn new_day(&mut self) {
        // Set new day
        self.start_of_day = SystemTime::now();
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

        self.give_out_totem_points();
        self.respawn_players();
    }

    pub fn increase_range(&mut self, player: &str, x: i16, y: i16) -> Result<(), WoopError> {
        // Check if zord in cell
        let zord = self
            .board
            .board
            .iter_mut()
            .find(|entity| entity.is_coord(x, y) && entity.is_zord());
        if zord.is_none() {
            return WoopError::zord_not_found(x, y);
        }

        let zord = zord.unwrap();

        // Check if own zord
        if zord.get_zord().unwrap().owner.as_str() != player {
            return WoopError::not_owned(x, y);
        }

        // Check if enough actions
        let name = zord.get_zord().unwrap().owner.as_str();
        let owner = self
            .players
            .iter_mut()
            .find(|o| name == o.name.as_str())
            .unwrap();
        if owner.actions == 0 {
            return WoopError::out_of_actions();
        }
        owner.spend_action();
        if zord.zord_increase_range() {
            Ok(())
        } else {
            WoopError::generic()
        }
    }

    pub fn build_zord(&mut self, player: &str, x: i16, y: i16) -> Result<(), WoopError> {
        // Check if (x, y) is nearby another zord
        if !self
            .board
            .board
            .iter()
            .any(|z| z.is_zord() && z.distance(x, y) <= 1 && z.get_zord().unwrap().owner == player)
        {
            return WoopError::no_zord_nearby(x, y);
        }

        // Check if enough actions
        if self
            .players
            .iter()
            .any(|p| p.name == player && p.actions == 0)
        {
            return WoopError::out_of_actions();
        }

        let p = self.players.iter().find(|p| p.name == player);
        if p.is_none() {
            return WoopError::player_not_found(player);
        }

        // Check if enough points
        if p.unwrap().points < NEW_ZORD_COST {
            return WoopError::not_enough_points(p.unwrap().points, NEW_ZORD_COST);
        }

        let p = self.players.iter_mut().find(|p| p.name == player).unwrap();
        p.spend_action();
        p.points -= NEW_ZORD_COST;
        self.board
            .board
            .push(Entity::Zord(Zord::new(p.name.as_str(), x, y)));
        Ok(())
    }

    fn create_totem(&mut self, x: i16, y: i16) -> Result<(), WoopError> {
        // Check if cell is empty
        if self.board.board.iter().any(|e| e.is_coord(x, y)) {
            return WoopError::cell_occupied(x, y);
        }

        self.board.board.push(Entity::Totem(Totem::new(x, y)));
        Ok(())
    }

    /// This is very slow for bigger boards
    fn calculate_respawn_coordinates(&self) -> (i16, i16) {
        let mut ris = (0, 0);
        let mut r_dis = 0;
        if self.board.board.iter().filter(|z| z.is_zord()).count() == 0 {
            return ris;
        }

        let zords_on_board: Vec<(i16, i16)> = self
            .board
            .board
            .iter()
            .filter(|z| z.is_zord())
            .map(|z| {
                let zord = z.get_zord().unwrap();
                (zord.x, zord.y)
            })
            .collect();

        for y_f in 0..BASE_BOARD_SIZE {
            for x_f in 0..BASE_BOARD_SIZE {
                let distance = zords_on_board
                    .iter()
                    .map(|(x, y)| ((x_f - x).abs().max((y_f - y).abs()), x, y))
                    .min()
                    .unwrap();
                if r_dis < distance.0 {
                    ris = (x_f, y_f);
                    r_dis = distance.0;
                }
            }
        }
        ris
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::game::{game::GRACE_PERIOD, player::BASE_ACTIONS, zord::BASE_RANGE};

    use super::Game;

    #[test]
    fn new_game() {
        let names = ["mroik", "fin", "warden"];
        let game = Game::new(names.iter().map(|name| name.to_string()).collect());
        for x in game.players {
            assert!(names.contains(&x.name.as_str()));
        }
    }

    #[test]
    fn shoot_and_kill() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        game.start_of_day = game
            .start_of_day
            .checked_sub(Duration::from_secs(GRACE_PERIOD + 1))
            .unwrap();
        game.create_zord("mroik", 0, 0);
        game.create_zord("fin", 1, 1);
        game.players
            .iter_mut()
            .find(|p| p.name.as_str() == "fin")
            .unwrap()
            .points = 100;
        let _ = game.player_shoot("mroik", 0, 0, 1, 1);
        let success = game.player_shoot("mroik", 0, 0, 1, 1);

        // Inverted since it is sorted on kill
        let t_points = game.players.get(0).unwrap().points;
        let points = game.players.get(1).unwrap().points;
        assert!(success.is_ok());
        assert_eq!(game.board.board.len(), 1);
        assert_eq!(points, 3);
        assert_eq!(t_points, 100 * 2 / 3);
    }

    #[test]
    fn shoot_during_grace_period() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 0);
        let p = game.players.get(1).cloned().unwrap();
        game.create_zord(p.name.as_str(), 1, 1);
        let _ = game.player_shoot("mroik", 0, 0, 1, 1);
        let success = game.player_shoot("mroik", 0, 0, 1, 1);
        assert!(success.is_err());
        assert_eq!(game.board.board.len(), 2);
        assert_eq!(p.points, 0);
    }

    #[test]
    fn shoot_and_out_of_range() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 0);
        let p = game.players.get(1).cloned().unwrap();
        game.create_zord(p.name.as_str(), 10, 10);
        let success = game.player_shoot("mroik", 0, 0, 10, 10);
        assert!(success.is_err());
        assert_eq!(p.points, 0);
    }

    #[test]
    fn shoot_but_not_found() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 0);
        let p = game.players.get(1).cloned().unwrap();
        game.create_zord(p.name.as_str(), 1, 1);
        let success = game.player_shoot("mroik", 2, 2, 0, 0);
        assert!(success.is_err());
        assert_eq!(p.points, 0);
    }

    #[test]
    fn shoot_but_not_owned() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 0);
        let p = game.players.get(1).cloned().unwrap();
        game.create_zord(p.name.as_str(), 1, 1);
        let success = game.player_shoot("fin", 0, 0, 1, 1);
        assert!(success.is_err());
        assert_eq!(p.points, 0);
    }

    #[test]
    fn generate_shield() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        game.create_zord("mroik", 0, 0);
        let success = game.generate_shield("mroik", 0, 0);
        let zord = game.board.board.first().unwrap().get_zord().unwrap();
        assert!(success.is_ok());
        assert_eq!(zord.shields, 1);
    }

    #[test]
    fn generate_shield_no_actions() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        game.create_zord("mroik", 0, 0);
        for _ in 0..5 {
            let _ = game.generate_shield("mroik", 0, 0);
        }
        let success = game.generate_shield("mroik", 0, 0);
        let zord = game.board.board.first().unwrap().get_zord().unwrap();
        assert!(success.is_err());
        assert_eq!(zord.shields, 5);
    }

    #[test]
    fn increase_range() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        game.create_zord("mroik", 0, 0);
        let _ = game.increase_range("mroik", 0, 0);
        assert_eq!(
            game.board.board.first().unwrap().get_zord().unwrap().range,
            6
        );
        assert_eq!(game.players.get(0).unwrap().actions, 4);
    }

    #[test]
    fn new_day() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        game.create_zord("mroik", 0, 0);
        game.new_day();
        let _ = game.generate_shield("mroik", 0, 0);
        let _ = game.increase_range("mroik", 0, 0);
        game.new_day();
        let zord = game.board.board.first().unwrap().get_zord().unwrap();
        assert_eq!(game.day, 2);
        assert_eq!(game.players.get(0).unwrap().actions, BASE_ACTIONS);
        assert_eq!(zord.range, BASE_RANGE);
        assert_eq!(zord.shields, 0);
        assert_eq!(game.board.board.len(), 3);
    }

    #[test]
    fn move_zord() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        game.create_zord("mroik", 0, 0);
        let success = game.move_zord("mroik", 0, 0, 1, 1);
        assert!(success.is_ok());
        let z = game.board.board.first().unwrap().get_zord().unwrap();
        assert_eq!(z.x, 1);
        assert_eq!(z.y, 1);
    }

    #[test]
    fn move_zord_out_of_bounds() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 0);
        let success = game.move_zord("mroik", 0, 0, -1, -1);
        assert!(success.is_err());
        let z = game.board.board.first().unwrap().get_zord().unwrap();
        assert_eq!(z.x, 0);
        assert_eq!(z.y, 0);
    }

    #[test]
    fn move_zord_out_of_range() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 0);
        let success = game.move_zord("mroik", 0, 0, 2, 2);
        assert!(success.is_err());
        let z = game.board.board.first().unwrap().get_zord().unwrap();
        assert_eq!(z.x, 0);
        assert_eq!(z.y, 0);
    }

    #[test]
    fn move_zord_cell_occupied() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 0);
        let p = game.players.get(1).cloned().unwrap();
        game.create_zord(p.name.as_str(), 1, 1);
        let success = game.move_zord("mroik", 0, 0, 1, 1);
        assert!(success.is_err());
        let z = game.board.board.first().unwrap().get_zord().unwrap();
        assert_eq!(z.x, 0);
        assert_eq!(z.y, 0);
    }

    #[test]
    fn donate_points() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        game.players
            .iter_mut()
            .find(|p| p.name == "mroik")
            .unwrap()
            .points = 100;
        let success = game.donate_points("mroik", "fin", 30);
        assert!(success.is_ok());
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
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        game.players
            .iter_mut()
            .find(|p| p.name == "mroik")
            .unwrap()
            .points = 10;
        let success = game.donate_points("mroik", "fin", 30);
        assert!(success.is_err());
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
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 0);
        game.players
            .iter_mut()
            .find(|p| p.name == "mroik")
            .unwrap()
            .points = 10;

        let success = game.build_zord("mroik", 1, 1);
        let z = game
            .board
            .board
            .iter()
            .find(|z| z.is_zord() && z.is_coord(1, 1))
            .unwrap()
            .get_zord()
            .unwrap();
        let p = game.players.get(0).unwrap();
        assert!(success.is_ok());
        assert_eq!(p.actions, 4);
        assert_eq!(p.points, 0);
        assert_eq!(z.owner, "mroik");
    }

    #[test]
    fn build_zord_out_of_range() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 0);
        game.players
            .iter_mut()
            .find(|p| p.name == "mroik")
            .unwrap()
            .points = 10;

        let success = game.build_zord("mroik", 3, 3);
        let p = game.players.get(0).unwrap();
        assert!(success.is_err());
        assert_eq!(p.actions, 5);
        assert_eq!(p.points, 10);
        assert_eq!(game.board.board.iter().len(), 1);
    }

    #[test]
    fn build_zord_not_enough_points() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 0);
        game.players
            .iter_mut()
            .find(|p| p.name == "mroik")
            .unwrap()
            .points = 9;

        let success = game.build_zord("mroik", 1, 1);
        let p = game.players.get(0).unwrap();
        assert!(success.is_err());
        assert_eq!(p.actions, 5);
        assert_eq!(p.points, 9);
        assert_eq!(game.board.board.iter().len(), 1);
    }

    #[test]
    fn give_out_points() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 0);
        let _ = game.create_totem(1, 1);
        game.give_out_totem_points();
        assert_eq!(
            game.players
                .iter()
                .find(|p| p.name == "mroik")
                .unwrap()
                .points,
            50
        );
    }

    #[test]
    fn give_out_points_double() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 0);
        let p = game.players.get(1).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 1);
        let _ = game.create_totem(2, 2);
        game.give_out_totem_points();
        assert_eq!(
            game.players
                .iter()
                .find(|p| p.name == "mroik")
                .unwrap()
                .points,
            25
        );
        assert_eq!(
            game.players
                .iter()
                .find(|p| p.name == "fin")
                .unwrap()
                .points,
            25
        );
    }

    #[test]
    fn give_out_out_of_range() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        let p = game.players.get(0).cloned().unwrap();
        game.create_zord(p.name.as_str(), 0, 0);
        let _ = game.create_totem(100, 100);
        game.give_out_totem_points();
        assert_eq!(
            game.players
                .iter()
                .find(|p| p.name == "mroik")
                .unwrap()
                .points,
            0
        );
    }

    #[test]
    fn respawn() {
        let names = ["mroik", "fin", "warden"];
        let mut game = Game::new(names.iter().map(|name| name.to_string()).collect());
        game.respawn_players();
        assert_eq!(game.board.board.len(), 3);
    }
}
