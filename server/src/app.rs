use axum::{Router, routing::get};

#[cfg(feature = "dev")]
use tower_http::cors::CorsLayer;

pub(crate) struct App {
    router: Router,
}

impl App {
    pub(crate) fn init() -> Self {
        #[allow(unused_mut)]
        let mut router = Router::new().route("/", get(|| async { "Hello, Sht!" }));

        #[cfg(feature = "dev")]
        {
            let cors = CorsLayer::permissive();
            router = router.layer(cors);
        }

        Self { router }
    }

    pub(crate) async fn run(self) {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:26197")
            .await
            .unwrap();
        axum::serve(listener, self.router).await.unwrap();
    }
}
