use yew::prelude::*;
use yew_router::prelude::*;

use crate::{ChapterPage, ChapterViewPage, FrontPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/:translation/:book")]
    Chapters { translation: String, book: String },
    #[at("/:translation/:book/:chapter")]
    Chapter {
        translation: String,
        book: String,
        chapter: String,
    },
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => html! { <FrontPage /> },
        Route::Chapters { translation, book } => {
            html! { <ChapterPage {translation} {book} /> }
        }
        Route::Chapter {
            translation,
            book,
            chapter,
        } => {
            html! { <ChapterViewPage {translation} {book} {chapter} /> }
        }
    }
}
