use crate::Book;
use crate::db::Client;
use crate::Result;

impl Client {
    pub async fn fetch_books(&self, translation: &str) -> Result<Vec<Book>> {
        let result = sqlx::query_as(
            r#"SELECT book_id, book_color, short_name, full_name, language, translation, translation_description
                    FROM books_view WHERE translation=$1"#)
            .bind(translation)
            .fetch_all(&self.connection_pool)
            .await;
        if let Ok(result) = result {
            Ok(result)
        } else {
            println!("{:?}", result);
            Err(Box::new(crate::error::Error::DatabaseError(result.unwrap_err())))
        }
    }
}