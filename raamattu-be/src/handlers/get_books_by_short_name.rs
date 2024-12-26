use crate::{database::*, BackendState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn get_books_by_short_name(
    Path(short_name): Path<String>,
    State(state): State<BackendState>,
) -> Result<Json<Vec<Book>>, StatusCode> {
    let stuff = state.query_books_by_short_name(short_name.as_str()).await;
    if let Ok(stuff) = stuff {
        Ok(Json(stuff))
    } else {
        println!("{}", stuff.unwrap_err());
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
}
