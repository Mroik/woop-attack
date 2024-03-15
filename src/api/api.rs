use std::sync::Arc;

use super::message::{Reply, Request};
use crate::game::{error::WoopError, game::Game};
use serde::Serialize;
use std::sync::Mutex;
use warp::Filter;

#[derive(Serialize)]
enum ApiReply {
    Err(WoopError),
    Reply(Reply),
}

async fn start_api(game: Game) {
    let game = Arc::new(Mutex::new(game));
    let shoot = warp::path("shoot")
        .and(warp::body::json())
        .map(move |req: Request| {
            let mut game = game.lock().unwrap();
            match req {
                Request::Shoot(player, (x_f, y_f), (x_t, y_t)) => {
                    match game.player_shoot(player.as_str(), x_f, y_f, x_t, y_t) {
                        Ok(_) => warp::reply::json(&ApiReply::Reply(Reply::Ok)),
                        Err(err) => warp::reply::json(&ApiReply::Err(err)),
                    }
                }
                _ => warp::reply::json(&ApiReply::Err(WoopError::Generic)),
            }
        });

    let routes = warp::post().and(shoot);
    warp::serve(routes).run(([127, 0, 0, 1], 6969)).await;
}
