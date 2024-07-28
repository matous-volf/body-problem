use nalgebra::Vector2;
use yew::{Callback, function_component, Html, html, use_effect_with, use_state};
use yew_agent::prelude::{use_reactor_subscription, UseReactorSubscriptionHandle};

use body_problem::Body;

use crate::agents::SimulationReactor;
use crate::components::{BodyCanvas, BodyTable};
use crate::components::simulation_controls::SimulationControls;
use crate::models::RenderedBody;

#[function_component(SimulationPanel)]
pub fn simulation_panel() -> Html {
    let rendered_bodies = use_state(|| vec![
        RenderedBody {
            index: 0,
            body: Body::new(1e16, Vector2::new(0f64, 0f64), Vector2::new(0f64, 0f64)),
            potential_energy: 0f64,
            color: "#ffffff".to_string(),
        },
        RenderedBody {
            index: 1,
            body: Body::new(1e16, Vector2::new(100f64, -100f64), Vector2::new(0f64, 0f64)),
            potential_energy: 0f64,
            color: "#ffffff".to_string(),
        },
        RenderedBody {
            index: 2,
            body: Body::new(1e16, Vector2::new(-200f64, -100f64), Vector2::new(0f64, 0f64)),
            potential_energy: 0f64,
            color: "#ffffff".to_string(),
        },
    ]);
    let rendered_bodies_after_last_edit = use_state(|| (*rendered_bodies).clone());
    let rendered_bodies_edited_this_pause = use_state(|| false);
    let simulation_paused = use_state(|| false);

    let simulation_agent: UseReactorSubscriptionHandle<SimulationReactor> = use_reactor_subscription::<SimulationReactor>();

    {
        let simulation_agent = simulation_agent.clone();
        let bodies = rendered_bodies.clone();
        use_effect_with((), move |_| {
            simulation_agent.send(Some((*bodies).iter().map(|b| b.body.clone()).collect()));
        });
    }

    let rendered_bodies_new = if *simulation_paused {
        (*rendered_bodies).to_vec()
    } else {
        let bodies_new = simulation_agent.last().map(|bodies| bodies.as_ref().as_ref()).unwrap_or_default();

        bodies_new.map(|bodies| bodies.iter().enumerate().map(|(index, body)| {
            RenderedBody {
                index,
                body: body.clone(),
                potential_energy: bodies.iter().enumerate().filter(|(index2, _)| index != *index2).map(|(_, body2)| body.potential_energy_to(body2)).sum(),
                color: (*rendered_bodies)[index].color.clone(),
            }
        }).collect()).unwrap_or_else(|| (*rendered_bodies).to_vec())
    };

    let toggle_pause_callback = {
        let rendered_bodies = rendered_bodies.clone();
        let rendered_bodies_after_last_edit = rendered_bodies_after_last_edit.clone();
        let rendered_bodies_edited_this_pause = rendered_bodies_edited_this_pause.clone();
        let simulation_paused = simulation_paused.clone();
        let rendered_bodies_new = rendered_bodies_new.clone();
        let simulation_agent = simulation_agent.clone();

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
                    simulation_agent.send(Some(rendered_bodies_new.iter().map(|b| b.body.clone()).collect()));
                }
            }
        )
    };

    let reset_callback = {
        let rendered_bodies = rendered_bodies.clone();
        let rendered_bodies_after_last_edit = rendered_bodies_after_last_edit.clone();
        let simulation_paused = simulation_paused.clone();
        let simulation_agent = simulation_agent.clone();

        Callback::from(
            move |_| {
                if *simulation_paused {
                    rendered_bodies.set((*rendered_bodies_after_last_edit).to_vec());
                } else {
                    simulation_agent.send(Some((*rendered_bodies_after_last_edit).iter().map(|b| b.body.clone()).collect()));
                }
            }
        )
    };

    let body_edit_callback = {
        let rendered_bodies = rendered_bodies.clone();
        let rendered_bodies_edited_this_pause = rendered_bodies_edited_this_pause.clone();

        Callback::from(
            move |rendered_body: RenderedBody| {
                let index = rendered_body.index;
                let mut rendered_bodies_new = (*rendered_bodies).to_vec();

                /* important for preserving the `rendered_bodies_last_edit` when the user just
                   clicks into an input or edits a body to the same value as before */
                if rendered_bodies_new[index] == rendered_body {
                    return;
                }
                rendered_bodies_edited_this_pause.set(true);

                rendered_bodies_new[index] = rendered_body;
                rendered_bodies.set(rendered_bodies_new);
            }
        )
    };
    
    let body_remove_callback = {
        let rendered_bodies = rendered_bodies.clone();
        let rendered_bodies_edited_this_pause = rendered_bodies_edited_this_pause.clone();

        Callback::from(
            move |index: usize| {
                let mut rendered_bodies_new: Vec<RenderedBody> = (*rendered_bodies).to_vec();
                rendered_bodies_edited_this_pause.set(true);

                rendered_bodies_new.remove(index);
                for (index, rendered_body) in rendered_bodies_new.iter_mut().enumerate() {
                    rendered_body.index = index;
                }
                rendered_bodies.set(rendered_bodies_new);
            }
        )
    };

    html! {
        <>
            <BodyCanvas rendered_bodies={rendered_bodies_new.clone()}/>
            <section class="p-4 flex flex-col gap-4">
                <SimulationControls simulation_paused={*simulation_paused} {toggle_pause_callback} {reset_callback}/>
                <BodyTable rendered_bodies={rendered_bodies_new} edit_allowed={*simulation_paused} edit_callback={body_edit_callback} remove_callback={body_remove_callback}/>
            </section>
        </>
    }
}
