use askama::{Template};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use super::filters;

#[derive(Template)]
#[template(path="about.jinja")]
struct About{
    language: String,
}
pub async fn about_page(Path(lang): Path<String>) -> (StatusCode, Html<String>) {
    (StatusCode::OK, Html(
        About{language: lang}.render().unwrap()))
}
