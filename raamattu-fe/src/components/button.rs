use yew::prelude::*;

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
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let bg = match &props.btype {
        ButtonType::Primary => "bg-primary",
        ButtonType::Secondary => "bg-secondary",
        ButtonType::Inactive => "bg-inactive",
    };
    html! {
        <button class={format!("border-2 focus:bg-rim border-rim rounded-md px-4 py-2 {} {}", bg, &props.class)}>
            {&props.text}
        </button>
    }
}
