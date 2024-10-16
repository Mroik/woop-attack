use super::{
    error::WoopError,
    log::Logger,
    player::{Player, BASE_ACTIONS},
    totem::Totem,
    zord::{Zord, BASE_RANGE},
};
use crate::config::Config;
use base64::{engine::general_purpose::URL_SAFE, Engine};
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, time::SystemTime};

const BASE_BOARD_SIZE: i16 = 140;
const GRACE_PERIOD: u64 = 60 * 60 * 3;
const NEW_ZORD_COST: u16 = 10;
const KILL_REWARD: u16 = 3;
const TOTEM_AURA: u16 = 5;
const TOTEM_REWARD: u16 = 50;
const ACTION_COST: u8 = 4;
const MAX_DONATION_PER_ACTION: u16 = 10;

#[derive(Debug)]
pub struct Game {
    pub players: HashMap<String, Player>,
    pub zords: Vec<Zord>,
    pub totems: (Totem, Totem),
    pub start_of_day: SystemTime,
    pub day: u8,
    pub auth: HashMap<String, String>,
    pub logged_actions: Logger,
}

impl Game {
    pub fn new(config: &Config) -> Self {
        let players: HashMap<String, Player> = config
            .players
            .iter()
            .map(|name| (name.clone(), Player::new(name)))
            .collect();

        let mut rng = rand::thread_rng();
        let mut auth = HashMap::new();
        players.iter().for_each(|(_, p)| {
            let mut hasher = Sha256::new();
            hasher.update(rng.gen::<[u8; 32]>());
            let data = hasher.finalize().to_vec();
            let mut password = URL_SAFE.encode(data);
            password.truncate(100);
            auth.insert(p.name.clone(), password);
        });

        Game {
            players,
            zords: Vec::new(),
            totems: (Totem::new(0, 0), Totem::new(0, 0)),
            start_of_day: SystemTime::now(),
            day: 0,
            auth,
            logged_actions: Logger::new(),
        }
    }

    pub fn authenticate(&self, username: &str, pass: &str) -> Result<(), WoopError> {
        match self.auth.get(username) {
            Some(p) if p.as_str() == pass => Ok(()),
            _ => Err(WoopError::AuthError),
        }
    }

    pub fn generate_shield(&mut self, player: &str, x: i16, y: i16) -> Result<(), WoopError> {
        // Check if zord in cell
        let zord = match self
            .zords
            .iter_mut()
            .find(|zord| zord.x == x && zord.y == y)
        {
            None => return WoopError::zord_not_found(x, y),
            Some(z) => z,
        };

        // Check if own zord
        let name = zord.owner.as_str();
        if name != player {
            return WoopError::not_owned(x, y);
        }

        // Check if enough actions
        let owner = self.players.get_mut(name).unwrap();
        if owner.actions == 0 {
            return WoopError::out_of_actions();
        }
        owner.spend_action(ACTION_COST);

        zord.generate_shield();
        self.logged_actions.generate_shield(player, (x, y));
        Ok(())
    }

    pub fn donate_points(&mut self, from: &str, to: &str, amount: u16) -> Result<(), WoopError> {
        // Limit donations to 10 points
        if amount > MAX_DONATION_PER_ACTION {
            return WoopError::donation_limit();
        }

        // Target exist
        if self.players.get(to).is_none() {
            return WoopError::player_not_found(to);
        }

        // Source exist
        let pf = match self.players.get_mut(from) {
            None => return WoopError::player_not_found(from),
            Some(p) => p,
        };

        if amount > pf.points {
            return WoopError::not_enough_points(pf.points, amount);
        }

        // Check if within range for donation (range for donations is shared with the range for
        // shooting
        let mut s_zord = self.zords.iter().filter(|zord| zord.owner.as_str() == from);
        let mut t_zord = self.zords.iter().filter(|zord| zord.owner.as_str() == to);

        let is_in_range = s_zord
            .any(|s| t_zord.any(|t| (s.x - t.x).abs().max((s.y - t.y).abs()) <= s.range as i16));
        if !is_in_range {
            return WoopError::donation_out_of_range();
        }

        // Has enough actions
        if pf.actions == 0 {
            return WoopError::out_of_actions();
        }
        pf.spend_action(ACTION_COST);

        pf.points -= amount;
        let pt = self.players.get_mut(to).unwrap();
        pt.points += amount;

        self.logged_actions.donate_points(from, to);
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
        if self.zords.iter().any(|zord| zord.x == x_t && zord.y == y_t) {
            return WoopError::cell_occupied(x_t, y_t);
        }

        // Check if zord in cell
        let zord = match self
            .zords
            .iter_mut()
            .find(|zord| zord.x == x_f && zord.y == y_f)
        {
            None => return WoopError::zord_not_found(x_f, y_f),
            Some(z) => z,
        };

        // Check if own zord
        if zord.owner.as_str() != player {
            return WoopError::not_owned(x_f, y_f);
        }

        let owner = self.players.get_mut(player).unwrap();

        // Check if within range
        let distance = (x_f - x_t).abs().max((y_f - y_t).abs());
        if distance > owner.actions as i16 {
            return WoopError::not_in_range(x_f, y_f, x_t, y_t);
        }

        // Check if in bounds
        if x_t < 0 || x_t >= BASE_BOARD_SIZE || y_t < 0 || y_t >= BASE_BOARD_SIZE {
            return WoopError::out_of_bounds(x_t, y_t);
        }

        // Check if enough actions
        if (owner.actions as i16) < distance {
            return WoopError::out_of_actions();
        }
        owner.spend_action(distance as u8);
        zord.set_coord(x_t, y_t);
        self.logged_actions
            .move_zord(player, (x_f, y_f), (x_t, y_t));
        Ok(())
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
        let zord = match self
            .zords
            .iter_mut()
            .find(|zord| zord.x == x_f && zord.y == y_f)
        {
            None => return WoopError::zord_not_found(x_f, y_f),
            Some(z) => z,
        };

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
        let owner = self.players.get_mut(player).unwrap();
        if owner.actions == 0 {
            return WoopError::out_of_actions();
        }

        // Check if target is your own
        let target = self
            .zords
            .iter_mut()
            .find(|zord| zord.x == x_t && zord.y == y_t)
            .unwrap();

        if target.owner.as_str() == player {
            return WoopError::own_zord();
        }

        owner.spend_action(ACTION_COST);

        // Shoot and cleanup
        let t_name = target.owner.clone();
        if target.hit() {
            owner.points += KILL_REWARD;
        }
        self.clear_dead();

        let has_zords = self
            .zords
            .iter()
            .filter(|zord| zord.owner == t_name)
            .count()
            > 0;
        if !has_zords {
            let t_player = self.players.get_mut(&t_name).unwrap();
            t_player.points = t_player.points * 2 / 3;
        }

        self.logged_actions
            .shoot(player, (x_f, y_f), (x_t, y_t), t_name.as_str());
        Ok(())
    }

    // Clean up dead zords
    fn clear_dead(&mut self) {
        self.zords.retain(|entity| entity.hp > 0);
    }

    // Add zord to the board
    fn create_zord(&mut self, player: &str, x: i16, y: i16) {
        let z = Zord::new(player, x, y);
        self.zords.push(z);
    }

    fn give_out_totem_points(&mut self) {
        let zords: Vec<&Zord> = self.zords.iter().collect();

        let mut points = |totem: &mut Totem| {
            let mut in_bounds = HashMap::new();
            let mut total = 0;
            zords.iter().for_each(|z| {
                if (totem.x - z.x).abs().max((totem.y - z.y).abs()) <= TOTEM_AURA as i16 {
                    match in_bounds.get(z.owner.as_str()) {
                        None => in_bounds.insert(z.owner.clone(), 1),
                        Some(v) => in_bounds.insert(z.owner.clone(), v + 1),
                    };
                    total += 1;
                }
            });

            for player in in_bounds.keys() {
                let many = in_bounds.get(player).unwrap();
                let p = self.players.get_mut(player).unwrap();
                p.points += TOTEM_REWARD * many / total;
                self.logged_actions.totem_points(
                    player.as_str(),
                    (totem.x, totem.y),
                    TOTEM_REWARD * many / total,
                );
            }
        };
        points(&mut self.totems.0);
        points(&mut self.totems.1);
    }

    fn respawn_players(&mut self) {
        let mut rng = thread_rng();
        let mut players: HashMap<String, i32> =
            self.players.keys().map(|name| (name.clone(), 0)).collect();

        self.zords.iter().for_each(|z| {
            let v = players.get(z.owner.as_str()).unwrap();
            players.insert(z.owner.clone(), v + 1);
        });

        let mut to_spawn: Vec<&String> = players
            .iter()
            .filter(|(_, many)| **many == 0)
            .map(|(p, _)| p)
            .collect();
        while !to_spawn.is_empty() {
            let player = to_spawn.remove(rng.gen_range(0..to_spawn.len()));
            let (x, y) = self.calculate_respawn_coordinates();
            self.create_zord(player.as_str(), x, y);
            self.logged_actions.respawn(player, (x, y));
        }
    }

    // Spawning totems before players gives a more interesting map disposition (this given the fact
    // that we also include totems in the algorithm to choose the spawn point for the players)
    pub fn new_day(&mut self) {
        // Set new day
        self.start_of_day = SystemTime::now();
        self.day += 1;

        self.give_out_totem_points();

        // Spawn totems at the beginning of the week
        if self.day % 7 == 1 {
            self.spawn_totems();
        }

        self.respawn_players();

        // Reset actions
        self.players
            .iter_mut()
            .for_each(|(_, player)| player.actions = BASE_ACTIONS);

        // Remove shields and reset range
        self.zords.iter_mut().for_each(|entity| {
            entity.range = BASE_RANGE;
            entity.shields = 0;
        });
    }

    pub fn increase_range(&mut self, player: &str, x: i16, y: i16) -> Result<(), WoopError> {
        // Check if zord in cell
        let zord = match self
            .zords
            .iter_mut()
            .find(|entity| entity.x == x && entity.y == y)
        {
            None => return WoopError::zord_not_found(x, y),
            Some(z) => z,
        };

        // Check if own zord
        if zord.owner.as_str() != player {
            return WoopError::not_owned(x, y);
        }

        // Check if enough actions
        let owner = self.players.get_mut(player).unwrap();
        if owner.actions == 0 {
            return WoopError::out_of_actions();
        }

        owner.spend_action(ACTION_COST / 2);
        zord.increase_range();
        self.logged_actions.increase_range(player, (x, y));
        Ok(())
    }

    pub fn build_zord(&mut self, player: &str, x: i16, y: i16) -> Result<(), WoopError> {
        fn distance(z: &Zord, x: i16, y: i16) -> i16 {
            (z.x - x).abs().max((z.y - y).abs())
        }

        // Check if (x, y) is nearby another zord
        if !self
            .zords
            .iter()
            .any(|z| distance(z, x, y) <= 1 && z.owner == player)
        {
            return WoopError::no_zord_nearby(x, y);
        }

        // Check if enough actions
        if self
            .players
            .iter()
            .any(|(_, p)| p.name == player && p.actions == 0)
        {
            return WoopError::out_of_actions();
        }

        let p = self.players.get(player);
        if p.is_none() {
            return WoopError::player_not_found(player);
        }

        // Check if enough points
        if p.unwrap().points < NEW_ZORD_COST {
            return WoopError::not_enough_points(p.unwrap().points, NEW_ZORD_COST);
        }

        let p = self.players.get_mut(player).unwrap();
        p.spend_action(ACTION_COST);
        p.points -= NEW_ZORD_COST;
        self.create_zord(player, x, y);
        self.zords.last_mut().unwrap().hit();

        self.logged_actions.build_zord(player, (x, y));
        Ok(())
    }

    // This is very slow for bigger boards
    fn calculate_respawn_coordinates(&self) -> (i16, i16) {
        let mut ris = (0, 0);
        let mut r_dis = 0;
        if self.zords.len() == 0 {
            return ris;
        }

        let zords_on_board: Vec<(i16, i16)> =
            self.zords.iter().map(|zord| (zord.x, zord.y)).collect();

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

    fn spawn_totems(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let t1 = (
                rng.gen_range(0..BASE_BOARD_SIZE),
                rng.gen_range(0..BASE_BOARD_SIZE),
            );
            let t2 = ((BASE_BOARD_SIZE - 1) - t1.0, (BASE_BOARD_SIZE - 1) - t1.1);

            let is_far_enough =
                (t1.0 - t2.0).abs().max((t1.1 - t2.1).abs()) as u16 > TOTEM_AURA * 2;

            if is_far_enough {
                self.totems = (Totem::new(t1.0, t1.1), Totem::new(t2.0, t2.1));
                self.logged_actions.totem_spawned((t1.0, t1.1));
                self.logged_actions.totem_spawned((t2.0, t2.1));
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Game;
    use crate::{
        config::Config,
        game::{
            game::{BASE_BOARD_SIZE, GRACE_PERIOD},
            player::BASE_ACTIONS,
            totem::Totem,
            zord::BASE_RANGE,
        },
    };
    use std::time::{Duration, SystemTime};

    fn generate_game() -> Game {
        let config = Config {
            players: ["mroik", "fin", "warden"]
                .iter()
                .map(|s| String::from(*s))
                .collect(),
            start_of_game: SystemTime::now(),
        };
        Game::new(&config)
    }

    #[test]
    fn new_game() {
        let game = generate_game();
        for (_, x) in game.players {
            assert!(["mroik", "fin", "warden"].contains(&x.name.as_str()));
        }
    }

    #[test]
    fn shoot_and_kill() {
        let mut game = generate_game();
        game.start_of_day = game
            .start_of_day
            .checked_sub(Duration::from_secs(GRACE_PERIOD + 1))
            .unwrap();
        game.create_zord("mroik", 0, 0);
        game.create_zord("fin", 1, 1);
        game.players.get_mut("fin").unwrap().points = 100;
        let _ = game.player_shoot("mroik", 0, 0, 1, 1);
        let success = game.player_shoot("mroik", 0, 0, 1, 1);

        let points = game.players.get("mroik").unwrap().points;
        let t_points = game.players.get("fin").unwrap().points;
        assert!(success.is_ok());
        assert_eq!(game.zords.len(), 1);
        assert_eq!(points, 3);
        assert_eq!(t_points, 100 * 2 / 3);
    }

    #[test]
    fn shoot_during_grace_period() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        game.create_zord("fin", 1, 1);
        let _ = game.player_shoot("mroik", 0, 0, 1, 1);
        let success = game.player_shoot("mroik", 0, 0, 1, 1);
        assert!(success.is_err());
        assert_eq!(game.zords.len(), 2);
        assert_eq!(game.players.get("mroik").unwrap().points, 0);
    }

    #[test]
    fn shoot_and_out_of_range() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        game.create_zord("fin", 10, 10);
        let success = game.player_shoot("mroik", 0, 0, 10, 10);
        assert!(success.is_err());
        assert_eq!(game.players.get("mroik").unwrap().points, 0);
    }

    #[test]
    fn shoot_but_not_found() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        game.create_zord("fin", 1, 1);
        let success = game.player_shoot("mroik", 2, 2, 0, 0);
        assert!(success.is_err());
        assert_eq!(game.players.get("mroik").unwrap().points, 0);
    }

    #[test]
    fn shoot_but_not_owned() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        game.create_zord("fin", 1, 1);
        let success = game.player_shoot("fin", 0, 0, 1, 1);
        assert!(success.is_err());
        assert_eq!(game.players.get("fin").unwrap().points, 0);
    }

    #[test]
    fn generate_shield() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        let success = game.generate_shield("mroik", 0, 0);
        let zord = game.zords.first().unwrap();
        assert!(success.is_ok());
        assert_eq!(zord.shields, 1);
    }

    #[test]
    fn generate_shield_no_actions() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        for _ in 0..5 {
            let _ = game.generate_shield("mroik", 0, 0);
        }
        let success = game.generate_shield("mroik", 0, 0);
        let zord = game.zords.first().unwrap();
        assert!(success.is_err());
        assert_eq!(zord.shields, 5);
    }

    #[test]
    fn increase_range() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        let _ = game.increase_range("mroik", 0, 0);
        assert_eq!(game.zords.first().unwrap().range, 6);
        assert_eq!(game.players.get("mroik").unwrap().actions, 18);
    }

    #[test]
    fn new_day() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        game.new_day();
        let _ = game.generate_shield("mroik", 0, 0);
        let _ = game.increase_range("mroik", 0, 0);
        game.new_day();
        let zord = game.zords.first().unwrap();
        assert_eq!(game.day, 2);
        assert_eq!(game.players.get("mroik").unwrap().actions, BASE_ACTIONS);
        assert_eq!(zord.range, BASE_RANGE);
        assert_eq!(zord.shields, 0);
        assert_eq!(game.zords.len(), 3);
    }

    #[test]
    fn move_zord() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        let success = game.move_zord("mroik", 0, 0, 1, 1);
        assert!(success.is_ok());
        let z = game.zords.first().unwrap();
        assert_eq!(z.x, 1);
        assert_eq!(z.y, 1);
    }

    #[test]
    fn move_zord_out_of_bounds() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        let success = game.move_zord("mroik", 0, 0, -1, -1);
        assert!(success.is_err());
        let z = game.zords.first().unwrap();
        assert_eq!(z.x, 0);
        assert_eq!(z.y, 0);
    }

    #[test]
    fn move_zord_out_of_range() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        let success = game.move_zord("mroik", 0, 0, 21, 2);
        assert!(success.is_err());
        let z = game.zords.first().unwrap();
        assert_eq!(z.x, 0);
        assert_eq!(z.y, 0);
    }

    #[test]
    fn move_zord_cell_occupied() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        game.create_zord("fin", 1, 1);
        let success = game.move_zord("mroik", 0, 0, 1, 1);
        assert!(success.is_err());
        let z = game.zords.first().unwrap();
        assert_eq!(z.x, 0);
        assert_eq!(z.y, 0);
    }

    #[test]
    fn donate_points() {
        let mut game = generate_game();
        game.players.get_mut("mroik").unwrap().points = 100;
        game.create_zord("mroik", 0, 0);
        game.create_zord("fin", 1, 1);
        let success = game.donate_points("mroik", "fin", 10);
        assert!(success.is_ok());
        assert_eq!(game.players.get("mroik").unwrap().points, 90);
        assert_eq!(game.players.get("fin").unwrap().points, 10);
    }

    #[test]
    fn donate_points_amount_too_big() {
        let mut game = generate_game();
        game.players.get_mut("mroik").unwrap().points = 10;
        let success = game.donate_points("mroik", "fin", 30);
        assert!(success.is_err());
        assert_eq!(game.players.get("mroik").unwrap().points, 10);
        assert_eq!(game.players.get("fin").unwrap().points, 0);
    }

    #[test]
    fn build_zord() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        game.players.get_mut("mroik").unwrap().points = 10;

        let success = game.build_zord("mroik", 1, 1);
        let z = game.zords.iter().find(|z| z.x == 1 && z.y == 1).unwrap();
        let p = game.players.get("mroik").unwrap();
        assert!(success.is_ok());
        assert_eq!(p.actions, 16);
        assert_eq!(p.points, 0);
        assert_eq!(z.owner, "mroik");
    }

    #[test]
    fn build_zord_out_of_range() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        game.players.get_mut("mroik").unwrap().points = 10;

        let success = game.build_zord("mroik", 3, 3);
        let p = game.players.get("mroik").unwrap();
        assert!(success.is_err());
        assert_eq!(p.actions, 20);
        assert_eq!(p.points, 10);
        assert_eq!(game.zords.iter().len(), 1);
    }

    #[test]
    fn build_zord_not_enough_points() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        game.players.get_mut("mroik").unwrap().points = 9;

        let success = game.build_zord("mroik", 1, 1);
        let p = game.players.get("mroik").unwrap();
        assert!(success.is_err());
        assert_eq!(p.actions, 20);
        assert_eq!(p.points, 9);
        assert_eq!(game.zords.iter().len(), 1);
    }

    #[test]
    fn give_out_points() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        game.totems = (
            Totem::new(0, 1),
            Totem::new(BASE_BOARD_SIZE - 1, BASE_BOARD_SIZE - 1),
        );
        game.give_out_totem_points();
        assert_eq!(game.players.get("mroik").unwrap().points, 50);
    }

    #[test]
    fn give_out_points_double() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        game.create_zord("fin", 0, 1);
        game.totems = (
            Totem::new(2, 2),
            Totem::new(BASE_BOARD_SIZE - 1, BASE_BOARD_SIZE - 1),
        );
        game.give_out_totem_points();
        assert_eq!(game.players.get("mroik").unwrap().points, 25);
        assert_eq!(game.players.get("fin").unwrap().points, 25);
    }

    #[test]
    fn give_out_out_of_range() {
        let mut game = generate_game();
        game.create_zord("mroik", 0, 0);
        game.totems = (
            Totem::new(BASE_BOARD_SIZE - 2, BASE_BOARD_SIZE - 2),
            Totem::new(BASE_BOARD_SIZE - 1, BASE_BOARD_SIZE - 1),
        );
        game.give_out_totem_points();
        assert_eq!(game.players.get("mroik").unwrap().points, 0);
    }

    #[test]
    fn respawn() {
        let mut game = generate_game();
        game.respawn_players();
        assert_eq!(game.zords.len(), 3);
    }
}
