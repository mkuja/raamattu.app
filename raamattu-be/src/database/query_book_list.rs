use serde::Serialize;
use sqlx::prelude::FromRow;
use std::error::Error;

use crate::{error::LanguageError, BackendState};
use super::Language;

#[derive(Serialize, FromRow)]
pub struct Book {
    book_id: i32,
    book_color: String,
    short_name: String,
    full_name: String,
    language: String,
    translation: String,
    translation_description: String
}

pub trait AbleToListBooks {
    async fn list_books(&self, translation: &String) -> Result<Vec<Book>, Box<dyn Error>>;
}

impl AbleToListBooks for BackendState {
    async fn list_books(&self, translation: &String) -> Result<Vec<Book>, Box<dyn Error>> {
        //let trans: Result<Language, LanguageError> = translation.try_into();
        //if trans.is_err() {
        //    return Err(Box::new(trans.unwrap_err()));
        //}
        let records: Result<Vec<Book>, sqlx::Error> = sqlx::query_as(
            "SELECT book_id, book_color, short_name, full_name, language::TEXT, translation, translation_description
                    FROM books_view WHERE translation=$1"
        )
        .bind(translation)
        .fetch_all(&self.database_connection)
        .await;
        match records {
            Ok(x) => Ok(x),
            Err(x) => Err(Box::new(x))
        }
    }
}
