use yew::prelude::*;
use yew_router::prelude::*;

use crate::FrontPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    Root,
    //    #[not_found]
    //    #[at("/404")]
    //    NotFound,
}

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Root => html! { <FrontPage /> },
    }
}
