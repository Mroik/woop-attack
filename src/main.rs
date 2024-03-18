mod api;
pub mod config;
mod game;

use api::api::start_api;
use clap::Parser;
use clokwerk::{Job, Scheduler, TimeUnits};
use config::Config;
use game::game::Game;
use std::sync::{Arc, Mutex};

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

    let mut scheduler = Scheduler::with_tz(&config.timezone);
    scheduler.every(1.day()).at("6:00 am").run(move || {
        scheduler_game.lock().unwrap().new_day();
    });

    start_api(game).await;
}
