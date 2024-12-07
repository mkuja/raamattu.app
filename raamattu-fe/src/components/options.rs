use yew::prelude::*;

use crate::components::{DropDown, DropDownMenuItem};

#[function_component(Options)]
pub fn options() -> Html {
    let items = vec![
        DropDownMenuItem::new("fi", "Suomi"),
        DropDownMenuItem::new("en", "English"),
    ];

    html! {
        <div class="mb-4">
            <DropDown name="lang" id="lang" items={items} />
        </div>
    }
}
