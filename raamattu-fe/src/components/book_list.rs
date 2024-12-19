use std::ops::Deref;

use yew::prelude::*;

use crate::components::LinkButton;
use crate::components::LinkButtonContainer;

use crate::hooks::use_translation;
use crate::hooks::{use_book_list, UseBookListStateVars};
use crate::Route;

#[derive(Properties, Clone, PartialEq)]
pub struct BookListProps {
    pub selected_translation: UseStateHandle<String>,
}

#[function_component(BookList)]
pub fn book_list(props: &BookListProps) -> Html {
    // This one is the eventual return value, and contains the stuff below.
    // It is updated when props.selected_translation changes.
    let books: UseBookListStateVars = use_book_list((*props.selected_translation).as_str());

    // The rendered list of books and related state vars..
    let book_list = use_state(|| vec![]);
    let is_loading = &books.clone().is_loading;
    let error = &books.clone().error;

    // Update the displayed book list.
    let bs = books.books.clone();
    let bl = book_list.clone();
    let pc = props.clone();
    use_effect_with((pc.selected_translation, books), move |(trans, _bks)| {
        bl.set(
            bs.iter()
                .map(|b| {
                    html! { <LinkButton text={b.full_name.to_string()} route={Route::Chapters {
                    translation: trans.to_string(), book: b.short_name.clone() }} /> }
                })
                .collect(),
        );
    });

    let loading_msg = use_translation("is_loading");
    let server_error_msg = use_translation("server_error");
    let class: Option<String> = None;

    html! {
        <LinkButtonContainer {class}>

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
