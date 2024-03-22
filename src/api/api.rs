use super::message::{ApiReply, Reply, Request};
use crate::game::game::Game;
use std::convert::Infallible;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::UNIX_EPOCH;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Json;
use warp::Filter;
use warp::Reply as WarpReply;

async fn handle_rejection(_: Rejection) -> Result<impl WarpReply, Infallible> {
    let msg = "Incorrect interaction with the api. Check method, endpoint and JSON data";
    let json = warp::reply::json(&ApiReply::Error(String::from(msg)));
    Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST))
}

fn get_game_status(game: &Game) -> Result<Json, ()> {
    match game.day {
        0 => Ok(warp::reply::json(&ApiReply::Error(
            "Game hasn't started yet".to_string(),
        ))),
        29 => Ok(warp::reply::json(&ApiReply::Error(
            "Game has ended".to_string(),
        ))),
        _ => Err(()),
    }
}

pub async fn start_api(game: Arc<Mutex<Game>>) {
    let shoot_game = game.clone();
    let move_game = game.clone();
    let shield_game = game.clone();
    let increase_game = game.clone();
    let donate_game = game.clone();
    let build_game = game.clone();
    let map_game = game.clone();
    let leaderboard_game = game.clone();
    let day_game = game.clone();

    let shoot_action = warp::path("shoot")
        .and(warp::body::json())
        .and(warp::header("username"))
        .and(warp::header("token"))
        .map(move |req: Request, username: String, pass: String| {
            let mut game = shoot_game.lock().unwrap();
            if let Err(err) = game.authenticate(username.as_str(), pass.as_str()) {
                return warp::reply::json(&ApiReply::Error(err.to_string()));
            }

            if let Ok(resp) = get_game_status(&game) {
                return resp;
            }
            match req {
                Request::DoubleCoord {
                    from: (x_f, y_f),
                    to: (x_t, y_t),
                } => match game.player_shoot(username.as_str(), x_f, y_f, x_t, y_t) {
                    Ok(_) => warp::reply::json(&ApiReply::Data(Reply::Ok)),
                    Err(err) => warp::reply::json(&ApiReply::Error(err.to_string())),
                },
                _ => warp::reply::json(&ApiReply::Error(String::from("Wrong JSON data"))),
            }
        });

    let move_action = warp::path("move")
        .and(warp::body::json())
        .and(warp::header("username"))
        .and(warp::header("token"))
        .map(move |req: Request, username: String, pass: String| {
            let mut game = move_game.lock().unwrap();
            if let Err(err) = game.authenticate(username.as_str(), pass.as_str()) {
                return warp::reply::json(&ApiReply::Error(err.to_string()));
            }

            if let Ok(resp) = get_game_status(&game) {
                return resp;
            }
            match req {
                Request::DoubleCoord {
                    from: (x_f, y_f),
                    to: (x_t, y_t),
                } => match game.move_zord(username.as_str(), x_f, y_f, x_t, y_t) {
                    Ok(_) => warp::reply::json(&ApiReply::Data(Reply::Ok)),
                    Err(err) => warp::reply::json(&ApiReply::Error(err.to_string())),
                },
                _ => warp::reply::json(&ApiReply::Error(String::from("Wrong JSON data"))),
            }
        });

    let shield_action = warp::path("shield")
        .and(warp::body::json())
        .and(warp::header("username"))
        .and(warp::header("token"))
        .map(move |req: Request, username: String, pass: String| {
            let mut game = shield_game.lock().unwrap();
            if let Err(err) = game.authenticate(username.as_str(), pass.as_str()) {
                return warp::reply::json(&ApiReply::Error(err.to_string()));
            }

            if let Ok(resp) = get_game_status(&game) {
                return resp;
            }
            match req {
                Request::SingleCoord { coord: (x, y) } => {
                    match game.generate_shield(username.as_str(), x, y) {
                        Ok(()) => warp::reply::json(&ApiReply::Data(Reply::Ok)),
                        Err(err) => warp::reply::json(&ApiReply::Error(err.to_string())),
                    }
                }
                _ => warp::reply::json(&ApiReply::Error(String::from("Wrong JSON data"))),
            }
        });

    let increase_action = warp::path("increase-range")
        .and(warp::body::json())
        .and(warp::header("username"))
        .and(warp::header("token"))
        .map(move |req: Request, username: String, pass: String| {
            let mut game = increase_game.lock().unwrap();
            if let Err(err) = game.authenticate(username.as_str(), pass.as_str()) {
                return warp::reply::json(&ApiReply::Error(err.to_string()));
            }

            if let Ok(resp) = get_game_status(&game) {
                return resp;
            }
            match req {
                Request::SingleCoord { coord: (x, y) } => {
                    match game.increase_range(username.as_str(), x, y) {
                        Ok(()) => warp::reply::json(&ApiReply::Data(Reply::Ok)),
                        Err(err) => warp::reply::json(&ApiReply::Error(err.to_string())),
                    }
                }
                _ => warp::reply::json(&ApiReply::Error(String::from("Wrong JSON data"))),
            }
        });

    let donate_action = warp::path("donate-points")
        .and(warp::body::json())
        .and(warp::header("username"))
        .and(warp::header("token"))
        .map(move |req: Request, username: String, pass: String| {
            let mut game = donate_game.lock().unwrap();
            if let Err(err) = game.authenticate(username.as_str(), pass.as_str()) {
                return warp::reply::json(&ApiReply::Error(err.to_string()));
            }

            if let Ok(resp) = get_game_status(&game) {
                return resp;
            }
            match req {
                Request::Donate { receiver, amount } => {
                    match game.donate_points(username.as_str(), receiver.as_str(), amount) {
                        Ok(()) => warp::reply::json(&ApiReply::Data(Reply::Ok)),
                        Err(err) => warp::reply::json(&ApiReply::Error(err.to_string())),
                    }
                }
                _ => warp::reply::json(&ApiReply::Error(String::from("Wrong JSON data"))),
            }
        });

    let build_action = warp::path("build-zord")
        .and(warp::body::json())
        .and(warp::header("username"))
        .and(warp::header("token"))
        .map(move |req: Request, username: String, pass: String| {
            let mut game = build_game.lock().unwrap();
            if let Err(err) = game.authenticate(username.as_str(), pass.as_str()) {
                return warp::reply::json(&ApiReply::Error(err.to_string()));
            }

            if let Ok(resp) = get_game_status(&game) {
                return resp;
            }
            match req {
                Request::SingleCoord { coord: (x, y) } => {
                    match game.build_zord(username.as_str(), x, y) {
                        Ok(()) => warp::reply::json(&ApiReply::Data(Reply::Ok)),
                        Err(err) => warp::reply::json(&ApiReply::Error(err.to_string())),
                    }
                }
                _ => warp::reply::json(&ApiReply::Error(String::from("Wrong JSON data"))),
            }
        });

    let map_action = warp::path("map").map(move || {
        let game = map_game.lock().unwrap();
        warp::reply::json(&ApiReply::Data(Reply::Map(&game.board.board)))
    });

    let leaderboard_action = warp::path("leaderboard").map(move || {
        let game = leaderboard_game.lock().unwrap();
        warp::reply::json(&ApiReply::Data(Reply::Leaderboard(&game.players)))
    });

    let day_action = warp::path("day").map(move || {
        let game = day_game.lock().unwrap();

        warp::reply::json(&ApiReply::Data(Reply::GameInfo {
            day: game.day,
            start_of_day: game
                .start_of_day
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }))
    });

    let routes = warp::post()
        .and(
            shoot_action
                .or(move_action)
                .or(shield_action)
                .or(increase_action)
                .or(donate_action)
                .or(build_action)
                .or(map_action)
                .or(leaderboard_action)
                .or(day_action),
        )
        .recover(handle_rejection);
    warp::serve(routes).run(([127, 0, 0, 1], 6969)).await;
}
