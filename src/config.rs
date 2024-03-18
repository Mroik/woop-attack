use serde::Deserialize;
use std::{
    fs::read,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tzfile::Tz;

#[derive(Deserialize)]
struct RawConfig {
    players: Vec<String>,
    timezone: String,
    start_of_game: u64,
}

pub struct Config {
    pub players: Vec<String>,
    pub timezone: Tz,
    pub start_of_game: SystemTime,
}

impl Config {
    pub fn read_file(path: &str) -> Config {
        let data = String::from_utf8(read(path).expect("Couldn't read config file"))
            .expect("Couldn't read config file");
        let config: RawConfig = toml::from_str(data.as_str()).unwrap();
        Self {
            players: config.players,
            timezone: tzfile::Tz::named(config.timezone.as_str())
                .expect(format!("Couldn't find timezone {}", config.timezone.as_str()).as_str()),
            start_of_game: UNIX_EPOCH + Duration::from_secs(config.start_of_game),
        }
    }
}
