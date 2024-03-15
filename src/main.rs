mod api;
mod game;

use api::api::start_api;
use game::game::Game;
use std::sync::Mutex;

#[tokio::main]
async fn main() {
    let players = vec!["mroik", "fin", "mallory"]
        .iter()
        .map(|p| String::from(*p))
        .collect();
    let game = Mutex::new(Game::new(players));
    start_api(game).await;
}
