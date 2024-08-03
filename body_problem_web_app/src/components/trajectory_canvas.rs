use std::collections::VecDeque;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use web_time::Duration;
use yew::{function_component, Html, html, Properties, use_context, use_effect_with, use_node_ref, use_state};

use crate::models::rendered_body::RenderedBody;
use crate::models::settings::Settings;
use crate::models::trajectory_segment::TrajectorySegment;
use crate::utils::{CanvasClear, SimulationCanvasInitialize};

const TRAJECTORY_MAX_SEGMENT_LENGTH: f64 = 0.5f64;

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
    let trajectory_segments = use_state(|| VecDeque::from([TrajectorySegment::new(props.rendered_bodies.iter().map(|rendered_body| rendered_body.body.position).collect())]));
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
                    resize_listener = Some(canvas.initialize_for_simulation(context, false));
                }

                move || drop(resize_listener)
            },
        );
    }

    let reset = {
        let body_positions = trajectory_segments.clone();
        let context = (*context).clone();
        move || {
            body_positions.set(VecDeque::new());
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

        if settings.trajectory_duration <= Duration::ZERO {
            context.clear().unwrap();
        } else if !props.simulation_paused
            && !props.simulation_reset
            && ((*trajectory_segments).is_empty()
            || props.rendered_bodies.iter().any(|rendered_body|
        (rendered_body.body.position - (*trajectory_segments).iter().last().unwrap().positions[rendered_body.index]).norm() > TRAJECTORY_MAX_SEGMENT_LENGTH)) {
            let mut trajectory_segments_new: VecDeque<TrajectorySegment> = (*trajectory_segments).clone();
            trajectory_segments_new.push_back(TrajectorySegment::new(props.rendered_bodies.iter().map(|rendered_body| rendered_body.body.position).collect()));

            while let Some(last) = trajectory_segments_new.front() {
                if last.recorded_at.elapsed() > settings.trajectory_duration {
                    trajectory_segments_new.pop_front();
                } else {
                    break;
                }
            }

            context.clear().unwrap();

            // reversing for a more intuitive layer order
            for (body_index, rendered_body) in props.rendered_bodies.iter().enumerate().rev() {
                let starting_position = trajectory_segments_new.front().unwrap().positions[body_index];
                context.set_stroke_style(&rendered_body.color.as_str().into());
                context.begin_path();
                context.move_to(starting_position.x, -starting_position.y);
                for position in trajectory_segments_new.iter().map(|trajectory_segment| trajectory_segment.positions[body_index]).skip(1) {
                    context.line_to(position.x, -position.y);
                }
                context.stroke();
            }

            trajectory_segments.set(trajectory_segments_new);
        }
    }

    html! {
        <canvas ref={canvas_ref} class="absolute"/>
    }
}
