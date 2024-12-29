use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(PartialEq)]
pub enum ButtonType {
    Primary,
    #[allow(unused)]
    Secondary,
    #[allow(unused)]
    Inactive,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub text: AttrValue,
    pub btype: ButtonType,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub svg_icon: Option<IconId>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let bg = match &props.btype {
        ButtonType::Primary => {
            if props.disabled {
                "bg-inactive"
            } else {
                "bg-primary"
            }
        }
        ButtonType::Secondary => {
            if props.disabled {
                "bg-inactive"
            } else {
                "bg-secondary"
            }
        }
        ButtonType::Inactive => "bg-inactive",
    };
    html! {
        <button disabled={props.disabled} class={
            format!("border-2 flex gap-2 border-rim rounded-md px-4 py-2 {} {} {}",
                bg,
                &props.class,
                if props.disabled {""} else {"hover:border-hilight"}
                )}
        >
            {if props.svg_icon.is_some() {
                html!{ <Icon icon_id={props.svg_icon.unwrap()}/> }
            } else { html! {<></>} }}
            {&props.text}
        </button>
    }
}
