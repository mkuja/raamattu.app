use std::ops::{RangeInclusive};
use serde::{Serialize};
use sqlx::FromRow;

/// This is the enum in postgres for translation language
#[derive(Debug, sqlx::Type, Serialize, PartialEq, Eq)]
#[sqlx(type_name = "lang", rename_all = "lowercase")]
pub enum Lang {
    Fi,
    He,
    En,
}

/// Book data. This is used by the front page template.
#[derive(FromRow, Debug, PartialEq, Eq, Serialize)]
pub struct Book {
    pub book_id: i32,
    pub book_color: String,
    pub short_name: String,
    pub full_name: String,
    pub language: Lang,
    pub translation: String,
    pub translation_description: String,
}

/// Chapter count for a book. This is used by the chapter numbers template.
#[derive(FromRow, Debug, PartialEq, Eq, Serialize)]
pub struct NumChapters {
    pub book_id: i32,
    pub full_name: String,
    pub short_name: String,
    pub lang: Lang,
    pub translation: String,
    pub translation_description: String,
    pub num_chapters: i32,
}

/// This identifies a verse or range of verses from the Bible.
///
/// It isn't guaranteed to be valid, as `book_name` could be anything and `verses` or
/// `chapter_number` could be invalid. This is because book name depends on which `Translation`
/// is considered.
///
/// `BibleReference` merely has methods to try parse a `&str` and fetch the corresponding verses
/// from database of given `Translation`, which is identified by its abbreviation.
#[derive(Debug, PartialEq, Eq)]
pub struct BibleReference {
    pub book_name: String,
    pub chapter_number: Option<u16>,
    pub verses: RangeInclusive<u16>,
}


/// This struct has same fields as database `verse_view` -view.
#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct VerseTextsViewRecord {
    pub language: String,
    pub book_id: i32,
    pub short_book_name: String,
    pub full_book_name: String,
    pub chapter_number: i32,
    pub verse_number: i32,
    pub translation_description: String,
    pub translation_name: String,
}


/// This struct has same fields as database `translations` -table.
#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct TranslationRecord {
    pub id: i32,
    pub language: String,
    pub description: String,
    pub name: String,
}


/// This struct has same fields as database `books` joined with `book_name` -table.
#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct BooksJoinRecord {
    pub language: String,
    pub short_name: String,
    pub full_name: String,
    pub book_id: i32,
    pub color: String,
}