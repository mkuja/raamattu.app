use askama::Template;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Html;
use crate::application_state::ApplicationState;
use crate::query_params::FrontPageQueryParams;

#[derive(Template)]
#[template(path = "front_page.jinja")]
pub struct FrontPageTemplate {
    pub books: Vec<raamattu_db::Book>,
    pub translation: String,
}

#[axum::debug_handler]
pub async fn frontpage_handler(
    State(app_state): State<ApplicationState>,
    Query(qp): Query<FrontPageQueryParams>,
) -> Result<Html<String>, StatusCode> {
    let translation;
    let books = if let Some(t) = qp.tr.clone() {
        translation = t;
        app_state
            .pg_client
            .fetch_books(&translation)
    } else {
        translation = "kr38".to_string();  // Default, if none was provided.
        app_state
            .pg_client
            .fetch_books("kr38")
    }.await;
    if let Ok(books) = books {
        println!("OK");
        Ok(Html(FrontPageTemplate {
            books,
            translation,
        }.render().unwrap()))
    } else {
        println!("Teapot");
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

