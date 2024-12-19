use std::error::Error;

use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::BackendState;

#[derive(FromRow, Serialize)]
pub struct Translation {
    pub id: i32,
    pub language: String,
    pub description: String,
    pub name: String,
}

impl BackendState {
    pub async fn fetch_translations(&self) -> Result<Vec<Translation>, Box<dyn Error>> {
        let records =
            sqlx::query_as("select id, language::TEXT, description, name from translations t")
                .fetch_all(&self.database_connection)
                .await?;
        Ok(records)
    }
}
