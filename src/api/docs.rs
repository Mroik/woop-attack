use crate::api::message::{Donate, DoubleCoord, SingleCoord};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::docs::shoot,
        crate::api::docs::move_,
        crate::api::docs::shield,
        crate::api::docs::increase_range,
        crate::api::docs::donate_points,
        crate::api::docs::build_zord,
        crate::api::docs::get_board_data,
        crate::api::docs::get_day,
        crate::api::docs::get_activity,
        crate::api::docs::leaderboard,
    ),
    components(schemas(
        crate::game::entity::Entity,
        crate::game::zord::Zord,
        crate::game::totem::Totem,
        crate::game::player::Player,
        crate::api::message::DoubleCoord,
        crate::api::message::SingleCoord,
        crate::api::message::Donate,
        crate::api::message::WoopMap,
        crate::api::message::Leaderboard,
        crate::api::message::GameInfo,
        crate::api::message::Activity,
        crate::api::message::Empty,
        crate::game::log::PlayerEvent,
    ))
)]
pub struct ApiDoc;

/// Shoot a zord with your own
#[allow(dead_code)]
#[utoipa::path(
    post,
    path = "/shoot",
    request_body(
        content = DoubleCoord,
        example = json!(DoubleCoord {from: (0, 0), to: (1, 1)}),
    ),
    params(
        ("username" = String, Header, example = json!(String::from("mirko.faina"))),
        ("token" = String, Header, example = json!(String::from("this_is_a_token"))),
    ),
    responses(
        (status = 200, body = Empty),
    ),
)]
pub async fn shoot() {}

/// Move zord
#[allow(dead_code)]
#[utoipa::path(
    post,
    path = "/move",
    request_body(
        content = DoubleCoord,
        example = json!(DoubleCoord {from: (0, 0), to: (1, 1)}),
    ),
    params(
        ("username" = String, Header, example = json!(String::from("mirko.faina"))),
        ("token" = String, Header, example = json!(String::from("this_is_a_token"))),
    ),
    responses(
        (status = 200, body = Empty)
    ),
)]
pub async fn move_() {}

/// Generate shield on zord
#[allow(dead_code)]
#[utoipa::path(
    post,
    path = "/shield",
    request_body(
        content = SingleCoord,
        example = json!(SingleCoord {coord: (0, 0)}),
    ),
    params(
        ("username" = String, Header, example = json!(String::from("mirko.faina"))),
        ("token" = String, Header, example = json!(String::from("this_is_a_token"))),
    ),
    responses(
        (status = 200, body = Empty),
    ),
)]
pub async fn shield() {}

/// Increase zord range
#[allow(dead_code)]
#[utoipa::path(
    post,
    path = "/increase",
    request_body(
        content = SingleCoord,
        example = json!(SingleCoord {coord: (0, 0)}),
    ),
    params(
        ("username" = String, Header, example = json!(String::from("mirko.faina"))),
        ("token" = String, Header, example = json!(String::from("this_is_a_token"))),
    ),
    responses(
        (status = 200, body = Empty),
    ),
)]
pub async fn increase_range() {}

/// Donate points to another player
#[allow(dead_code)]
#[utoipa::path(
    post,
    path = "/donate",
    request_body(
        content = Donate,
        example = json!(Donate { receiver: String::from("fin"), amount: 10 }),
    ),
    params(
        ("username" = String, Header, example = json!(String::from("mirko.faina"))),
        ("token" = String, Header, example = json!(String::from("this_is_a_token"))),
    ),
    responses(
        (status = 200, body = Empty),
    ),
)]
pub async fn donate_points() {}

/// Build another zord
#[allow(dead_code)]
#[utoipa::path(
    post,
    path = "/build",
    request_body(
        content = SingleCoord,
        example = json!(SingleCoord {coord: (0, 0)}),
    ),
    params(
        ("username" = String, Header, example = json!(String::from("mirko.faina"))),
        ("token" = String, Header, example = json!(String::from("this_is_a_token"))),
    ),
    responses(
        (status = 200, body = Empty),
    ),
)]
pub async fn build_zord() {}

/// Get entities present on the game board
#[allow(dead_code)]
#[utoipa::path(post, path = "/map", responses((status = 200, body = WoopMap),))]
pub async fn get_board_data() {}

/// Get list of players sorted by points
#[allow(dead_code)]
#[utoipa::path(post, path = "/leaderboard", responses((status = 200, body = Leaderboard),))]
pub async fn leaderboard() {}

/// Get info on the current day
#[allow(dead_code)]
#[utoipa::path(post, path = "/day", responses((status = 200, body = GameInfo),))]
pub async fn get_day() {}

/// Get a list of the last 100 actions
#[allow(dead_code)]
#[utoipa::path(post, path = "/activity", responses((status = 200, body = Activity),))]
pub async fn get_activity() {}
