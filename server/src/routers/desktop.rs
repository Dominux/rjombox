use axum::{Router, routing::get};

use crate::game::Game;

pub fn build_router(game: &mut Game) -> Router {
    Router::new().route("/start_game", get(start_game))
}

async fn start_game() -> &'static str {
    "OK"
}
