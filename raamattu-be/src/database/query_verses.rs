use axum::http::StatusCode;
use serde::Serialize;
use sqlx::Row;

use crate::BackendState;

#[derive(Serialize)]
pub struct Verse {
    verse_number: i32,
    verse_text: String,
}

impl Verse {
    #[allow(unused)]
    pub fn verse_number(&self) -> i32 {
        self.verse_number
    }
    #[allow(unused)]
    pub fn verse_text(&self) -> String {
        self.verse_text.clone()
    }
    #[allow(unused)]
    pub fn verse_text_as_str(&self) -> &str {
        &self.verse_text.as_str()
    }
}

#[derive(Serialize)]
pub struct Chapter {
    pub language: String,
    pub book_id: i32,
    pub short_book_name: String,
    pub full_book_name: String,
    pub chapter_number: i32,
    pub translation_description: String,
    pub translation_name: String,
    pub verses: Vec<Verse>,
}

impl BackendState {
    pub async fn fetch_chapter_verses(
        &self,
        translation: &str,
        book: &str,
        chapter: i32,
    ) -> Result<Chapter, StatusCode> {
        let tmp  = sqlx::query("select language::TEXT as language,
                                book_id,
                                short_book_name,
                                full_book_name,
                                chapter_number,
                                verse_number,
                                translation_description,
                                translation_name,
                                verse_text
                        from verse_texts vt where vt.translation_name=$1 and vt.short_book_name=$2 and vt.chapter_number=$3;")
            .bind(translation)
            .bind(book)
            .bind(chapter)
            .fetch_all(&self.database_connection)
            .await;

        match tmp {
            Ok(verses_db) => {
                if verses_db.len() > 0 {
                    let mut chapter = Chapter {
                        language: verses_db[0].get("language"),
                        book_id: verses_db[0].get("book_id"),
                        short_book_name: verses_db[0].get("short_book_name"),
                        full_book_name: verses_db[0].get("full_book_name"),
                        chapter_number: verses_db[0].get("chapter_number"),
                        translation_description: verses_db[0].get("translation_description"),
                        translation_name: verses_db[0].get("translation_name"),
                        verses: vec![],
                    };
                    let verses = verses_db.into_iter().map(|v| Verse {
                        verse_number: v.get("verse_number"),
                        verse_text: v.get("verse_text"),
                    });
                    chapter.verses.extend(verses);
                    Ok(chapter)
                } else {
                    Err(StatusCode::BAD_REQUEST)
                }
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
