use std::f64::consts::PI;

use nalgebra::Vector2;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use web_sys::wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;
use yew_hooks::use_interval;

use body_problem::{Body, simulate};

#[function_component(App)]
pub fn app() -> Html {
    let bodies = use_state(|| vec![
        Body::new(1e16, Vector2::new(0f64, 0f64), Vector2::new(0f64, 0f64)),
        Body::new(1e16, Vector2::new(100f64, -100f64), Vector2::new(0f64, 0f64)),
        Body::new(1e16, Vector2::new(-200f64, -100f64), Vector2::new(0f64, 0f64)),
    ]);

    {
        let bodies = bodies.clone();
        use_interval(
            move || {
                let mut bodies_new = (*bodies).clone();
                for _ in 0..10 {
                    bodies_new = simulate(&bodies_new, 0.001f64);
                }
                bodies.set(bodies_new);
            },
            10,
        );
    }

    html! {
        <>
            <BodyCanvas bodies={(*bodies).clone()}/>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct BodyCanvasProps {
    pub bodies: Vec<Body>,
}

#[function_component(BodyCanvas)]
pub fn body_canvas(props: &BodyCanvasProps) -> Html {
    let canvas_ref = use_node_ref();

    use_effect_with(
        canvas_ref.clone(),
        move |canvas_ref| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let canvas: HtmlCanvasElement = canvas;
                let context: CanvasRenderingContext2d = canvas
                    .get_context_with_context_options("2d", &JsValue::from_serde(&serde_json::json!({
                        "alpha": false,
                        "depth": false,
                        "stencil": false,
                    })).unwrap())
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                context.translate((canvas.width() / 2) as f64, (canvas.height() / 2) as f64).unwrap();

                // context.set_fill_style(&"#000000".into());
                // context.fill_rect(-((canvas.width() / 2) as f64), -((canvas.height() / 2) as f64), canvas.width() as f64, canvas.height() as f64);

                context.set_fill_style(&"#ffffff".into());
            }
        },
    );

    use_effect_with(
        (canvas_ref.clone(), props.bodies.clone()),
        move |(canvas_ref, bodies)| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let canvas: HtmlCanvasElement = canvas;
                let context: CanvasRenderingContext2d = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                context.clear_rect(-((canvas.width() / 2) as f64), -((canvas.height() / 2) as f64), canvas.width() as f64, canvas.height() as f64);

                for body in bodies {
                    context.begin_path();
                    context.arc(body.position.x, body.position.y, 10f64, 0f64, 2f64 * PI).unwrap();
                    context.fill();
                    context.close_path();
                }
            }
            || ()
        },
    );

    html! {
        <div>
            <canvas ref={canvas_ref} width={1000} height={500}/>
        </div>
    }
}
