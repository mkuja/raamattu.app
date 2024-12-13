use web_sys::{self, wasm_bindgen::JsCast, HtmlSelectElement};
use yew::prelude::*;

use crate::{
    components::{DropDown, DropDownMenuItem},
    context::ApplicationOptions,
};

#[function_component(Options)]
pub fn options() -> Html {
    let lang_items = vec![
        DropDownMenuItem::new("fi", "Suomi"),
        DropDownMenuItem::new("en", "English"),
    ];
    let trans_items = vec![
        DropDownMenuItem::new("kr38", "Kirkkoraamattu 1933/1938"),
        DropDownMenuItem::new("web", "World English Bible"),
    ];

    let ctx = use_context::<UseStateHandle<ApplicationOptions>>().unwrap();

    let ctx_ = ctx.clone();
    let language_on_change = Callback::from(move |ev: Event| {
        let lang = ev
            .target()
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .ok()
            .unwrap()
            .value();
        let new_ctx = ApplicationOptions {
            language: lang,
            ..(*ctx_).clone()
        };
        ctx_.set(new_ctx);
    });

    let ctx_ = ctx.clone();
    let translation_on_change = Callback::from(move |ev: Event| {
        let trans = ev
            .target()
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .ok()
            .unwrap()
            .value();
        let new_ctx = ApplicationOptions {
            translation: trans,
            ..(*ctx_).clone()
        };
        ctx_.set(new_ctx)
    });

    html! {
        <div class="flex flex-row flex-nowrap gap-4">
            <div class="mb-4">
                <DropDown name="lang" id="lang" items={lang_items} on_change={language_on_change} />
            </div>
            <div class="mb-4">
                <DropDown name="trans" id="trans" items={trans_items} on_change={translation_on_change} />
            </div>
        </div>
    }
}
