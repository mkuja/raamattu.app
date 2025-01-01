use std::ops::Deref;

use gloo_storage::{LocalStorage, Storage};
use log::{info, warn};
use web_sys::{self, wasm_bindgen::JsCast, HtmlSelectElement};
use yew::prelude::*;

use crate::{
    components::{DropDown, DropDownMenuItem},
    context::ApplicationOptions,
    hooks::{use_book_translations, use_translation},
};

#[derive(Properties, Clone, PartialEq)]
pub struct OptionsProps {
    /// Used to lift the selected translations to parent component.
    pub selected_translation: UseStateHandle<AttrValue>,
    #[prop_or_default]
    pub selected_book: Option<AttrValue>,
    #[prop_or_default]
    pub show_save_defaults: bool,
}

#[function_component(Options)]
pub fn options(props: &OptionsProps) -> Html {
    // ctx is used on some occasions, so it's common for the page.
    let ctx = use_context::<UseStateHandle<ApplicationOptions>>().unwrap();

    // Languages are a client-side thing, and defined here is the list.
    let ctx_ = ctx.clone();
    let lang_items = use_state(|| {
        let active_language = ctx_.language.clone();
        vec![
            DropDownMenuItem::new("fi", "Suomi", "fi" == active_language.as_str()),
            DropDownMenuItem::new("en", "English", "en" == active_language.as_str()),
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
    let translations_ = translations.clone();
    let trans_items_ = trans_items.clone();
    let ctx_ = ctx.clone();
    use_effect_with((translations_, ctx_), move |(t, ctx)| {
        info!("{}", ctx.deref());
        trans_items_.set(
            t.iter()
                .map(|item| {
                    DropDownMenuItem::new(
                        item.name.as_str(),
                        item.description.as_str(),
                        item.name.as_str() == ctx.translation.as_str(),
                    )
                })
                .collect(),
        );
    });

    // Update active language option of options selects, eg. on settings load from localstorage.
    let ctx_ = ctx.clone();
    let lang_items_ = lang_items.clone();
    use_effect_with(ctx_, move |ctx| {
        lang_items_.set(
            lang_items_
                .iter()
                .map(|item| {
                    DropDownMenuItem::new(
                        item.value.clone(),
                        item.display_value.clone(),
                        item.value == ctx.language,
                    )
                })
                .collect(),
        );
    });

    // CB for when user changes the selected translation.
    let selected_trans = props.selected_translation.clone();
    let ctx_ = ctx.clone();
    let translation_on_change = Callback::from(move |ev: Event| {
        let trans = ev
            .target()
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .ok()
            .unwrap()
            .value();
        let trans_ = trans.clone();
        selected_trans.set(trans.into());
        warn!("{}", trans_);
        ctx_.set(ApplicationOptions {
            translation: trans_,
            ..(*ctx_).clone()
        })
    });

    let save_changes = use_translation("save_changes");

    // Save save settings
    let ctx_ = ctx.clone();
    let onclick = Callback::from(move |_ev: MouseEvent| {
        let success = LocalStorage::set("settings", (*ctx_).clone());
        if let Ok(_success) = success {
            info!("Saved settings: {}", *ctx_);
        } else {
            info!("Could not save settings..");
        }
    });

    html! {
        <div class={format!("grid w-full mb-4 gap-4 {}", if props.show_save_defaults {"grid-cols-3"} else {"grid-cols-2"})}>
            <div class="min-w-fit">
                <DropDown class="h-full" name="lang" id="lang" items={(*lang_items).clone()} on_change={language_on_change} />
            </div>
            <div class="">
                <DropDown class="h-full" name="trans" id="trans" items={(*trans_items).clone()} on_change={translation_on_change} />
            </div>
            if props.show_save_defaults {
                <div class="">
                    <button onclick={&onclick} class="p-1 px-2 w-full bg-secondary border-2 border-rim rounded-md" type="button">{save_changes.get_translation()}</button>
                </div>
            }
        </div>
    }
}
