mod app;

use crate::app::App;

#[tokio::main]
async fn main() {
    let app = App::init();
    app.run().await;
}
