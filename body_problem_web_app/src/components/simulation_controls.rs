use web_sys::MouseEvent;
use yew::{Callback, function_component, Html, html, Properties};
use crate::components::button::Button;

#[derive(PartialEq, Properties)]
pub struct SimulationControlsProps {
    pub(crate) simulation_paused: bool,
    pub(crate) toggle_pause_callback: Callback<MouseEvent>,
    pub(crate) reset_callback: Callback<MouseEvent>,
}

#[function_component(SimulationControls)]
pub fn simulation_controls(props: &SimulationControlsProps) -> Html {
    html! {
        <div class="flex flex-row gap-3">
            <Button onclick={props.toggle_pause_callback.clone()} class="w-24">
                {if props.simulation_paused {
                    html! {
                        <><i class="fa-solid fa-play mr-2"></i> {"play"}</>
                    }
                } else {
                    html! {
                        <><i class="fa-solid fa-pause mr-2"></i>{"pause"}</>
                    }
                }}
            </Button>
            <Button onclick={props.reset_callback.clone()}>
                <i class="fa-solid fa-rotate-right mr-2"></i> {"reset"}
            </Button>
        </div>
    }
}
