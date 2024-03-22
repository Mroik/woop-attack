mod api;
pub mod config;
mod game;

use api::api::start_api;
use clap::Parser;
use clokwerk::{Job, Scheduler, TimeUnits};
use config::Config;
use game::game::Game;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Parser)]
struct Args {
    config: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = Config::read_file(args.config.as_str());

    let game = Arc::new(Mutex::new(Game::new(&config)));
    let scheduler_game = game.clone();

    let mut scheduler = Scheduler::new();
    scheduler.every(1.day()).at("6:00 am").run(move || {
        let mut game = scheduler_game.lock().unwrap();
        let start_stamp = game.start_of_day.duration_since(UNIX_EPOCH).unwrap();
        let current_stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let passed_day = start_stamp <= current_stamp;
        if (game.day > 0 && game.day < 29) || (game.day == 0 && passed_day) {
            game.new_day();
        }
    });
    let scheduler_handler = scheduler.watch_thread(Duration::from_secs(60));

    start_api(game).await;
    scheduler_handler.stop();
}
