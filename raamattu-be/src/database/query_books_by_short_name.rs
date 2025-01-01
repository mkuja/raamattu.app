use crate::BackendState;
use std::error::Error;

use super::Book;

impl BackendState {
    pub async fn query_books_by_short_name(
        &self,
        short_book_name: &str,
    ) -> Result<Vec<Book>, Box<dyn Error>> {
        let books: Vec<Book> = sqlx::query_as(
            "select book_id, book_color, short_name, full_name, language::TEXT, translation, translation_description
            from books_view where book_id IN (select book_id from books_view bv where bv.short_name=$1)"
        )
            .bind(short_book_name)
            .fetch_all(&self.database_connection)
            .await?;
        Ok(books)
    }
}
