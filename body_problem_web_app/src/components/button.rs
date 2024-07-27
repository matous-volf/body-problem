use web_sys::MouseEvent;
use yew::{Callback, function_component, Html, html, classes, Properties};

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub(crate) children: Html,
    pub(crate) onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub(crate) disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button class={classes!("hover:bg-neutral-400", "duration-150", "text-neutral-800", "font-bold", "py-2", "px-4", "rounded", if props.disabled {"bg-neutral-400"} else {"bg-white"})} onclick={props.onclick.clone()} disabled={props.disabled}>
            { props.children.clone() }
        </button>
    }
}
