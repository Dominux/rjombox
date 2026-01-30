use axum::{Router, routing::get};

#[cfg(feature = "dev")]
use tower_http::cors::CorsLayer;

use crate::game::Game;

#[derive(Clone)]
pub struct AppState;

pub(crate) struct App {
    router: Router,
    game: Game,
}

impl App {
    pub(crate) fn new() -> Self {
        let game = Game::new();

        let state = AppState {};

        let router = {
            let router = Router::new();

            #[cfg(feature = "dev")]
            {
                let cors = CorsLayer::permissive();
                router = router.layer(cors);
            };

            router
                .route("/", get(|| async { "Hello, Sht!" }))
                .with_state(state)
        };

        Self { router, game }
    }

    pub(crate) async fn run(self) {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:26197")
            .await
            .unwrap();
        axum::serve(listener, self.router).await.unwrap();
    }
}
