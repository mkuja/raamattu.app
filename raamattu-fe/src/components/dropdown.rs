use yew::prelude::*;

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
    pub on_change: Callback<Event>,
}

#[function_component(DropDown)]
/**
 * Also import `DropDownMenuItem` to use with this component.
 */
pub fn drop_down(props: &DropDownProps) -> Html {
    html! {
        <select class="p-1 bg-secondary border-2 border-rim rounded-md" name={&props.name} id={&props.id} onchange={&props.on_change}>
            {for props.items.iter().map(|item| {
                html!{
                    <option value={&item.value}>{&item.display_value}</option>
                }
            }).into_iter()}
        </select>
    }
}
