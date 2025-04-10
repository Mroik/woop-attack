use super::docs::ApiDoc;
use super::message::Empty;
use crate::api::message::{
    Activity, Donate, DoubleCoord, GameInfo, Leaderboard, SingleCoord, WoopMap,
};
use crate::game::game::Game;
use crate::game::player::Player;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::UNIX_EPOCH;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Json;
use warp::Filter;
use warp::Reply as WarpReply;

async fn handle_rejection(_: Rejection) -> Result<impl WarpReply, Infallible> {
    let msg = "Incorrect interaction with the api. Check method, endpoint and JSON data";
    let json = warp::reply::json(&Empty::Error(msg.to_string()));
    Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST))
}

fn get_game_status(game: &Game) -> Result<Json, ()> {
    match game.day {
        0 => Ok(warp::reply::json(&Empty::Error(
            "Game hasn't started yet".to_string(),
        ))),
        29 => Ok(warp::reply::json(&Empty::Error(
            "Game has ended".to_string(),
        ))),
        _ => Err(()),
    }
}

pub async fn start_api(game: Arc<Mutex<Game>>) {
    let shoot_action = warp::path("shoot")
        .and(warp::body::json())
        .and(warp::header("username"))
        .and(warp::header("token"))
        .map({
            let game = game.clone();
            move |req: DoubleCoord, username: String, pass: String| {
                let mut game = game.lock().unwrap();
                if let Err(err) = game.authenticate(username.as_str(), pass.as_str()) {
                    return warp::reply::json(&Empty::Error(err.to_string()));
                }

                if let Ok(resp) = get_game_status(&game) {
                    return resp;
                }
                let (x_f, y_f) = req.from;
                let (x_t, y_t) = req.to;
                match game.player_shoot(username.as_str(), x_f, y_f, x_t, y_t) {
                    Ok(_) => warp::reply::json(&Empty::Ok),
                    Err(err) => warp::reply::json(&Empty::Error(err.to_string())),
                }
            }
        });

    let move_action = warp::path("move")
        .and(warp::body::json())
        .and(warp::header("username"))
        .and(warp::header("token"))
        .map({
            let game = game.clone();
            move |req: DoubleCoord, username: String, pass: String| {
                let mut game = game.lock().unwrap();
                if let Err(err) = game.authenticate(username.as_str(), pass.as_str()) {
                    return warp::reply::json(&Empty::Error(err.to_string()));
                }

                if let Ok(resp) = get_game_status(&game) {
                    return resp;
                }
                let (x_f, y_f) = req.from;
                let (x_t, y_t) = req.to;
                match game.move_zord(username.as_str(), x_f, y_f, x_t, y_t) {
                    Ok(_) => warp::reply::json(&Empty::Ok),
                    Err(err) => warp::reply::json(&Empty::Error(err.to_string())),
                }
            }
        });

    let shield_action = warp::path("shield")
        .and(warp::body::json())
        .and(warp::header("username"))
        .and(warp::header("token"))
        .map({
            let game = game.clone();
            move |req: SingleCoord, username: String, pass: String| {
                let mut game = game.lock().unwrap();
                if let Err(err) = game.authenticate(username.as_str(), pass.as_str()) {
                    return warp::reply::json(&Empty::Error(err.to_string()));
                }

                if let Ok(resp) = get_game_status(&game) {
                    return resp;
                }
                let (x, y) = req.coord;
                match game.generate_shield(username.as_str(), x, y) {
                    Ok(()) => warp::reply::json(&Empty::Ok),
                    Err(err) => warp::reply::json(&Empty::Error(err.to_string())),
                }
            }
        });

    let increase_action = warp::path("increase-range")
        .and(warp::body::json())
        .and(warp::header("username"))
        .and(warp::header("token"))
        .map({
            let game = game.clone();
            move |req: SingleCoord, username: String, pass: String| {
                let mut game = game.lock().unwrap();
                if let Err(err) = game.authenticate(username.as_str(), pass.as_str()) {
                    return warp::reply::json(&Empty::Error(err.to_string()));
                }

                if let Ok(resp) = get_game_status(&game) {
                    return resp;
                }
                let (x, y) = req.coord;
                match game.increase_range(username.as_str(), x, y) {
                    Ok(()) => warp::reply::json(&Empty::Ok),
                    Err(err) => warp::reply::json(&Empty::Error(err.to_string())),
                }
            }
        });

    let donate_action = warp::path("donate-points")
        .and(warp::body::json())
        .and(warp::header("username"))
        .and(warp::header("token"))
        .map({
            let game = game.clone();
            move |req: Donate, username: String, pass: String| {
                let mut game = game.lock().unwrap();
                if let Err(err) = game.authenticate(username.as_str(), pass.as_str()) {
                    return warp::reply::json(&Empty::Error(err.to_string()));
                }

                if let Ok(resp) = get_game_status(&game) {
                    return resp;
                }
                match game.donate_points(username.as_str(), req.receiver.as_str(), req.amount) {
                    Ok(()) => warp::reply::json(&Empty::Ok),
                    Err(err) => warp::reply::json(&Empty::Error(err.to_string())),
                }
            }
        });

    let build_action = warp::path("build-zord")
        .and(warp::body::json())
        .and(warp::header("username"))
        .and(warp::header("token"))
        .map({
            let game = game.clone();
            move |req: SingleCoord, username: String, pass: String| {
                let mut game = game.lock().unwrap();
                if let Err(err) = game.authenticate(username.as_str(), pass.as_str()) {
                    return warp::reply::json(&Empty::Error(err.to_string()));
                }

                if let Ok(resp) = get_game_status(&game) {
                    return resp;
                }
                let (x, y) = req.coord;
                match game.build_zord(username.as_str(), x, y) {
                    Ok(()) => warp::reply::json(&Empty::Ok),
                    Err(err) => warp::reply::json(&Empty::Error(err.to_string())),
                }
            }
        });

    let map_action = warp::path("map").map({
        let game = game.clone();
        move || {
            let game = game.lock().unwrap();
            warp::reply::json(&WoopMap {
                zords: &game.zords,
                totems: (&game.totems.0, &game.totems.1),
            })
        }
    });

    let leaderboard_action = warp::path("leaderboard").map({
        let game = game.clone();
        move || {
            let game = game.lock().unwrap();
            let mut lead: Vec<&Player> = game.players.values().collect();
            lead.sort_by_key(|p| p.points);
            lead.reverse();
            warp::reply::json(&Leaderboard { leaderboard: &lead })
        }
    });

    let day_action = warp::path("day").map({
        let game = game.clone();
        move || {
            let game = game.lock().unwrap();
            warp::reply::json(&GameInfo {
                day: game.day,
                start_of_day: game
                    .start_of_day
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            })
        }
    });

    let log_action = warp::path("activity").and(warp::query::query()).map({
        let game = game.clone();
        move |q: HashMap<String, usize>| {
            let game = game.lock().unwrap();
            let chunk = q.get("chunk").copied().unwrap_or(0);
            let data = game.logged_actions.get_chunk(chunk);
            warp::reply::json(&Activity { activity: data })
        }
    });

    let auth_action = warp::path("auth")
        .and(warp::header("username"))
        .and(warp::header("token"))
        .map({
            let game = game.clone();
            move |user: String, token: String| {
                let game = game.lock().unwrap();
                if let Err(err) = game.authenticate(user.as_str(), token.as_str()) {
                    return warp::reply::json(&Empty::Error(err.to_string()));
                }
                warp::reply::json(&Empty::Ok)
            }
        });

    // Static pages
    let index_static = warp::path("index.html").map(|| {
        let body = fs::read("static/index.html").unwrap();
        warp::reply::html(body)
    });
    let index_empty_static = warp::path::end().map(|| {
        let body = fs::read("static/index.html").unwrap();
        warp::reply::html(body)
    });
    let style_static = warp::path("style.css").map(|| {
        let body = fs::read("static/style.css").unwrap();
        warp::reply::html(body)
    });
    let js_static = warp::path("client.js").map(|| {
        let body = fs::read("static/client.js").unwrap();
        warp::reply::html(body)
    });

    let docs = warp::path("docs")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));
    let rapidoc = warp::path("rapidoc")
        .and(warp::get())
        .map(|| warp::reply::html(RapiDoc::new("/docs").to_html()));
    let static_pages = warp::get().and(
        index_static
            .or(index_empty_static)
            .or(style_static)
            .or(js_static),
    );

    let cors =
        warp::cors()
            .allow_method("POST")
            .allow_headers(["username", "token", "Content-Type"]);
    let logger = warp::log("api::api");
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
                .or(day_action)
                .or(log_action)
                .or(auth_action),
        )
        .or(docs)
        .or(rapidoc)
        .or(static_pages)
        .recover(handle_rejection)
        .with(cors)
        .with(logger);
    warp::serve(routes).run(([0, 0, 0, 0], 6969)).await;
}
