mod serve_static;
mod templates;
// mod search;  // TODO: Laita tantivy päälle kunhan etusivu toimii
mod application_state;
mod sitemap;
mod handlers;
mod database;
mod utility;

use axum::{routing::get, Router};
use sqlx::{FromRow};
use askama::Template;
use axum::response::Redirect;
use rust_i18n::i18n;
use application_state::ApplicationState;
use crate::handlers::about::about_page;

i18n!("locales", fallback = "en");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn_str = std::env::var("RAAMATTU_PG")?;
    let client = database::db::Client::new(&conn_str)
        .await?;

    // Prepare searches with Tantivy.
    // let mut conn = client.connection_pool.acquire().await?;
    // let (index, reader) = search::build_index(&mut conn)
    //     .await
    //     .expect("could not build search index");
    //
    // generate_sitemap(&mut conn).await;
    // drop(conn);

    let app_state = ApplicationState {
        pg_client: client,
        // index,
        // reader
    };
    
    let app = Router::new()
        .route("/", get(|| async { Redirect::temporary("/en/kr38") }))
        .route("/:lang/:trans", get(handlers::front_page::front_page_handler))
        // .route("/search", get(search::search_route))
        .route("/:lang/about", get(about_page))
        // .route("/search-help", get(search_help))
        // .route(
        //     "/books/:short_name",
        //     get(templates::chapter_number_page::chapter_numbers_page),
        // )
        // .route(
        //     "/books/:short_name/:chapter_num",
        //     get(templates::chapter_page::chapter_page),
        // )


        // .route("/api/v1/enumerate_books", get(api::Book::enumerate_books))
        // .route("/api/v1/search/:search_string", get(api::search))
        // .route("/api/v1/:book/num_chapters", get(api::Book::num_chapters))
        // .route("/api/v1/:book/:chapter", get(api::Book::chapter))
        .with_state(app_state)
        .nest_service("/static", serve_static::serve_static::serve_static());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}



// async fn search_help(State(state): State<ApplicationState>) -> Result<Html<String>, (axum::http::StatusCode, String)> {
//     let mut conn = state.pool.acquire().await.unwrap();
//     let sh = SearchHelp::new(&mut conn).await;
//     Ok(Html(sh.render().unwrap()))
// }
//
// impl SearchHelp {
//     pub async fn new(conn: &mut sqlx::SqliteConnection) -> SearchHelp {
//         let rows: Vec<(String, String)> = sqlx::query_as("SELECT short_name as short, long_name as long FROM books")
//             .fetch_all(conn)
//             .await
//             .unwrap();
//         SearchHelp {
//             bible_books: rows,
//         }
//     }
// }
