use axum::{Router, routing::get};

#[cfg(feature = "dev")]
use tower_http::cors::CorsLayer;

use crate::game::Game;

pub(crate) struct App {
    router: Router,
    game: Game,
}

impl App {
    pub(crate) fn init() -> Self {
        let game = Game::new();

        let router = {
            let router = Router::new();

            #[cfg(feature = "dev")]
            {
                let cors = CorsLayer::permissive();
                router = router.layer(cors);
            };

            router.route("/", get(|| async { "Hello, Sht!" }))
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
