use yew::prelude::*;
use yew_router::prelude::*;

use crate::{ChapterPage, FrontPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    Root,
    #[at("/:translation/:book")]
    Chapters { translation: String, book: String },
}

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Root => html! { <FrontPage /> },
        Routes::Chapters { translation, book } => {
            html! { <ChapterPage {translation} {book} /> }
        }
    }
}
