use crate::BackendState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct NumChapters {
    num_chapters: i32,
}

pub async fn get_num_chapters_for_book(
    Path(p_params): Path<(String, String)>,
    State(state): State<BackendState>,
) -> Result<Json<NumChapters>, StatusCode> {
    let num_chapters_result = state
        .fetch_chapter_count_for_book(p_params.0.as_str(), p_params.1.as_str())
        .await;
    match num_chapters_result {
        Ok(num_chapters) => Ok(Json::from(NumChapters { num_chapters })),
        Err(x) => {
            println!("{}", x);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
