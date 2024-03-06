mod game;

use serde::{Deserialize, Serialize};
use warp::{reply::Json, Filter};

#[derive(Serialize, Deserialize)]
enum GameReq {
    List(Entity),
}

#[derive(Serialize, Deserialize)]
struct Entity {
    x: i32,
    y: i32,
    type_: EntityType,
}

#[derive(Serialize, Deserialize)]
enum EntityType {
    Human,
    Animal,
}

#[tokio::main]
async fn main() {
    let routes = warp::any().map(handle_request);
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

fn handle_request() -> Json {
    let entity = Entity {
        x: 0,
        y: 0,
        type_: EntityType::Human,
    };
    warp::reply::json(&GameReq::List(entity))
}
