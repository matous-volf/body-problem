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
        let simulation_paused = simulation_paused.clone();
        let rendered_bodies_new = rendered_bodies_new.clone();

        Callback::from(
            move |_| {
                let simulation_paused_new = !*simulation_paused;
                simulation_paused.set(simulation_paused_new);

                if simulation_paused_new {
                    rendered_bodies.set(rendered_bodies_new.to_vec());
                    simulation_agent.send(None);
                } else {
                    simulation_agent.send(Some(rendered_bodies_new.iter().map(|b| b.body.clone()).collect()));
                }
            }
        )
    };

    html! {
        <>
            <BodyCanvas rendered_bodies={rendered_bodies_new.clone()}/>
            <section class="p-4">
                <SimulationControls simulation_paused={*simulation_paused} {toggle_pause_callback}/>
            </section>
            <section class="p-4">
                <BodyTable rendered_bodies={rendered_bodies_new}/>
            </section>
        </>
    }
}
