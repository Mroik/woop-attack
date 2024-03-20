use serde::Deserialize;
use std::{
    fs::read,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Deserialize)]
struct RawConfig {
    players: String,
    start_of_game: u64,
}

pub struct Config {
    pub players: Vec<String>,
    pub start_of_game: SystemTime,
}

impl Config {
    pub fn read_file(path: &str) -> Config {
        let data = String::from_utf8(read(path).expect("Couldn't read config file"))
            .expect("Couldn't read config file");
        let config: RawConfig = toml::from_str(data.as_str()).unwrap();
        Self {
            players: config.players.split('|').map(|s| String::from(s)).collect(),
            start_of_game: UNIX_EPOCH + Duration::from_secs(config.start_of_game),
        }
    }
}
