use web_sys::MouseEvent;
use yew::{Callback, function_component, Html, html, Properties};
use crate::components::button::Button;

#[derive(PartialEq, Properties)]
pub struct SimulationControlsProps {
    pub(crate) simulation_paused: bool,
    pub(crate) toggle_pause_callback: Callback<MouseEvent>,
}

#[function_component(SimulationControls)]
pub fn simulation_controls(props: &SimulationControlsProps) -> Html {
    html! {
        <>
            <Button onclick={props.toggle_pause_callback.clone()}>
                {if props.simulation_paused { "play" } else { "pause" }}
            </button>
        </>
    }
}
