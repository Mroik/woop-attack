use super::message::{ApiReply, Reply, Request};
use crate::game::game::Game;
use std::convert::Infallible;
use std::sync::Arc;
use std::sync::Mutex;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::Filter;
use warp::Reply as WarpReply;

async fn handle_rejection(_: Rejection) -> Result<impl WarpReply, Infallible> {
    let msg = "Incorrect interaction with the api. Check method, endpoint and JSON data";
    let json = warp::reply::json(&ApiReply::Error(String::from(msg)));
    Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST))
}

pub async fn start_api(game: Mutex<Game>) {
    let shoot_game = Arc::new(game);
    let move_game = shoot_game.clone();
    let shield_game = shoot_game.clone();
    let increase_game = shoot_game.clone();
    let donate_game = shoot_game.clone();
    let build_game = shoot_game.clone();

    let shoot_action = warp::path("shoot")
        .and(warp::body::json())
        .map(move |req: Request| {
            let mut game = shoot_game.lock().unwrap();
            match req {
                Request::DoubleCoord {
                    player,
                    from: (x_f, y_f),
                    to: (x_t, y_t),
                } => match game.player_shoot(player.as_str(), x_f, y_f, x_t, y_t) {
                    Ok(_) => warp::reply::json(&ApiReply::Data(Reply::Ok)),
                    Err(err) => warp::reply::json(&ApiReply::Error(err.to_string())),
                },
                _ => warp::reply::json(&ApiReply::Error(String::from("Wrong JSON data"))),
            }
        });

    let move_action = warp::path("move")
        .and(warp::body::json())
        .map(move |req: Request| {
            let mut game = move_game.lock().unwrap();
            match req {
                Request::DoubleCoord {
                    player,
                    from: (x_f, y_f),
                    to: (x_t, y_t),
                } => match game.move_zord(player.as_str(), x_f, y_f, x_t, y_t) {
                    Ok(_) => warp::reply::json(&ApiReply::Data(Reply::Ok)),
                    Err(err) => warp::reply::json(&ApiReply::Error(err.to_string())),
                },
                _ => warp::reply::json(&ApiReply::Error(String::from("Wrong JSON data"))),
            }
        });

    let shield_action = warp::path("shield")
        .and(warp::body::json())
        .map(move |req: Request| {
            let mut game = shield_game.lock().unwrap();
            match req {
                Request::SingleCoord {
                    player,
                    coord: (x, y),
                } => match game.generate_shield(player.as_str(), x, y) {
                    Ok(()) => warp::reply::json(&ApiReply::Data(Reply::Ok)),
                    Err(err) => warp::reply::json(&ApiReply::Error(err.to_string())),
                },
                _ => warp::reply::json(&ApiReply::Error(String::from("Wrong JSON data"))),
            }
        });

    let increase_action =
        warp::path("increase-range")
            .and(warp::body::json())
            .map(move |req: Request| {
                let mut game = increase_game.lock().unwrap();
                match req {
                    Request::SingleCoord {
                        player,
                        coord: (x, y),
                    } => match game.increase_range(player.as_str(), x, y) {
                        Ok(()) => warp::reply::json(&ApiReply::Data(Reply::Ok)),
                        Err(err) => warp::reply::json(&ApiReply::Error(err.to_string())),
                    },
                    _ => warp::reply::json(&ApiReply::Error(String::from("Wrong JSON data"))),
                }
            });

    let donate_action =
        warp::path("donate-points")
            .and(warp::body::json())
            .map(move |req: Request| {
                let mut game = donate_game.lock().unwrap();
                match req {
                    Request::Donate {
                        donator,
                        receiver,
                        amount,
                    } => match game.donate_points(donator.as_str(), receiver.as_str(), amount) {
                        Ok(()) => warp::reply::json(&ApiReply::Data(Reply::Ok)),
                        Err(err) => warp::reply::json(&ApiReply::Error(err.to_string())),
                    },
                    _ => warp::reply::json(&ApiReply::Error(String::from("Wrong JSON data"))),
                }
            });

    let build_action = warp::path("build-zord")
        .and(warp::body::json())
        .map(move |req: Request| {
            let mut game = build_game.lock().unwrap();
            match req {
                Request::SingleCoord {
                    player,
                    coord: (x, y),
                } => match game.build_zord(player.as_str(), x, y) {
                    Ok(()) => warp::reply::json(&ApiReply::Data(Reply::Ok)),
                    Err(err) => warp::reply::json(&ApiReply::Error(err.to_string())),
                },
                _ => warp::reply::json(&ApiReply::Error(String::from("Wrong JSON data"))),
            }
        });

    let routes = warp::post()
        .and(
            shoot_action
                .or(move_action)
                .or(shield_action)
                .or(increase_action)
                .or(donate_action)
                .or(build_action),
        )
        .recover(handle_rejection);
    warp::serve(routes).run(([127, 0, 0, 1], 6969)).await;
}
