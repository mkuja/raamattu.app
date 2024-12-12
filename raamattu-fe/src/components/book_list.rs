use yew::prelude::*;

use crate::components::{LinkButton, LinkButtonContainer};

use crate::hooks::{use_book_list, UseBookListStateVars};

#[function_component(BookList)]
pub fn book_list() -> Html {
    let books: UseStateHandle<UseBookListStateVars> = use_book_list();

    html! {
        <LinkButtonContainer>
            {books.clone().books.iter().map(|book| html!
                { <LinkButton text={"lala"}/> })
            }
        </LinkButtonContainer>
    }
}
// <LinkButton text={&book.full_name.clone()}/>
