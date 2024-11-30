use askama::Template;
use sqlx::FromRow;
use super::filters;

#[derive(Template, FromRow)]
#[template(path="search_help.jinja")]
pub struct SearchHelp {
    pub language: String,
    pub bible_books: Vec<(String, String)>,
}
