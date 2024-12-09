use crate::{database::Book, BackendState};
use axum::{extract::{Path, State}, http::StatusCode, Json};

use crate::database::*;

pub async fn get_book_list(
    Path(translation): Path<String>,
    State(state): State<BackendState>,
) -> Result<Json<Vec<Book>>, StatusCode> {
    let books = state.list_books(&translation).await;

    match books {
        Ok(x) => Ok(Json(x)),
        Err(x) => {
            println!("error: {}", x);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }

}
