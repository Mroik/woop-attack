use serde::Serialize;
use std::time::UNIX_EPOCH;
use utoipa::ToSchema;

const ACTIVITY_CHUNK_SIZE: usize = 100;

#[derive(Debug, Serialize, Clone, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PlayerEvent {
    Shoot {
        shooter: String,
        from: (i16, i16),
        to: (i16, i16),
        target: String,
        timestamp: u64,
    },
    Move {
        player: String,
        from: (i16, i16),
        to: (i16, i16),
        timestamp: u64,
    },
    GenerateShield {
        player: String,
        zord_coord: (i16, i16),
        timestamp: u64,
    },
    IncreaseRange {
        player: String,
        zord_coord: (i16, i16),
        timestamp: u64,
    },
    DonatePoints {
        from: String,
        to: String,
        timestamp: u64,
    },
    BuildZord {
        player: String,
        zord_coord: (i16, i16),
        timestamp: u64,
    },
    TotemPoints {
        player: String,
        coord: (i16, i16),
        points: u16,
        timestamp: u64,
    },
    Respawn {
        player: String,
        coord: (i16, i16),
        timestamp: u64,
    },
    TotemSpawned {
        coord: (i16, i16),
        timestamp: u64,
    },
}

#[derive(Debug)]
pub struct Logger {
    data: Vec<PlayerEvent>,
}

impl Logger {
    pub fn shoot(&mut self, shooter: &str, from: (i16, i16), to: (i16, i16), target: &str) {
        self.data.push(PlayerEvent::Shoot {
            shooter: shooter.to_string(),
            from,
            to,
            target: target.to_string(),
            timestamp: unix_timestamp(),
        });
    }

    pub fn move_zord(&mut self, player: &str, from: (i16, i16), to: (i16, i16)) {
        self.data.push(PlayerEvent::Move {
            player: player.to_string(),
            from,
            to,
            timestamp: unix_timestamp(),
        });
    }

    pub fn generate_shield(&mut self, player: &str, zord_coord: (i16, i16)) {
        self.data.push(PlayerEvent::GenerateShield {
            player: player.to_string(),
            zord_coord,
            timestamp: unix_timestamp(),
        });
    }

    pub fn increase_range(&mut self, player: &str, zord_coord: (i16, i16)) {
        self.data.push(PlayerEvent::IncreaseRange {
            player: player.to_string(),
            zord_coord,
            timestamp: unix_timestamp(),
        });
    }

    pub fn donate_points(&mut self, from: &str, to: &str) {
        self.data.push(PlayerEvent::DonatePoints {
            from: from.to_string(),
            to: to.to_string(),
            timestamp: unix_timestamp(),
        });
    }

    pub fn build_zord(&mut self, player: &str, zord_coord: (i16, i16)) {
        self.data.push(PlayerEvent::BuildZord {
            player: player.to_string(),
            zord_coord,
            timestamp: unix_timestamp(),
        });
    }

    pub fn totem_points(&mut self, player: &str, coord: (i16, i16), points: u16) {
        self.data.push(PlayerEvent::TotemPoints {
            player: player.to_string(),
            coord,
            points,
            timestamp: unix_timestamp(),
        });
    }

    pub fn respawn(&mut self, player: &str, coord: (i16, i16)) {
        self.data.push(PlayerEvent::Respawn {
            player: player.to_string(),
            coord,
            timestamp: unix_timestamp(),
        });
    }

    pub fn totem_spawned(&mut self, coord: (i16, i16)) {
        self.data.push(PlayerEvent::TotemSpawned {
            coord,
            timestamp: unix_timestamp(),
        });
    }

    pub fn new() -> Logger {
        Self { data: Vec::new() }
    }

    pub fn get_chunk(&self, chunk: usize) -> Vec<PlayerEvent> {
        self.data
            .iter()
            .rev()
            .skip(ACTIVITY_CHUNK_SIZE * chunk)
            .take(ACTIVITY_CHUNK_SIZE)
            .cloned()
            .collect()
    }
}

fn unix_timestamp() -> u64 {
    UNIX_EPOCH.elapsed().unwrap().as_secs()
}
