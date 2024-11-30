use askama::Template;
use axum::http::StatusCode;
use tantivy::tokenizer::Language;
use crate::handlers::filters;

#[derive(Template)]
#[template(path = "error.jinja")]
pub struct ErrorResponse {
    message: String,
    language: String,
    code: u16,
}

impl ErrorResponse {
    pub fn new(
        message: impl Into<String>,
        language: impl Into<String>,
        code: u16,
    ) -> Self {
        Self {
            message: message.into(),
            language: language.into(),
            code,
        }
    }
}
