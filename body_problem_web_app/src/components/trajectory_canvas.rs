use gloo_console::__macro::JsValue;
use gloo_events::EventListener;
use gloo_utils::format::JsValueSerdeExt;
use nalgebra::Vector2;
use web_sys::{CanvasRenderingContext2d, Event, HtmlCanvasElement, window};
use web_sys::wasm_bindgen::JsCast;
use yew::{Callback, function_component, Html, html, Properties, use_effect_with, use_node_ref, use_state};
use crate::models::RenderedBody;

#[derive(Properties, PartialEq)]
pub struct TrajectoryCanvasProps {
    pub(crate) rendered_bodies: Vec<RenderedBody>,
}

#[function_component(TrajectoryCanvas)]
pub fn trajectory_canvas(props: &TrajectoryCanvasProps) -> Html {
    let canvas_ref = use_node_ref();
    let canvas = canvas_ref.cast::<HtmlCanvasElement>();
    let context = use_state(|| None);
    let body_positions = use_state(|| vec![props.rendered_bodies.iter().map(|rendered_body| rendered_body.body.position).collect::<Vec<Vector2<f64>>>()]);

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
                                "alpha": false,
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
    
    if let (Some(context), Some(canvas)) = ((*context).clone(), canvas) {
        let context: CanvasRenderingContext2d = context;
        if props.rendered_bodies.iter().any(|rendered_body|
        (rendered_body.body.position - (*body_positions).last().unwrap()[rendered_body.index]).norm() > 5f64) {
            let mut body_positions_new: Vec<Vec<Vector2<f64>>> = (*body_positions).clone();
            body_positions_new.push(props.rendered_bodies.iter().map(|rendered_body| rendered_body.body.position).collect());

            context.clear_rect(-((canvas.width() / 2) as f64), -((canvas.height() / 2) as f64), canvas.width() as f64, canvas.height() as f64);

            // reversing for a more intuitive layer order
            for (body_index, rendered_body) in (0..body_positions_new[0].len()).map(|body_index| (body_index, &props.rendered_bodies[body_index])) {
                let starting_position = body_positions_new.first().unwrap()[body_index];
                context.set_stroke_style(&rendered_body.color.as_str().into());
                context.begin_path();
                context.move_to(starting_position.x, -starting_position.y);
                for position in body_positions_new.iter().map(|positions| positions[body_index]).skip(1) {
                    context.line_to(position.x, -position.y);
                }
                context.stroke();
            }
            
            // for (positions1, positions2) in body_positions_new.iter().tuple_windows() {
            //     for (position1, position2) in positions1.iter().zip(positions2) {
            //         
            //     }
            // }

            body_positions.set(body_positions_new);
        }
    }

    html! {
        <canvas ref={canvas_ref} height="700" class="absolute"/>
    }
}
