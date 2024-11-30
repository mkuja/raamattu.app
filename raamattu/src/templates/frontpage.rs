use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use serde::Deserialize;
use crate::application_state::ApplicationState;
use crate::error::Result;

#[derive(Template)]
#[template(path = "front_page.jinja")]
pub struct FrontPageTemplate {
    pub books: Vec<raamattu_db::Book>,
    pub translation: String,
}

#[derive(Deserialize)]
pub struct FrontPagePathParam {
    pub value: String,
}

pub struct InvalidTranslation;

pub async fn frontpage_handler(
    State(app_state): State<ApplicationState>,
    Path(params): Path<Vec<String>>,
) -> Result<Html<String>> {

    let books = app_state
        .pg_client
        .fetch_books(&params.get(1)
            .unwrap_or(&"kr38".to_string()))
            .await
            .unwrap();
    
    let rendered = Ok(Html(FrontPageTemplate {
        books,
        translation: params.get(0).unwrap_or(&"kr38".to_string()).into()
    }.render().unwrap()));
    
    match rendered {
        Ok(page) => Ok(page),
        Err(_why) => Err(StatusCode::NOT_FOUND),
    }
}

