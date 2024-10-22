use std::ops::RangeInclusive;
use unicode_segmentation::UnicodeSegmentation;
use itertools::{ChunkBy, Itertools};
use crate::error::Error;
use crate::error::Result;
use crate::types::{BibleReference, Book};


impl BibleReference {
    /// Parses bible reference and fetches content.
    ///
    /// The format is as follows: `book. chapter:verseFrom-verseTo`
    /// For example for kr38 translation: `1Moos. 1:1-3` with the period after book name being
    /// optional. Also, the reference is case-insensitive. Return `Book` with relevant `Chapter`
    /// with specified `Verse`s in it, or `None` in case of invalid reference.
    ///
    /// Book names are `Translation` specific.
    pub async fn get_by_bible_reference(&self, translation_name: &str, reference: &str) -> Option<Book> {
        let possibly_reference_values = Self::try_parse_bible_reference(reference);
        if let Ok(reference_values) = possibly_reference_values {}
        return None;
    }


    /// Returns `Option<(short_book_name, chapter_number, from_verse, Option<to_verse>)>`.
    pub fn try_parse_bible_reference(
        reference: &str,
    ) -> Result<BibleReference> {
        // case 1) No book name => return Error,
        // case 2a) Book name, but no numbers, => return BibleReference{book, None, 0..=u16::MAX}
        // case 2b) Book name, and chapter num,
        // case 2c) Book name, and chapter num, and one verse num,
        // case 2d) Book name, and chapter num, and two verse nums => Return Ok
        // case  _) Any other case return Error.

        if let Some(book_name) = Self::try_parse_bible_book_name(reference) {

            // cases 2b, 2c, 2d
            if let Some(nums) = Self::parse_numbers(&reference) {
                Ok(BibleReference {
                    book_name: book_name.to_string(),
                    chapter_number: Some(nums.0),
                    verses: nums.1,
                })
            } else {  // case 2a
                Ok(BibleReference {
                    book_name: book_name.to_string(),
                    chapter_number: None,
                    verses: 0..=u16::MAX,
                })
            }
        } else {  // case 1
            Err(Box::new(Error::BibleRefError("Book missing from reference")))
        }
    }


    /// Parse book name from the Bible reference.
    fn try_parse_bible_book_name(reference: &str) -> Option<&str> {
        let g = reference.graphemes(true);
        let mut total_len = 0;
        for grapheme in g.into_iter() {
            if grapheme.chars().all(|x| { x.is_alphanumeric() }) {
                total_len = grapheme.chars().fold(total_len, |a, b| { a + b.len_utf8() })
            } else {
                return Some(&reference[0..total_len]);
            }
        }
        if total_len > 0 { Some(&reference[0..total_len]) } else { None }
    }


    /// Parse chapter number, and verse range from the Bible reference.
    ///
    /// Returns `Some((chapter_number, verses_inclusive_range))`  or
    /// `None` in case even chapter number is absent. If both verse numbers are missing,
    /// then verses range equals to `0..=u16::MAX`. If one verse number is missing, then the
    /// range will span one number.
    fn parse_numbers(reference: &str) -> Option<(u16, RangeInclusive<u16>)> {
        let graphemes = reference.graphemes(true)
            .collect::<Vec<&str>>();

        let mut numbers = vec![];
        for (_, chunk) in &graphemes.into_iter().chunk_by(
            |g| g.chars().all(|c| c.is_alphanumeric())
        ) {
            let word_or_number: String = chunk.collect();
            if word_or_number.chars().all(|c| { c.is_numeric() }) {
                numbers.push(word_or_number.parse::<u16>().unwrap());
            }
        }

        if numbers.is_empty() {
            return None;
        }
        let chapter_number = numbers[0];
        if let Some(from_verse) = numbers.get(1) {
            if let Some(to_verse) = numbers.get(2) {
                return Some((chapter_number, *from_verse..=*to_verse));
            }
            return Some((chapter_number, *from_verse..=u16::MAX));
        }
        Some((chapter_number, 0..=u16::MAX))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_parse_bible_references() {
        let refs = vec![
            ("1Moos. 1:1-2", BibleReference {
                book_name: "1Moos".to_string(),
                chapter_number: Some(1),
                verses: 1..=2,
            }),
            ("1Moos", BibleReference {
                book_name: "1Moos".to_string(),
                chapter_number: None,
                verses: 0..=u16::MAX,
            }),
            ("1moos.1:1-2", BibleReference {
                book_name: "1moos".to_string(),
                chapter_number: Some(1),
                verses: 1..=2,
            }),
        ];


        for reference in refs {
            assert!(BibleReference::try_parse_bible_reference(reference.0).is_ok());
            assert_eq!(BibleReference::try_parse_bible_reference(reference.0).unwrap(), reference.1)
        }
    }
}
