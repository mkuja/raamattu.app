use std::ops::Deref;

use yew::prelude::*;

use crate::components::LinkButton;
use crate::components::LinkButtonContainer;

use crate::hooks::use_translation;
use crate::hooks::{use_book_list, UseBookListStateVars};
use crate::Route;

#[function_component(BookList)]
pub fn book_list() -> Html {
    let books: UseBookListStateVars = use_book_list();

    let book_list = use_state(|| vec![]);
    let is_loading = &books.clone().is_loading;
    let error = &books.clone().error;

    {
        let book_list = book_list.clone();
        let books = books.clone();

        use_effect_with(books, move |bks| {
            book_list.set(
                bks.books
                    .iter()
                    .map(|b| {
                        html! { <LinkButton text={b.full_name.to_string()} route={Route::Chapters {
                        translation: b.translation.clone(), book: b.short_name.clone() }} /> }
                    })
                    .collect(),
            );
        });
    };

    let loading_msg = use_translation("is_loading");
    let server_error_msg = use_translation("server_error");

    html! {
        <LinkButtonContainer>

            if *is_loading.deref() {
                <span>{loading_msg.get_translation()}</span>
            } else if error.is_some() {
                <span>{server_error_msg.get_translation()}</span>
            } else {
                {for book_list.deref().into_iter().map(|b|{html!{b.clone()}})}
            }

        </LinkButtonContainer>
    }
}
