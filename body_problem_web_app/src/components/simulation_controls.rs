use std::time::Duration;

use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use web_sys::wasm_bindgen::JsCast;
use yew::{Callback, function_component, Html, html, Properties, use_context};

use crate::components::button::Button;
use crate::components::validated_input::ValidatedInput;
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
                    <ValidatedInput id="input_trajectory_duration_text" class="w-24 mr-2"
                        value={settings.trajectory_duration.as_millis().to_string()}
                        on_input={props.set_settings_callback.reform(move |value: String| {
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
                <ValidatedInput id="input_simulation_speed_text" class="w-24"
                    value={settings.simulation_speed.to_string()}
                    on_input={props.set_settings_callback.reform(move |value: String| {
                        Settings {
                            simulation_speed: value.parse::<f64>().ok().filter(|&value| value >= 0f64).unwrap_or(settings.simulation_speed),
                            ..settings
                        }
                    })}/>
            </div>
            <div class="flex flex-row gap-3 items-center">
                <label for="input_body_circle_radius_range">{"circle radius"}</label>
                <input id="input_body_circle_radius_range" type="range" class="accent-white" min=0 max=30 step=0.1 value={settings.body_circle_radius.to_string()} oninput={props.set_settings_callback.reform(move |e: InputEvent| {
                        Settings {
                            body_circle_radius: e.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse::<f64>().unwrap_or(settings.body_circle_radius),
                            ..settings
                        }
                    })}/>
                <ValidatedInput id="input_body_circle_radius_text" class="w-24"
                    value={settings.body_circle_radius.to_string()}
                    on_input={props.set_settings_callback.reform(move |value: String| {
                        Settings {
                            body_circle_radius: if value.is_empty() { 0f64 } else {value.parse::<f64>().ok().filter(|&value| value >= 0f64).unwrap_or(settings.body_circle_radius)},
                            ..settings
                        }
                    })}/>
            </div>
            <div class="flex flex-row gap-3 items-center">
                <label for="input_scale_body_circles_with_mass">{"scale circles with mass"}</label>
                <input id="input_scale_body_circles_with_mass" type="checkbox" class="accent-white" checked={settings.scale_body_circles_with_mass} oninput={props.set_settings_callback.reform(move |e: InputEvent| {
                        Settings {
                            scale_body_circles_with_mass: e.target().unwrap().unchecked_into::<HtmlInputElement>().checked(),
                            ..settings
                        }
                    })}/>
            </div>
        </div>
    }
}
