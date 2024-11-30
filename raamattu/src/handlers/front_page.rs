use std::vec::Vec;
use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use crate::application_state::ApplicationState;
use crate::database::types::Book;
use crate::utility::error_response::ErrorResponse;
use crate::handlers::filters;

#[derive(Template)]
#[template(path = "front_page.jinja")]
pub struct FrontPageTemplate {
    pub title: String,
    pub books: Vec<Book>,
    pub translation: String,
    pub language: String,
}

pub async fn front_page_handler(
    State(app_state): State<ApplicationState>,
    Path((lang, trans)): Path<(String, String)>,
) -> (StatusCode, Html<String>) {
    // Fetch books for the translation.
    let books = app_state
        .pg_client
        .fetch_books(&trans)
        .await;
    if books.is_err() {
        return (
            StatusCode::NOT_FOUND,
            Html(ErrorResponse::new(
                "Translation not found.",
                lang.as_str(), 404)
                .render().unwrap())
        )
    }

    match lang.as_str() {
        "fi" => {
            (StatusCode::OK,
            Html(FrontPageTemplate{
                title: "Kirjat".to_string(),
                books: books.unwrap(),
                translation: trans,
                language: "fi".to_string()
            }.render().unwrap()))
        }
        "en" | _ => {
            (StatusCode::OK,
            Html(FrontPageTemplate{
                title: "Books".to_string(),
                books: books.unwrap(),
                translation: trans,
                language: "en".to_string()
            }.render().unwrap()))}
    }
}

