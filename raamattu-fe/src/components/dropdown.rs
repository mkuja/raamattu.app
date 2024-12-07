use web_sys::{self, wasm_bindgen::JsCast, HtmlSelectElement};
use yew::prelude::*;

use crate::context::ApplicationOptions;

/**
 * A Vec of these must be provided for DropDown in props `items`
 */
#[derive(Properties, PartialEq)]
pub struct DropDownMenuItem {
    pub value: AttrValue,
    pub display_value: AttrValue,
}

impl DropDownMenuItem {
    /**
     * Create a new DropDownMenuItem
     */
    pub fn new(value: impl Into<AttrValue>, display_value: impl Into<AttrValue>) -> Self {
        return Self {
            value: value.into(),
            display_value: display_value.into(),
        };
    }
}

#[derive(Properties, PartialEq)]
pub struct DropDownProps {
    pub items: Vec<DropDownMenuItem>,
    pub name: AttrValue,
    pub id: AttrValue,
}

#[function_component(DropDown)]
/**
 * Also import `DropDownMenuItem` to use with this component.
 */
pub fn drop_down(props: &DropDownProps) -> Html {
    let ctx = use_context::<UseStateHandle<ApplicationOptions>>();

    let onchange = Callback::from(move |ev: Event| {
        let lang = ev
            .target()
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .ok()
            .unwrap()
            .value();
        let new_ctx = ApplicationOptions {
            language: lang,
            ..*ctx.clone().unwrap()
        };
        ctx.as_ref().unwrap().set(new_ctx);
    });

    html! {
        <select class="p-1 bg-secondary border-2 border-rim rounded-md" name={&props.name} id={&props.id} {onchange}>
            {for props.items.iter().map(|item| {
                html!{
                    <option value={&item.value}>{&item.display_value}</option>
                }
            }).into_iter()}
        </select>
    }
}
