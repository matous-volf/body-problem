use std::f64::consts::PI;
use std::rc::Rc;

use gloo_utils::format::JsValueSerdeExt;
use nalgebra::Vector2;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use web_sys::wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;
use yew_agent::prelude::{use_reactor_subscription, UseReactorSubscriptionHandle};
use yew_agent::reactor::ReactorProvider;

use body_problem::Body;

use crate::reactor::SimulationReactor;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ReactorProvider<SimulationReactor> path="/simulation_reactor.js">
        <main>
            <SimulationPanel/>
        </main>
        </ReactorProvider<SimulationReactor>>
    }
}

#[function_component(SimulationPanel)]
pub fn simulation_panel() -> Html {
    let bodies = use_state(|| Rc::new(vec![
        Body::new(1e16, Vector2::new(0f64, 0f64), Vector2::new(0f64, 0f64)),
        Body::new(1e16, Vector2::new(100f64, -100f64), Vector2::new(0f64, 0f64)),
        Body::new(1e16, Vector2::new(-200f64, -100f64), Vector2::new(0f64, 0f64)),
    ]));

    let simulation_agent: UseReactorSubscriptionHandle<SimulationReactor> = use_reactor_subscription::<SimulationReactor>();

    {
        let simulation_agent = simulation_agent.clone();
        let bodies = bodies.clone();
        use_effect_with((), move |_| {
            simulation_agent.send((*bodies).to_vec());
        });
    }

    let b = simulation_agent.last().map(|b| b.to_vec()).unwrap_or_default();

    html! {
        <>
        <p>
            {b.iter().map(|body| {
                    format!("Body at ({}, {})", body.position.x, body.position.y)
                }).collect::<Vec<String>>().join("\n")
            }
        </p>
            <BodyCanvas bodies={b}/>
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
