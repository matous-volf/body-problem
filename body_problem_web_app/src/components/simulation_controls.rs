use std::time::Duration;

use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use web_sys::wasm_bindgen::JsCast;
use yew::{Callback, function_component, Html, html, Properties, use_context};

use crate::components::button::Button;
use crate::models::settings::Settings;

#[derive(PartialEq, Properties)]
pub struct SimulationControlsProps {
    pub(crate) simulation_paused: bool,
    pub(crate) toggle_pause_callback: Callback<MouseEvent>,
    pub(crate) reset_callback: Callback<MouseEvent>,
    pub(crate) set_settings_callback: Callback<Settings>,
}

#[function_component(SimulationControls)]
pub fn simulation_controls(props: &SimulationControlsProps) -> Html {
    let settings = use_context::<Settings>().unwrap();

    html! {
        <div class="flex flex-row gap-10">
            <div class="flex flex-row gap-3">
                <Button onclick={props.toggle_pause_callback.clone()} class="w-24 py-2 px-4">
                    {if props.simulation_paused {
                        html! {
                            <><i class="fa-solid fa-play mr-2"></i>{"play"}</>
                        }
                    } else {
                        html! {
                            <><i class="fa-solid fa-pause mr-2"></i>{"pause"}</>
                        }
                    }}
                </Button>
                <Button onclick={props.reset_callback.clone()} class="w-24 py-2 px-4">
                    <i class="fa-solid fa-rotate-right mr-2"></i>{"reset"}
                </Button>
            </div>
            <div class="flex flex-row gap-3 items-center">
                <label for="input_trajectory_duration_range">{"trajectory duration"}</label>
                <input id="input_trajectory_duration_range" type="range" class="accent-white" min=0 max=60_000 step=500 value={settings.trajectory_duration.as_millis().to_string()} oninput={props.set_settings_callback.reform(move |e: InputEvent| {
                        Settings {
                            trajectory_duration: Duration::from_millis(e.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse::<u64>().unwrap_or(settings.trajectory_duration.as_millis() as u64)),
                            ..settings
                        }
                    })}/>
                <div>
                    <input id="input_trajectory_duration_text" type="text" class="bg-neutral-800 text-right font-mono text-lg w-24 py-1 px-3 border border-neutral-500 rounded mr-2" value={settings.trajectory_duration.as_millis().to_string()} oninput={props.set_settings_callback.reform(move |e: InputEvent| {
                            let mut value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
                            if value.is_empty() {
                                value = "0".to_string();
                            }
                            Settings {
                                trajectory_duration: Duration::from_millis(value.parse::<u64>().unwrap_or(settings.trajectory_duration.as_millis() as u64)),
                                ..settings
                            }
                        })}/>
                    <span>{"ms"}</span>
                </div>
            </div>
            <div class="flex flex-row gap-3 items-center">
                <label for="input_simulation_speed_range">{"simulation speed"}</label>
                <input id="input_simulation_speed_range" type="range" class="accent-white" min=0 max=10 step=0.1 value={settings.simulation_speed.to_string()} oninput={props.set_settings_callback.reform(move |e: InputEvent| {
                        Settings {
                            simulation_speed: e.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse::<f64>().unwrap_or(settings.simulation_speed),
                            ..settings
                        }
                    })}/>
                <input id="input_simulation_speed_text" type="text" class="bg-neutral-800 text-right font-mono text-lg w-16 py-1 px-3 border border-neutral-500 rounded mr-2" value={settings.simulation_speed.to_string()} oninput={props.set_settings_callback.reform(move |e: InputEvent| {
                        let mut value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
                        if value.is_empty() {
                            value = "0".to_string();
                        }
                        Settings {
                            simulation_speed: value.parse::<f64>().ok().filter(|&value| value >= 0f64).unwrap_or(settings.simulation_speed),
                            ..settings
                        }
                    })}/>
            </div>
        </div>
    }
}
