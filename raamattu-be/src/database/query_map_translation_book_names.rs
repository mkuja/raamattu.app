use crate::BackendState;
use serde::Serialize;
use sqlx::prelude::*;
use std::error::Error;

/// Used to map one book to matching books of other translations.
#[derive(FromRow, Serialize)]
pub struct TranslationRow {
    pub book_id: i32,
    pub book_color: String,
    pub short_name: String,
    pub full_name: String,
    pub language: String,
    pub translation: String,
    pub translation_description: String,

    #[sqlx(skip)]
    matching: Option<Vec<TranslationRow>>,
}

impl TranslationRow {
    pub fn get_matching(&self) -> Option<&Vec<TranslationRow>> {
        self.matching.as_ref()
    }
}

impl BackendState {
    pub async fn fetch_map_to_other_translation_books(
        &self,
        from_translation: &str,
        from_book: &str,
    ) -> Result<TranslationRow, Box<dyn Error>> {
        let mut mapping: TranslationRow = sqlx::query_as(
            r#"SELECT book_id, book_color, short_name,
                full_name, language, translation,
                translation_description
            FROM books_view bv
            WHERE "translation"=$1 AND short_name=$2"#,
        )
        .bind(from_translation)
        .bind(from_book)
        .fetch_one(&self.database_connection)
        .await?;

        let map_to: Vec<TranslationRow> = sqlx::query_as(
            r#"SELECT book_id, book_color, short_name,
                full_name, language, translation,
                translation_description
            FROM books_view bv
            WHERE book_id=$1"#,
        )
        .bind(mapping.book_id)
        .fetch_all(&self.database_connection)
        .await?;

        mapping.matching = Some(map_to);

        Ok(mapping)
    }
}
