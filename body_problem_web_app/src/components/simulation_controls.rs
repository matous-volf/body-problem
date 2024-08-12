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
    pub(crate) duration_elapsed_total: Duration,
}

#[function_component(SimulationControls)]
pub fn simulation_controls(props: &SimulationControlsProps) -> Html {
    let settings = use_context::<Settings>().unwrap();

    html! {
        <>
            <div class="flex flex-col sm:items-center sm:flex-row gap-6">
                <div class="grid grid-cols-2 justify-stretch sm:grid-cols-none sm:flex sm:flex-row gap-3 shrink">
                    <Button onclick={props.toggle_pause_callback.clone()} class="py-2 px-4 sm:w-24">
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
                    <Button onclick={props.reset_callback.clone()} class="py-2 px-4 sm:w-24">
                        <i class="fa-solid fa-rotate-right mr-2"></i>{"reset"}
                    </Button>
                </div>
                <span>
                    {"duration elapsed: "}
                    <span class="font-mono text-lg">
                        {format!("{:10.2}", props.duration_elapsed_total.as_secs_f64())}
                    </span>
                    {" s"}
                </span>
            </div>
            <div class="grid justify-items-stretch grid-cols-1 sm:grid-cols-2 xl:flex xl:justify-between gap-6">
                <div class="flex flex-col grow">
                    <label for="input_trajectory_duration_range" class="whitespace-nowrap">{"trajectory duration [ms]"}</label>
                    <div class="flex flex-row gap-3">
                        <input id="input_trajectory_duration_range" type="range" class="accent-white grow" min=0 max=60_000 step=500 value={settings.trajectory_duration.as_millis().to_string()} oninput={props.set_settings_callback.reform(move |e: InputEvent| {
                            Settings {
                                trajectory_duration: Duration::from_millis(e.target().unwrap().unchecked_into::<HtmlInputElement>().value().parse::<u64>().unwrap_or(settings.trajectory_duration.as_millis() as u64)),
                                ..settings
                            }
                        })}/>
                        <div>
                            <ValidatedInput id="input_trajectory_duration_text" class="w-24"
                                value={settings.trajectory_duration.as_millis().to_string()}
                                on_input={props.set_settings_callback.reform(move |value: String| {
                                    Settings {
                                        trajectory_duration: Duration::from_millis(value.parse::<u64>().unwrap_or(settings.trajectory_duration.as_millis() as u64)),
                                        ..settings
                                    }
                                })}/>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col grow">
                    <label for="input_simulation_speed_range" class="whitespace-nowrap">{"simulation speed"}</label>
                    <div class="flex flex-row gap-3">
                        <input id="input_simulation_speed_range" type="range" class="accent-white grow" min=0 max=20 step=0.1 value={settings.simulation_speed.to_string()} oninput={props.set_settings_callback.reform(move |e: InputEvent| {
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
                </div>
                <div class="flex flex-col grow">
                    <label for="input_body_circle_radius_range" class="whitespace-nowrap">{"circle radius [px]"}</label>
                    <div class="flex flex-row gap-3">
                        <input id="input_body_circle_radius_range" type="range" class="accent-white grow" min=0 max=30 step=0.1 value={settings.body_circle_radius.to_string()} oninput={props.set_settings_callback.reform(move |e: InputEvent| {
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
                </div>
                <div class="flex flex-row gap-3 items-center">
                    <label for="input_scale_body_circles_with_mass" class="whitespace-nowrap">{"scale circles with mass"}</label>
                    <input id="input_scale_body_circles_with_mass" type="checkbox" class="accent-white" checked={settings.scale_body_circles_with_mass} oninput={props.set_settings_callback.reform(move |e: InputEvent| {
                            Settings {
                                scale_body_circles_with_mass: e.target().unwrap().unchecked_into::<HtmlInputElement>().checked(),
                                ..settings
                            }
                        })}/>
                </div>
            </div>
        </>
    }
}
