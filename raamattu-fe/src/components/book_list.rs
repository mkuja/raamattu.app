use std::ops::Deref;

use rust_i18n::t;
use yew::prelude::*;

use crate::components::LinkButton;
use crate::components::LinkButtonContainer;

use crate::hooks::{use_book_list, UseBookListStateVars};

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
                        html! { <LinkButton text={b.full_name.to_string()} /> }
                    })
                    .collect(),
            );
        });
    };

    html! {
        <LinkButtonContainer>

            if *is_loading.deref() {
                <span>{t!("is-loading")}</span>
            } else if error.is_some() {
                <span>{t!("server-error")}</span>
            } else {
                {for books.books.iter().map(|book| html! { <LinkButton text={book.full_name.to_string()} /> })
                }
            }

        </LinkButtonContainer>
    }
}
