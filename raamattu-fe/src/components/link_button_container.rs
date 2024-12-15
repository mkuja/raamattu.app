use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinkButtonContainerProps {
    #[prop_or_default]
    pub children: Children,
    pub class: Option<String>,
}

/// LinkButtonContainer is container component for book or chapter links.
#[function_component(LinkButtonContainer)]
pub fn link_button_container(props: &LinkButtonContainerProps) -> Html {
    let styles = match &props.class {
        Some(s) => s.clone(),
        None => String::from("grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 border-2 rounded-md p-4 border-hilight mt-2")
    };

    html! {
        <div class={styles}>
            {for props.children.clone()}
        </div>
    }
}
