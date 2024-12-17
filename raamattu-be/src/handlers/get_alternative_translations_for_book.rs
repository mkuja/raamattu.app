use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::database::TranslationRow;
use crate::BackendState;

pub async fn get_alternative_translations_for_book(
    Path((translation, book)): Path<(String, String)>,
    State(state): State<BackendState>,
) -> Result<Json<TranslationRow>, StatusCode> {
    let book_mappings = state
        .fetch_map_to_other_translation_books(&translation, &book)
        .await;

    match book_mappings {
        Ok(x) => Ok(Json(x)),
        Err(x) => {
            println!("error: {}", x);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
