use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub title: AttrValue
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    html!{
        <h1 class="font-cursive text-8xl w-fit mb-8">{&props.title}</h1>
    }
}