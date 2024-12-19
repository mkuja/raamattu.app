use axum::{extract::State, http::StatusCode, Json};

use crate::database::*;
use crate::BackendState;

pub async fn get_translations(
    State(state): State<BackendState>,
) -> Result<Json<Vec<Translation>>, StatusCode> {
    if let Ok(resp) = state.fetch_translations().await {
        Ok(Json(resp))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
