use gloo_utils::document;
use web_sys::{Element, HtmlInputElement, InputEvent};
use web_sys::wasm_bindgen::JsCast;
use yew::{Callback, Classes, classes, function_component, html, Html, Properties, use_effect_with, use_node_ref, use_state};

#[derive(PartialEq, Properties)]
pub struct ValidatedInputProps {
    pub(crate) id: String,
    #[prop_or_default]
    pub(crate) class: Classes,
    pub(crate) value: String,
    pub(crate) on_input: Callback<String>,
}

#[function_component(ValidatedInput)]
pub fn validated_input(props: &ValidatedInputProps) -> Html {
    let value = use_state(|| props.value.clone());
    let input_ref = use_node_ref();

    {
        let value = value.clone();
        let input_ref = input_ref.clone();

        use_effect_with(
            props.value.clone(),
            move |value_new| {
                if let (Some(input), Some(active_element)) = (input_ref.cast::<Element>(), document().active_element()) {
                    let input: Element = input;

                    if active_element == input {
                        return;
                    }

                    value.set(value_new.clone());
                }
            },
        );
    }
    
    let on_blur = {
        let value_new = props.value.clone();
        let value = value.clone();
        props.on_input.reform(move |e: html::onblur::Event| {
            value.set(value_new.clone());
            e.target().unwrap().unchecked_into::<HtmlInputElement>().value()
        })
    };

    html! {
        <input
            id={props.id.clone()}
            class={classes!("bg-neutral-800", "text-right", "font-mono", "text-lg", "py-1", "px-3", "border", "border-neutral-500", "rounded", props.class.clone())}
            type="text"
            value={(*value).clone()}
            onblur={on_blur}
            oninput={Callback::from(move |e: InputEvent| {
                value.set(e.target().unwrap().unchecked_into::<HtmlInputElement>().value());
            })}
            ref={input_ref}
        />
    }
}