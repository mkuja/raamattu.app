use std::ops::Deref;

use web_sys::{self, wasm_bindgen::JsCast, HtmlSelectElement};
use yew::{platform::spawn_local, prelude::*};

use crate::{
    components::{DropDown, DropDownMenuItem},
    context::ApplicationOptions,
    hooks::{use_book_translations, use_cross_translations},
};

#[function_component(Options)]
pub fn options() -> Html {
    let lang_items = vec![
        DropDownMenuItem::new("fi", "Suomi"),
        DropDownMenuItem::new("en", "English"),
    ];
    let lang_items = use_state(|| lang_items);

    let (translations, translations_are_loading, translations_error) = use_book_translations();
    let trans_items = use_state(|| {
        translations
            .iter()
            .map(|item| {
                let name = item.name.clone();
                let name_str = name.as_str();
                let description = item.name.clone();
                let description_str = description.as_str();
                DropDownMenuItem::new(name_str, description_str)
            })
            .collect();
    });
    let trans_items_copy = trans_items.clone();
    let translations_copy = translations.clone();

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

    let translation_on_change = Callback::from(move |ev: Event| {
        let trans = ev
            .target()
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .ok()
            .unwrap()
            .value();
    });

    html! {
        <div class="flex flex-row flex-nowrap gap-4">
            <div class="mb-4">
                <DropDown name="lang" id="lang" items={lang_items} on_change={language_on_change} />
            </div>
            <div class="mb-4">
                <DropDown name="trans" id="trans" items={trans_items.as_ref().deref()} on_change={translation_on_change} />
            </div>
        </div>
    }
}
