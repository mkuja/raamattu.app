use web_sys::{self, wasm_bindgen::JsCast, HtmlSelectElement};
use yew::prelude::*;

use crate::{
    components::{DropDown, DropDownMenuItem},
    context::ApplicationOptions,
    hooks::use_book_translations,
};

#[derive(Properties, Clone, PartialEq)]
pub struct OptionsProps {
    /// Used to lift the selected translations to parent component.
    pub selected_translation: UseStateHandle<String>,
}

#[function_component(Options)]
pub fn options(props: &OptionsProps) -> Html {
    // ctx is used on some occasions, so it's common for the page.
    let ctx = use_context::<UseStateHandle<ApplicationOptions>>().unwrap();

    // Languages are a client-side thing, and defined here is the list.
    let lang_items = use_state(|| {
        vec![
            DropDownMenuItem::new("fi", "Suomi"),
            DropDownMenuItem::new("en", "English"),
        ]
    });

    // CB for when user changes language.
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

    // Translations come from the backend.
    let (translations, _translations_are_loading, _translations_error) = use_book_translations();
    let trans_items: UseStateHandle<Vec<DropDownMenuItem>> = use_state(move || vec![]);

    // Pointers cloning along with effect for updating the translations vec.
    let t = translations.clone();
    let ti = trans_items.clone();
    use_effect_with(translations, move |_| {
        ti.set(
            t.iter()
                .map(|item| DropDownMenuItem::new(item.name.as_str(), item.description.as_str()))
                .collect(),
        );
    });

    // CB for when user changes the selected translation.
    let selected_trans = props.selected_translation.clone();
    let translation_on_change = Callback::from(move |ev: Event| {
        let trans = ev
            .target()
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .ok()
            .unwrap()
            .value();
        selected_trans.set(trans);
    });

    html! {
        <div class="flex flex-row flex-nowrap gap-4">
            <div class="mb-4">
                <DropDown name="lang" id="lang" items={(*lang_items).clone()} on_change={language_on_change} />
            </div>
            <div class="mb-4">
                <DropDown name="trans" id="trans" items={(*trans_items).clone()} on_change={translation_on_change} />
            </div>
        </div>
    }
}
