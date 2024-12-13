use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct LoadingOrErrorProps {
    msg: AttrValue,
    is_error: bool,
}

#[function_component(LoadingOrError)]
pub fn loading_or_error(props: &LoadingOrErrorProps) -> Html {
    html! {
        <span class="">
            {&props.msg}
        </span>
    }
}
