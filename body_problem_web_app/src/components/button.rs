use web_sys::MouseEvent;
use yew::{Callback, function_component, Html, html, classes, Properties, Classes};

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub(crate) children: Html,
    pub(crate) onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub(crate) disabled: bool,
    #[prop_or_default]
    pub(crate) class: Classes,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button class={classes!("hover:bg-neutral-400", "duration-150", "text-neutral-800", "font-semibold", "rounded", if props.disabled {"bg-neutral-400"} else {"bg-white"}, props.class.clone())} onclick={props.onclick.clone()} disabled={props.disabled}>
            { props.children.clone() }
        </button>
    }
}
