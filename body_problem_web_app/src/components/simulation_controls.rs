use web_sys::MouseEvent;
use yew::{Callback, function_component, Html, html, Properties};

#[derive(PartialEq, Properties)]
pub struct SimulationControlsProps {
    pub(crate) simulation_paused: bool,
    pub(crate) toggle_pause_callback: Callback<MouseEvent>,
}

#[function_component(SimulationControls)]
pub fn simulation_controls(props: &SimulationControlsProps) -> Html {
    html! {
        <>
            <button class="bg-white hover:bg-neutral-400 duration-150 text-neutral-800 font-bold py-2 px-4 rounded"
                onclick={props.toggle_pause_callback.clone()}>
                {if props.simulation_paused { "play" } else { "pause" }}
            </button>
        </>
    }
}
