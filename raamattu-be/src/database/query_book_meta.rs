use crate::BackendState;
use sqlx::prelude::*;
use std::error::Error;

impl BackendState {
    pub async fn fetch_chapter_count_for_book(
        &self,
        translation_name: &str,
        short_book_name: &str,
    ) -> Result<i32, Box<dyn Error>> {
        let count = sqlx::query(
            "SELECT MAX(chapter_number) FROM verse_texts where translation_name=$1 AND short_book_name=$2 LIMIT 1"
        )
            .bind(translation_name)
            .bind(short_book_name)
            .fetch_one(&self.database_connection)
            .await?;
        Ok(count.try_get(0)?)
    }
}
