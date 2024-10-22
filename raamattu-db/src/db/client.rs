use sqlx::{Executor, PgPool, Pool, Postgres, Row};
use sqlx::postgres::PgPoolOptions;
use crate::error::Result;
use crate::types::{BooksJoinRecord, TranslationRecord, VerseTextsViewRecord, Book};


#[derive(Clone)]
pub struct Client {
    pub connection_pool: PgPool,
}


impl Client {
    /// Connect to PostgreSQL at `url`, and return new `Client` or error.
    pub async fn new(url: &str) -> Result<Client> {
        let mut opts = PgPoolOptions::new()
            .max_connections(5);
        Ok(Client {
            connection_pool: opts.connect(url)
                .await?
        })
    }


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

    /// Fetch verse or a range of verses from a chapter and book of a translation.
    ///
    /// params:
    /// - `translation_name`: `name` -field in the table `translations`.
    /// - `book`: `book_name` -field in the table `book_name`.
    /// - `chapter`: Desired chapter number.
    /// - `from_verse`: Starting verse.
    /// - `to_verse`: Ending verse. Inclusive.
    pub async fn fetch_some_verses_of_given_chapter(
        &self,
        translation_name: &str,
        book: &str,
        chapter: i16,
        from_verse: i16,
        to_verse: i16,
    ) -> Result<Vec<VerseTextsViewRecord>> {
        let records = sqlx::query_as(
            "SELECT
                language,
                book_id,
                short_book_name,
                full_book_name,
                chapter_number,
                verse_number,
                translation_description,
                translation_name
            FROM verse_texts v
            WHERE
                short_book_name=$1 AND chapter_number=$2 AND verse_number>=$3 AND verse_number<=$4
            AND translation_name=$5")
            .bind(book)
            .bind(chapter)
            .bind(from_verse)
            .bind(to_verse)
            .bind(translation_name)
            .fetch_all(&self.connection_pool)
            .await;
        Ok(records?)
    }


    /// Fetch list of Bible translations in the database.
    pub async fn fetch_translations(&self) -> Result<Vec<TranslationRecord>> {
        let records = sqlx::query_as(
            "SELECT id, language, description, name FROM translations")
            .fetch_all(&self.connection_pool)
            .await;
        Ok(records?)
    }


    /// Fetch list of books
    pub async fn fetch_books_of_translation(&self, language: &str) -> Result<Vec<BooksJoinRecord>> {
        let records = sqlx::query_as(
            "SELECT
            language, short_name, full_name, book_id, color FROM books b JOIN book_name bn
            ON b.id=bn.book_id AND bn.language=$1")
            .bind(language)
            .fetch_all(&self.connection_pool)
            .await;
        Ok(records?)
    }


    /// Fetch number of chapters in a book.
    pub async fn fetch_chapter_count(&self, short_book_name: &str) -> Result<i32> {
        let count = sqlx::query(  // Todo: Fixaa parempi query tähän
                                  "SELECT MAX(chapter_number) FROM verse_texts WHERE short_book_name=$1 LIMIT 1")
            .bind(short_book_name)
            .fetch_one(&self.connection_pool)
            .await?;
        Ok(count.try_get(0)?)
    }


    /// Fetch number of verses in a chapter.
    ///
    /// params:
    /// - `translation` is the `name` field of the `translations` table.
    /// - `short_book_name` is the `short_name` field of the `book_name` table.
    /// - `chapter_number` is the chapter number. Number of chapters can be found using the `fetch_chapter_count(..)` -method.
    pub async fn fetch_verse_count(
        &self, translation: &str, short_book_name: &str, chapter_number: i16,
    ) -> Result<i32> {
        let count = sqlx::query(
            "SELECT max(verse_number) FROM verse_texts WHERE translation_name=$1 AND short_book_name=$2 AND chapter_number=$3"
        ).bind(translation)
            .bind(short_book_name)
            .bind(chapter_number)
            .fetch_one(&self.connection_pool)
            .await?;
        Ok(count.try_get(0)?)
    }
}