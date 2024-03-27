use crate::api::message::{Donate, DoubleCoord, SingleCoord};

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
