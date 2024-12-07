use yew::prelude::*;

use super::LinkButton;

#[derive(Properties, PartialEq)]
pub struct LinkButtonProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<LinkButton>,
}

#[function_component(LinkButtonContainer)]
pub fn link_button_container(props: &LinkButtonProps) -> Html {
    html! {
        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 border-2 rounded-md p-4 border-hilight mt-2">
            {for &mut props.children.iter()}
        </div>
    }
}
