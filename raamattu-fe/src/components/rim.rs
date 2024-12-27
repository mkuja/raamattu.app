use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RimProps {
    pub children: Children,
}

#[function_component(Rim)]
pub fn rim(props: &RimProps) -> Html {
    html! {
        <div class="rounded-md p-4 w-full border-2 border-hilight mt-2">
            {props.children.clone()}
        </div>
    }
}
