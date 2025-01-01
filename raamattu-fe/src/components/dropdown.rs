use yew::prelude::*;

/**
 * A Vec of these must be provided for DropDown in props `items`
 */
#[derive(Properties, PartialEq, Clone)]
pub struct DropDownMenuItem {
    pub value: String,
    pub display_value: String,
    #[prop_or_default]
    pub selected: bool,
}

impl DropDownMenuItem {
    /**
     * Create a new DropDownMenuItem
     */
    pub fn new(value: impl Into<String>, display_value: impl Into<String>, selected: bool) -> Self {
        return Self {
            value: value.into(),
            display_value: display_value.into(),
            selected,
        };
    }
}

#[derive(Properties, PartialEq)]
pub struct DropDownProps {
    pub items: Vec<DropDownMenuItem>,
    pub name: AttrValue,
    pub id: AttrValue,
    pub on_change: Callback<Event>,
    #[prop_or_default]
    pub class: AttrValue,
}

#[function_component(DropDown)]
/**
 * Also import `DropDownMenuItem` to use with this component.
 */
pub fn drop_down(props: &DropDownProps) -> Html {
    html! {
        <select class={format!("p-1 w-full bg-secondary border-2 border-rim rounded-md {}",
            &props.class)}
            name={&props.name}
            id={&props.id}
            onchange={&props.on_change}>
            {for props.items.iter().map(|item| {
                html!{
                    <option selected={item.selected} value={item.value.to_owned()}>{&item.display_value}</option>
                }
            }).into_iter()}
        </select>
    }
}
