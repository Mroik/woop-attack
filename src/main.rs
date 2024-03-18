mod api;
mod game;

use api::api::start_api;
use clokwerk::{Job, Scheduler, TimeUnits};
use game::game::Game;
use std::sync::{Arc, Mutex};
use tzfile::Tz;

#[tokio::main]
async fn main() {
    let players = vec!["mroik", "fin", "mallory"]
        .iter()
        .map(|p| String::from(*p))
        .collect();

    let game = Arc::new(Mutex::new(Game::new(players)));
    let scheduler_game = game.clone();

    let timezone = Tz::named("Europe/Rome").unwrap();
    let mut scheduler = Scheduler::with_tz(&timezone);
    scheduler.every(1.day()).at("6:00 am").run(move || {
        scheduler_game.lock().unwrap().new_day();
    });

    start_api(game).await;
}
