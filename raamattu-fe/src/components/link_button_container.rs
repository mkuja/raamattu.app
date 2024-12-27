use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinkButtonContainerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: AttrValue,
}

/// LinkButtonContainer is container component for book or chapter links.
#[function_component(LinkButtonContainer)]
pub fn link_button_container(props: &LinkButtonContainerProps) -> Html {
    html! {
        <div class={&props.class}>
            {for props.children.clone()}
        </div>
    }
}
