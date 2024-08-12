use std::time::Duration;

use nalgebra::Vector2;
use web_sys::MouseEvent;
use yew::{Callback, ContextProvider, function_component, Html, html, use_effect_with, use_state};
use yew_agent::prelude::{use_reactor_subscription, UseReactorSubscriptionHandle};
use yew_hooks::use_window_size;

use body_problem::Body;

use crate::agents::simulation_reactor::{SimulationReactor, SimulationReactorInstruction, SimulationState};
use crate::components::body_canvas::BodyCanvas;
use crate::components::body_table::BodyTable;
use crate::components::energy_sum_table::EnergySumTable;
use crate::components::simulation_controls::SimulationControls;
use crate::components::trajectory_canvas::TrajectoryCanvas;
use crate::models::rendered_body::RenderedBody;
use crate::models::settings::Settings;

const SETTINGS_DEFAULT: Settings = Settings::new(Duration::from_secs(5), 1f64, 0.0001f64, true);

#[derive(Clone)]
pub(crate) struct RenderedSimulationState {
    pub(crate) rendered_bodies: Vec<RenderedBody>,
    pub(crate) duration_elapsed_total: Duration,
}

impl RenderedSimulationState {
    pub fn new(rendered_bodies: Vec<RenderedBody>, duration_elapsed_total: Duration) -> Self {
        Self { rendered_bodies, duration_elapsed_total }
    }
}

impl PartialEq for RenderedSimulationState {
    fn eq(&self, other: &Self) -> bool {
        self.rendered_bodies.iter().eq(other.rendered_bodies.iter())
            && self.duration_elapsed_total == other.duration_elapsed_total
    }
}

impl From<RenderedSimulationState> for SimulationState {
    fn from(rendered_simulation_state: RenderedSimulationState) -> Self {
        Self::new(rendered_simulation_state.rendered_bodies.iter().map(|b| b.body.clone()).collect(), rendered_simulation_state.duration_elapsed_total)
    }
}

impl PartialEq<RenderedSimulationState> for SimulationState {
    fn eq(&self, other: &RenderedSimulationState) -> bool {
        self.bodies.iter().eq(other.rendered_bodies.iter().map(|b| &b.body))
    }
}

#[function_component(SimulationPanel)]
pub fn simulation_panel() -> Html {
    let rendered_state = use_state(|| RenderedSimulationState::new(vec![
        RenderedBody::new(0, Body::new(1e17, Vector2::new(0f64, 0f64), Vector2::new(0f64, -1.52f64)), "#ffff3f".to_string()),
        RenderedBody::new(1, Body::new(1e15, Vector2::new(300f64, 0f64), Vector2::new(0f64, 149.76f64)), "#5a8cc8".to_string()),
        RenderedBody::new(2, Body::new(1e13, Vector2::new(320f64, 0f64), Vector2::new(0f64, 206.92f64)), "#bfbfbf".to_string()),
    ], Duration::ZERO));
    let rendered_state_after_last_edit = use_state(|| (*rendered_state).clone());
    let rendered_state_edited_this_pause = use_state(|| false);
    let simulation_paused = use_state(|| false);
    let simulation_reset = use_state(|| false);
    let settings = use_state(|| SETTINGS_DEFAULT);

    let simulation_agent: UseReactorSubscriptionHandle<SimulationReactor> = use_reactor_subscription::<SimulationReactor>();

    {
        let simulation_agent = simulation_agent.clone();
        let rendered_state = rendered_state.clone();
        let settings = settings.clone();
        use_effect_with((), move |_| {
            simulation_agent.send(Some(SimulationReactorInstruction::new(
                Some((*rendered_state).clone().into()),
                settings.simulation_speed,
            )))
        });
    }

    let rendered_state_new = if *simulation_paused {
        (*rendered_state).clone()
    } else {
        let state_new = simulation_agent.last()
            .map(|state| state.as_ref().as_ref()).unwrap_or_default();

        // The simulation was reset, and we are waiting for the agent to send the reset bodies.
        if *simulation_reset {
            if let Some(state_new) = state_new {
                if state_new == &*rendered_state_after_last_edit {
                    simulation_reset.set(false);
                }
            }

            (*rendered_state_after_last_edit).clone()
        } else {
            match state_new {
                None => (*rendered_state).clone(),
                Some(state_new) => RenderedSimulationState::new(
                    state_new.bodies.iter()
                        .enumerate()
                        .map(|(index, body)| {
                            RenderedBody {
                                index,
                                body: body.clone(),
                                potential_energy: state_new.bodies.iter()
                                    .enumerate()
                                    .filter(|(index2, _)| index != *index2)
                                    .map(|(_, body2)| body.potential_energy_to(body2))
                                    .sum(),
                                color: rendered_state.rendered_bodies[index].color.clone(),
                            }
                        }).collect(), state_new.duration_elapsed_total)
            }
        }
    };

    let toggle_pause_callback = {
        let rendered_state = rendered_state.clone();
        let rendered_state_after_last_edit = rendered_state_after_last_edit.clone();
        let rendered_state_edited_this_pause = rendered_state_edited_this_pause.clone();
        let simulation_paused = simulation_paused.clone();
        let rendered_state_new = rendered_state_new.clone();
        let simulation_agent = simulation_agent.clone();
        let settings = settings.clone();

        Callback::from(
            move |_| {
                let simulation_paused_new = !*simulation_paused;
                simulation_paused.set(simulation_paused_new);

                if simulation_paused_new {
                    rendered_state.set(rendered_state_new.clone());
                    simulation_agent.send(None);
                } else {
                    if *rendered_state_edited_this_pause {
                        rendered_state_after_last_edit.set((*rendered_state).clone());
                        rendered_state_edited_this_pause.set(false);
                    }
                    simulation_agent.send(Some(SimulationReactorInstruction::new(
                        Some(rendered_state_new.clone().into()),
                        settings.simulation_speed))
                    );
                }
            }
        )
    };

    let reset_callback = {
        let rendered_state = rendered_state.clone();
        let rendered_state_after_last_edit = rendered_state_after_last_edit.clone();
        let simulation_paused = simulation_paused.clone();
        let simulation_agent = simulation_agent.clone();
        let simulation_reset = simulation_reset.clone();
        let settings = settings.clone();

        Callback::from(
            move |_| {
                simulation_reset.set(true);
                if *simulation_paused {
                    rendered_state.set((*rendered_state_after_last_edit).clone());
                } else {
                    simulation_agent.send(Some(SimulationReactorInstruction::new(
                        Some((*rendered_state_after_last_edit).clone().into()),
                        settings.simulation_speed)));
                }
            }
        )
    };

    let body_add_callback = {
        let rendered_state = rendered_state.clone();
        let rendered_state_edited_this_pause = rendered_state_edited_this_pause.clone();
        let simulation_paused = simulation_paused.clone();
        let toggle_pause_callback = toggle_pause_callback.clone();

        Callback::from(
            move |_| {
                if !*simulation_paused {
                    toggle_pause_callback.emit(MouseEvent::new("").unwrap());
                }

                let mut rendered_state_new = (*rendered_state).clone();
                rendered_state_new.rendered_bodies.push(RenderedBody {
                    index: rendered_state_new.rendered_bodies.len(),
                    body: Body::new(1f64, Vector2::new(0f64, 0f64), Vector2::new(0f64, 0f64)),
                    potential_energy: 0f64,
                    color: "#ffffff".to_string(),
                });
                rendered_state_new.duration_elapsed_total = Duration::ZERO;
                
                rendered_state.set(rendered_state_new);
                rendered_state_edited_this_pause.set(true);
            }
        )
    };

    let body_edit_callback = {
        let rendered_state = rendered_state.clone();
        let rendered_state_edited_this_pause = rendered_state_edited_this_pause.clone();

        Callback::from(
            move |rendered_body: RenderedBody| {
                let mut rendered_state_new = (*rendered_state).clone();
                let index = rendered_body.index;
                /* important for preserving the `rendered_bodies_last_edit` when the user just
                   clicks into an input or edits a body to the same value as before */
                if rendered_state_new.rendered_bodies[index] == rendered_body {
                    return;
                }

                rendered_state_new.rendered_bodies[index] = rendered_body;
                rendered_state_new.duration_elapsed_total = Duration::ZERO;
                
                rendered_state.set(rendered_state_new);
                rendered_state_edited_this_pause.set(true);
            }
        )
    };

    let body_remove_callback = {
        let rendered_state = rendered_state.clone();
        let rendered_state_edited_this_pause = rendered_state_edited_this_pause.clone();

        Callback::from(
            move |index: usize| {
                let mut rendered_state_new = (*rendered_state).clone();
                rendered_state_new.rendered_bodies.remove(index);
                for (index, rendered_body) in rendered_state_new.rendered_bodies.iter_mut()
                    .enumerate() {
                    rendered_body.index = index;
                }
                rendered_state_new.duration_elapsed_total = Duration::ZERO;

                rendered_state.set(rendered_state_new);
                rendered_state_edited_this_pause.set(true);
            }
        )
    };

    let set_settings_callback = {
        let settings = settings.clone();

        Callback::from(
            move |settings_new: Settings| {
                if settings_new.simulation_speed != settings.simulation_speed {
                    simulation_agent.send(Some(SimulationReactorInstruction::new(
                        None,
                        settings_new.simulation_speed,
                    )));
                }

                settings.set(settings_new);
            }
        )
    };

    let window_size = use_window_size();
    html! {
        <ContextProvider<Settings> context={(*settings).clone()}>
            <div style={format!("height: {}px", (window_size.1 - 150f64).max(0f64))}>
                <TrajectoryCanvas rendered_state={rendered_state_new.clone()}
                    rendered_bodies_edited_this_pause={*rendered_state_edited_this_pause}
                    simulation_paused={*simulation_paused} simulation_reset={*simulation_reset}/>
                <BodyCanvas rendered_bodies={rendered_state_new.rendered_bodies.clone()}/>
            </div>
            <section class="p-4 flex flex-col gap-8">
                <SimulationControls simulation_paused={*simulation_paused} {toggle_pause_callback}
                    {reset_callback} {set_settings_callback}
                    duration_elapsed_total={rendered_state_new.duration_elapsed_total}/>
                <BodyTable rendered_bodies={rendered_state_new.rendered_bodies.clone()}
                    edit_allowed={*simulation_paused} add_callback={body_add_callback}
                    edit_callback={body_edit_callback} remove_callback={body_remove_callback}/>
                <EnergySumTable rendered_bodies={rendered_state_new.rendered_bodies}/>
            </section>
        </ContextProvider<Settings>>
    }
}
