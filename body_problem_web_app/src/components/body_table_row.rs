use web_sys::{FocusEvent, HtmlInputElement, InputEvent};
use web_sys::wasm_bindgen::JsCast;
use yew::{Callback, function_component, Html, html, Properties};

use crate::components::button::Button;
use crate::models::rendered_body::RenderedBody;

#[derive(PartialEq, Properties)]
pub struct BodyTableRowProps {
    pub(crate) rendered_body: RenderedBody,
    pub(crate) edit_allowed: bool,
    pub(crate) edit_callback: Callback<RenderedBody>,
    pub(crate) remove_callback: Callback<usize>,
}

#[function_component]
pub fn BodyTableRow(props: &BodyTableRowProps) -> Html {
    let rendered_body = &props.rendered_body;

    html! {
        <tr class="font-mono text-lg divide-x divide-neutral-600">
            <td class="py-2 px-4 text-center">
                {rendered_body.index + 1}
            </td>
            <td class="px-4 align-middle text-center">
            {
                if props.edit_allowed {
                    let rendered_body = rendered_body.clone();
                    html! {
                        <input type="color" class="bg-neutral-800 w-12 h-6" value={rendered_body.color.to_string()}
                        oninput={props.edit_callback.reform(move |e: InputEvent| {
                            let mut rendered_body = rendered_body.clone();
                            rendered_body.color = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
                            rendered_body
                        })}
                        />
                    }
                } else {
                    html! {
                        <div style={format!("background-color: {};", rendered_body.color)} class="w-12 h-6 m-auto"></div>
                    }
                }
            }
            </td>
            {
                if props.edit_allowed {
                    let rendered_body = rendered_body.clone();
                    html! {
                        <td>
                            <input type="text" class="bg-neutral-800 py-1 px-3 text-right min-w-full" value={rendered_body.body.mass.to_string()}
                            onblur={props.edit_callback.reform(move |e: FocusEvent| {
                                let mut rendered_body = rendered_body.clone();
                                rendered_body.body.mass = e.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(rendered_body.body.mass);
                                rendered_body
                            })}
                            />
                        </td>
                    }
                } else {
                    html! {
                        <td class="py-2 px-4 text-right">{rendered_body.body.mass}</td>
                    }
                }
            }
            {
                if props.edit_allowed {
                    let rendered_body = rendered_body.clone();
                    html! {
                        <td>
                            <input type="text" class="bg-neutral-800 py-1 px-3 text-right min-w-full" value={rendered_body.body.position.x.to_string()}
                            onblur={props.edit_callback.reform(move |e: FocusEvent| {
                                let mut rendered_body = rendered_body.clone();
                                rendered_body.body.position.x = e.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(rendered_body.body.position.x);
                                rendered_body
                            })}
                            />
                        </td>
                    }
                } else {
                    html! {
                        <td class="py-2 px-4 text-right">{format!("{:\u{00a0}>10.1}", rendered_body.body.position.x)}</td>
                    }
                }
            }
            {
                if props.edit_allowed {
                    let rendered_body = rendered_body.clone();
                    html! {
                        <td>
                            <input type="text" class="bg-neutral-800 py-1 px-3 text-right min-w-full" value={rendered_body.body.position.y.to_string()}
                            onblur={props.edit_callback.reform(move |e: FocusEvent| {
                                let mut rendered_body = rendered_body.clone();
                                rendered_body.body.position.y = e.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(rendered_body.body.position.y);
                                rendered_body
                            })}
                            />
                        </td>
                    }
                } else {
                    html! {
                        <td class="py-2 px-4 text-right">{format!("{:\u{00a0}>10.1}", rendered_body.body.position.y)}</td>
                    }
                }
            }
            {
                if props.edit_allowed {
                    let rendered_body = rendered_body.clone();
                    html! {
                        <td>
                            <input type="text" class="bg-neutral-800 py-1 px-3 text-right min-w-full" value={rendered_body.body.velocity.x.to_string()}
                            onblur={props.edit_callback.reform(move |e: FocusEvent| {
                                let mut rendered_body = rendered_body.clone();
                                rendered_body.body.velocity.x = e.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(rendered_body.body.velocity.x);
                                rendered_body
                            })}
                            />
                        </td>
                    }
                } else {
                    html! {
                        <td class="py-2 px-4 text-right">{format!("{:\u{00a0}>10.1}", rendered_body.body.velocity.x)}</td>
                    }
                }
            }
            {
                if props.edit_allowed {
                    let rendered_body = rendered_body.clone();
                    html! {
                        <td>
                            <input type="text" class="bg-neutral-800 py-1 px-3 text-right min-w-full" value={rendered_body.body.velocity.y.to_string()}
                            onblur={props.edit_callback.reform(move |e: FocusEvent| {
                                let mut rendered_body = rendered_body.clone();
                                rendered_body.body.velocity.y = e.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse().unwrap_or(rendered_body.body.velocity.y);
                                rendered_body
                            })}
                            />
                        </td>
                    }
                } else {
                    html! {
                        <td class="py-2 px-4 text-right">{format!("{:\u{00a0}>10.1}", rendered_body.body.velocity.y)}</td>
                    }
                }
            }
            <td class="py-2 px-4 text-right">{format!("{:\u{00a0}>30.1}", rendered_body.potential_energy)}</td>
            <td class="py-2 px-4 text-right">{format!("{:\u{00a0}>30.1}", rendered_body.body.kinetic_energy())}</td>
            {
                props.edit_allowed.then(|| {
                    let rendered_body = rendered_body.clone();

                    html! {
                        <td class="p-1 text-center">
                            <Button onclick={props.remove_callback.reform(move |_| rendered_body.index)} class="py-1 px-3">
                                <i class="fa-solid fa-xmark"></i>
                            </Button>
                        </td>
                    }
                })
            }
        </tr>
    }
}
