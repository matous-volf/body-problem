use nalgebra::Vector2;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{function_component, Html, html, Properties, use_effect_with, use_node_ref, use_state};
use crate::models::rendered_body::RenderedBody;
use crate::utils::{CanvasClear, SimulationCanvasInitialize};

const TRAJECTORY_MAX_SEGMENT_LENGTH: f64 = 5f64;

#[derive(Properties, PartialEq)]
pub struct TrajectoryCanvasProps {
    pub(crate) rendered_bodies: Vec<RenderedBody>,
    pub(crate) rendered_bodies_edited_this_pause: bool,
    pub(crate) simulation_paused: bool,
    pub(crate) simulation_reset: bool,
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
                    resize_listener = Some(canvas.initialize_for_simulation(context, false));
                }

                move || drop(resize_listener)
            },
        );
    }

    let reset = {
        let body_positions = body_positions.clone();
        let context = (*context).clone();
        move || {
            body_positions.set(Vec::new());
            if let Some(context) = context {
                context.clear().unwrap();
            }
        }
    };

    {
        let reset = reset.clone();
        use_effect_with(
            props.rendered_bodies_edited_this_pause,
            move |&rendered_bodies_edited_this_pause| {
                if rendered_bodies_edited_this_pause {
                    reset();
                }
            },
        );
    }

    {
        let reset = reset.clone();
        use_effect_with(
            props.simulation_reset,
            move |&simulation_reset| {
                if simulation_reset {
                    reset();
                }
            },
        );
    }

    if let (Some(context), Some(_)) = ((*context).clone(), canvas) {
        let context: CanvasRenderingContext2d = context;
        if !props.simulation_paused
            && !props.simulation_reset
            && ((*body_positions).is_empty()
            || props.rendered_bodies.iter().any(|rendered_body|
        (rendered_body.body.position - (*body_positions).last().unwrap()[rendered_body.index]).norm() > TRAJECTORY_MAX_SEGMENT_LENGTH)) {
            let mut body_positions_new: Vec<Vec<Vector2<f64>>> = (*body_positions).clone();
            body_positions_new.push(props.rendered_bodies.iter().map(|rendered_body| rendered_body.body.position).collect());

            context.clear().unwrap();

            // reversing for a more intuitive layer order
            for (body_index, rendered_body) in (0..body_positions_new[0].len()).map(|body_index| (body_index, &props.rendered_bodies[body_index])).rev() {
                let starting_position = body_positions_new.first().unwrap()[body_index];
                context.set_stroke_style(&rendered_body.color.as_str().into());
                context.begin_path();
                context.move_to(starting_position.x, -starting_position.y);
                for position in body_positions_new.iter().map(|positions| positions[body_index]).skip(1) {
                    context.line_to(position.x, -position.y);
                }
                context.stroke();
            }

            body_positions.set(body_positions_new);
        }
    }

    html! {
        <canvas ref={canvas_ref} height="700" class="absolute"/>
    }
}
