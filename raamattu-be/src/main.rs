mod database;
mod error;
mod handlers;
mod state;

use axum::Router;
use axum::{http::Method, routing::get};

use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use handlers::{
    get_alternative_translations_for_book, get_book_list, get_chapter_verses,
    get_num_chapters_for_book, get_translations,
};
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
        .route(
            "/chapter/:translation/:book/:chapter",
            get(get_chapter_verses),
        )
        .route(
            "/other-translations/:from_translation/:from_book",
            get(get_alternative_translations_for_book),
        )
        .route("/translations", get(get_translations))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(be_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.expect("LALA");
}
