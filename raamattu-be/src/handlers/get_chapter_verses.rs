use crate::{database::Chapter, BackendState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn get_chapter_verses(
    Path(p_params): Path<(String, String, i32)>,
    State(state): State<BackendState>,
) -> Result<Json<Chapter>, StatusCode> {
    match state
        .fetch_chapter_verses(p_params.0.as_str(), p_params.1.as_str(), p_params.2)
        .await
    {
        Ok(x) => Ok(Json::from(x)),
        Err(err) => Err(err),
    }
}
