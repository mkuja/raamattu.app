mod database;
mod error;
mod handlers;
mod state;

use axum::routing::get;
use axum::Router;
use state::*;
use handlers::get_book_list;

#[tokio::main]
async fn main() {
    let conn_str = std::env::var("RAAMATTU_PG");
    if conn_str.is_err() {
        return
    }

    let app = Router::new()
        .route("/", get(|| async { "Hello" }))
        .route("/:translation/book-list", get(get_book_list))
        .with_state(BackendState::new(conn_str.unwrap().as_str()).await);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
