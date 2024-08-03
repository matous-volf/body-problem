use std::f64::consts::PI;

use web_sys::HtmlCanvasElement;
use yew::{function_component, Html, html, Properties, use_context, use_effect_with, use_node_ref, use_state};
use crate::models::rendered_body::RenderedBody;
use crate::models::settings::Settings;
use crate::utils::{CanvasClear, SimulationCanvasInitialize};

#[derive(Properties, PartialEq)]
pub struct BodyCanvasProps {
    pub(crate) rendered_bodies: Vec<RenderedBody>,
}

#[function_component(BodyCanvas)]
pub fn body_canvas(props: &BodyCanvasProps) -> Html {
    let canvas_ref = use_node_ref();
    let canvas = canvas_ref.cast::<HtmlCanvasElement>();
    let context = use_state(|| None);
    let settings = use_context::<Settings>().unwrap();

    {
        let context = context.clone();
        // on each change of the canvas (e.g. window resize)
        use_effect_with(
            canvas.clone(),
            move |canvas| {
                let mut resize_listener = None;
                if let Some(canvas) = canvas {
                    let canvas: HtmlCanvasElement = canvas.clone();
                    resize_listener = Some(canvas.initialize_for_simulation(context, true));
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
            let circle_radius = settings.body_circle_radius * if settings.scale_body_circles_with_mass {
                rendered_body.body.mass.cbrt()
            } else {
                1f64
            };
            context.arc(rendered_body.body.position.x, -rendered_body.body.position.y, circle_radius, 0f64, 2f64 * PI).unwrap();
            context.fill();
            context.close_path();
        }
    }

    html! {
        <canvas ref={canvas_ref} height="700" class="absolute"/>
    }
}
