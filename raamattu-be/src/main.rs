mod database;
mod error;
mod handlers;
mod state;

use axum::extract::State;
use axum::{http::Method, routing::get};
use axum::{Extension, Router};

use tower::{Service, ServiceBuilder, ServiceExt};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use handlers::{get_book_list, get_num_chapters_for_book};
use state::*;

#[tokio::main]
async fn main() {
    let conn_str = std::env::var("RAAMATTU_PG");
    if conn_str.is_err() {
        return;
    }

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET])
        // allow requests from any origin
        .allow_origin(Any);

    let be_state = BackendState::new(conn_str.expect("RAAMATTU_PG is undefined").as_str()).await;

    let app = Router::new()
        .route("/", get(|| async { "Hello" }))
        .route("/book-list/by-translation/:translation", get(get_book_list))
        .route(
            "/chapter-list/:translation/:book",
            get(get_num_chapters_for_book),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(be_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.expect("LALA");
}
