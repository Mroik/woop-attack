use serde::Serialize;

#[derive(Debug, Serialize)]
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
