mod query_book_list;
mod query_book_meta;
mod query_verses;

pub use query_book_list::*;
pub use query_book_meta::*;
pub use query_verses::Chapter;

use crate::error::LanguageError;

/// lang is an enum in postgres, and it does not accept strings. Thus this is here.
#[derive(Debug)]
pub enum Language {
    Finnish,
    English,
    Hebrew,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Self::Finnish => "fi".to_string(),
            Self::English => "en".to_string(),
            Self::Hebrew => "he".to_string(),
        }
    }
}

impl TryFrom<&String> for Language {
    type Error = LanguageError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "fi" => Ok(Language::Finnish),
            "en" => Ok(Language::English),
            "he" => Ok(Language::Hebrew),
            _ => Err(LanguageError("unreconized language.".to_string())),
        }
    }
}

impl TryFrom<String> for Language {
    type Error = LanguageError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "fi" => Ok(Language::Finnish),
            "en" => Ok(Language::English),
            "he" => Ok(Language::Hebrew),
            _ => Err(LanguageError("unreconized language.".into())),
        }
    }
}

impl TryFrom<&str> for Language {
    type Error = LanguageError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "fi" => Ok(Language::Finnish),
            "en" => Ok(Language::English),
            "he" => Ok(Language::Hebrew),
            _ => Err(LanguageError("unreconized language.".into())),
        }
    }
}
