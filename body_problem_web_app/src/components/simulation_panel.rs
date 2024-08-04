use std::time::Duration;

use nalgebra::Vector2;
use web_sys::MouseEvent;
use yew::{Callback, ContextProvider, function_component, Html, html, use_effect_with, use_state};
use yew_agent::prelude::{use_reactor_subscription, UseReactorSubscriptionHandle};
use yew_hooks::use_window_size;

use body_problem::Body;

use crate::agents::simulation_reactor::{SimulationReactor, SimulationReactorInstruction};
use crate::components::body_canvas::BodyCanvas;
use crate::components::body_table::BodyTable;
use crate::components::simulation_controls::SimulationControls;
use crate::components::trajectory_canvas::TrajectoryCanvas;
use crate::models::rendered_body::RenderedBody;
use crate::models::settings::Settings;

const SETTINGS_DEFAULT: Settings = Settings::new(Duration::from_secs(5), 1f64, 0.0001f64, true);

#[function_component(SimulationPanel)]
pub fn simulation_panel() -> Html {
    let rendered_bodies = use_state(|| vec![
        RenderedBody::new(0, Body::new(1e17, Vector2::new(0f64, 0f64), Vector2::new(0f64, -1.52f64)), "#ffff3f".to_string()),
        RenderedBody::new(1, Body::new(1e15, Vector2::new(300f64, 0f64), Vector2::new(0f64, 149.76f64)), "#5a8cc8".to_string()),
        RenderedBody::new(2, Body::new(1e13, Vector2::new(320f64, 0f64), Vector2::new(0f64, 206.92f64)), "#bfbfbf".to_string()),
    ]);
    let rendered_bodies_after_last_edit = use_state(|| (*rendered_bodies).clone());
    let rendered_bodies_edited_this_pause = use_state(|| false);
    let simulation_paused = use_state(|| false);
    let simulation_reset = use_state(|| false);
    let settings = use_state(|| SETTINGS_DEFAULT);

    let simulation_agent: UseReactorSubscriptionHandle<SimulationReactor> = use_reactor_subscription::<SimulationReactor>();

    {
        let simulation_agent = simulation_agent.clone();
        let bodies = rendered_bodies.clone();
        let settings = settings.clone();
        use_effect_with((), move |_| {
            simulation_agent.send(Some(SimulationReactorInstruction::new(Some((*bodies).iter().map(|b| b.body.clone()).collect()), settings.simulation_speed)));
        });
    }

    let rendered_bodies_new = if *simulation_paused {
        (*rendered_bodies).to_vec()
    } else {
        let bodies_new = simulation_agent.last().map(|bodies| bodies.as_ref().as_ref()).unwrap_or_default();

        // the simulation was reset, and we are waiting for the agent to send the reset bodies
        if *simulation_reset {
            if let Some(bodies_new) = bodies_new {
                if bodies_new.iter().eq((*rendered_bodies_after_last_edit).iter().map(|b| &b.body)) {
                    simulation_reset.set(false);
                }
            }

            (*rendered_bodies_after_last_edit).to_vec()
        } else {
            match bodies_new {
                None => (*rendered_bodies).to_vec(),
                Some(bodies_new) => bodies_new.iter().enumerate().map(|(index, body)| {
                    RenderedBody {
                        index,
                        body: body.clone(),
                        potential_energy: bodies_new.iter().enumerate().filter(|(index2, _)| index != *index2).map(|(_, body2)| body.potential_energy_to(body2)).sum(),
                        color: (*rendered_bodies)[index].color.clone(),
                    }
                }).collect()
            }
        }
    };

    let toggle_pause_callback = {
        let rendered_bodies = rendered_bodies.clone();
        let rendered_bodies_after_last_edit = rendered_bodies_after_last_edit.clone();
        let rendered_bodies_edited_this_pause = rendered_bodies_edited_this_pause.clone();
        let simulation_paused = simulation_paused.clone();
        let rendered_bodies_new = rendered_bodies_new.clone();
        let simulation_agent = simulation_agent.clone();
        let settings = settings.clone();

        Callback::from(
            move |_| {
                let simulation_paused_new = !*simulation_paused;
                simulation_paused.set(simulation_paused_new);

                if simulation_paused_new {
                    rendered_bodies.set(rendered_bodies_new.to_vec());
                    simulation_agent.send(None);
                } else {
                    if *rendered_bodies_edited_this_pause {
                        rendered_bodies_after_last_edit.set((*rendered_bodies).to_vec());
                        rendered_bodies_edited_this_pause.set(false);
                    }
                    simulation_agent.send(Some(SimulationReactorInstruction::new(Some(rendered_bodies_new.iter().map(|b| b.body.clone()).collect()), settings.simulation_speed)));
                }
            }
        )
    };

    let reset_callback = {
        let rendered_bodies = rendered_bodies.clone();
        let rendered_bodies_after_last_edit = rendered_bodies_after_last_edit.clone();
        let simulation_paused = simulation_paused.clone();
        let simulation_agent = simulation_agent.clone();
        let simulation_reset = simulation_reset.clone();
        let settings = settings.clone();

        Callback::from(
            move |_| {
                simulation_reset.set(true);
                if *simulation_paused {
                    rendered_bodies.set((*rendered_bodies_after_last_edit).to_vec());
                } else {
                    simulation_agent.send(Some(SimulationReactorInstruction::new(Some((*rendered_bodies_after_last_edit).iter().map(|b| b.body.clone()).collect()), settings.simulation_speed)));
                }
            }
        )
    };

    let body_add_callback = {
        let rendered_bodies = rendered_bodies.clone();
        let rendered_bodies_edited_this_pause = rendered_bodies_edited_this_pause.clone();
        let simulation_paused = simulation_paused.clone();
        let toggle_pause_callback = toggle_pause_callback.clone();
        let rendered_bodies_new = rendered_bodies_new.clone();

        Callback::from(
            move |_| {
                if !*simulation_paused {
                    toggle_pause_callback.emit(MouseEvent::new("").unwrap());
                }

                let mut rendered_bodies_new: Vec<RenderedBody> = rendered_bodies_new.to_vec();
                rendered_bodies_edited_this_pause.set(true);

                rendered_bodies_new.push(RenderedBody {
                    index: rendered_bodies_new.len(),
                    body: Body::new(1f64, Vector2::new(0f64, 0f64), Vector2::new(0f64, 0f64)),
                    potential_energy: 0f64,
                    color: "#ffffff".to_string(),
                });
                rendered_bodies.set(rendered_bodies_new);
            }
        )
    };

    let body_edit_callback = {
        let rendered_bodies = rendered_bodies.clone();
        let rendered_bodies_edited_this_pause = rendered_bodies_edited_this_pause.clone();

        Callback::from(
            move |rendered_body: RenderedBody| {
                let mut rendered_bodies_new = (*rendered_bodies).to_vec();
                let index = rendered_body.index;
                /* important for preserving the `rendered_bodies_last_edit` when the user just
                   clicks into an input or edits a body to the same value as before */
                if rendered_bodies_new[index] == rendered_body {
                    return;
                }

                rendered_bodies_new[index] = rendered_body;
                rendered_bodies.set(rendered_bodies_new);
                rendered_bodies_edited_this_pause.set(true);
            }
        )
    };

    let body_remove_callback = {
        let rendered_bodies = rendered_bodies.clone();
        let rendered_bodies_edited_this_pause = rendered_bodies_edited_this_pause.clone();

        Callback::from(
            move |index: usize| {
                let mut rendered_bodies_new: Vec<RenderedBody> = (*rendered_bodies).to_vec();
                rendered_bodies_new.remove(index);
                for (index, rendered_body) in rendered_bodies_new.iter_mut().enumerate() {
                    rendered_body.index = index;
                }

                rendered_bodies.set(rendered_bodies_new);
                rendered_bodies_edited_this_pause.set(true);
            }
        )
    };

    let set_settings_callback = {
        let settings = settings.clone();

        Callback::from(
            move |settings_new: Settings| {
                if settings_new.simulation_speed != settings.simulation_speed {
                    simulation_agent.send(Some(SimulationReactorInstruction::new(None, settings_new.simulation_speed)));
                }

                settings.set(settings_new);
            }
        )
    };

    let window_size = use_window_size();
    html! {
        <ContextProvider<Settings> context={(*settings).clone()}>
            <div style={format!("height: {}px", (window_size.1 - 150f64).max(0f64))}>
                <TrajectoryCanvas rendered_bodies={rendered_bodies_new.clone()} rendered_bodies_edited_this_pause={*rendered_bodies_edited_this_pause} simulation_paused={*simulation_paused} simulation_reset={*simulation_reset}/>
                <BodyCanvas rendered_bodies={rendered_bodies_new.clone()}/>
            </div>
            <section class="p-4 flex flex-col gap-8">
                <SimulationControls simulation_paused={*simulation_paused} {toggle_pause_callback} {reset_callback} {set_settings_callback}/>
                <BodyTable rendered_bodies={rendered_bodies_new} edit_allowed={*simulation_paused} add_callback={body_add_callback} edit_callback={body_edit_callback} remove_callback={body_remove_callback}/>
            </section>
        </ContextProvider<Settings>>
    }
}
