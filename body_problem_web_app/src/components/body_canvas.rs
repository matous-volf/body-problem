use std::f64::consts::PI;
use gloo_console::__macro::JsValue;
use gloo_events::EventListener;
use gloo_utils::format::JsValueSerdeExt;
use web_sys::{CanvasRenderingContext2d, Event, HtmlCanvasElement, window};
use web_sys::wasm_bindgen::JsCast;
use yew::{Callback, function_component, Html, html, Properties, use_effect_with, use_node_ref, use_state};
use crate::models::RenderedBody;
use crate::utils::CanvasClear;

#[derive(Properties, PartialEq)]
pub struct BodyCanvasProps {
    pub(crate) rendered_bodies: Vec<RenderedBody>,
}

#[function_component(BodyCanvas)]
pub fn body_canvas(props: &BodyCanvasProps) -> Html {
    let canvas_ref = use_node_ref();
    let canvas = canvas_ref.cast::<HtmlCanvasElement>();
    let context = use_state(|| None);

    {
        let context = context.clone();
        // on each change of the canvas (e.g. window resize)
        use_effect_with(
            canvas.clone(),
            move |canvas| {
                let mut resize_listener = None;
                if let Some(canvas) = canvas {
                    let canvas: HtmlCanvasElement = canvas.clone();

                    let initialize_context = Callback::from(move |_: Event| {
                        canvas.set_width(window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32);

                        let context_new: CanvasRenderingContext2d = canvas
                            .get_context_with_context_options("2d", &JsValue::from_serde(&serde_json::json!({
                                "alpha": true,
                                "depth": false,
                                "stencil": false,
                            })).unwrap())
                            .unwrap()
                            .unwrap()
                            .dyn_into::<CanvasRenderingContext2d>()
                            .unwrap();

                        context_new.translate((canvas.width() / 2) as f64, (canvas.height() / 2) as f64).unwrap();

                        context.set(Some(context_new));
                    });

                    initialize_context.emit(Event::new("").unwrap());

                    resize_listener = Some(EventListener::new(
                        &window().unwrap(),
                        "resize",
                        move |e| initialize_context.emit(e.clone()),
                    ));
                }

                move || drop(resize_listener)
            },
        );
    }

    if let (Some(context), Some(_)) = ((*context).clone(), canvas) {
        context.clear().unwrap();

        // reversing for a more intuitive layer order
        for rendered_body in props.rendered_bodies.iter().rev() {
            context.set_fill_style(&rendered_body.color.as_str().into());
            context.begin_path();
            context.arc(rendered_body.body.position.x, -rendered_body.body.position.y, 10f64, 0f64, 2f64 * PI).unwrap();
            context.fill();
            context.close_path();
        }
    }

    html! {
        <canvas ref={canvas_ref} height="700" class="absolute"/>
    }
}
