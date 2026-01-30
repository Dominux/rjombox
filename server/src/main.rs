mod app;
mod errors;
mod game;
mod routers;
mod stores;

use crate::app::App;

#[tokio::main]
async fn main() {
    let app = App::new();
    app.run().await;
}
