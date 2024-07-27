use yew::{Callback, function_component, Html, html, Properties};

use crate::components::BodyTableRow;
use crate::models::RenderedBody;

#[derive(Properties, PartialEq)]
pub struct BodyTableProps {
    pub(crate) rendered_bodies: Vec<RenderedBody>,
    pub(crate) edit_allowed: bool,
    pub(crate) edit_callback: Callback<RenderedBody>,
}

#[function_component(BodyTable)]
pub fn body_table(props: &BodyTableProps) -> Html {
    html! {
        <div class="overflow-x-auto">
            <table class="table-auto divide-y divide-neutral-600 min-w-full">
                <thead class="text-neutral-500 whitespace-nowrap">
                    <tr class="divide-x divide-neutral-600">
                        <th class="py-2 px-4">{"#"}</th>
                        <th class="py-2 px-4">{"color"}</th>
                        <th class="py-2 px-4">{"mass [kg]"}</th>
                        <th class="py-2 px-4" colspan=2>{"position [px]"}</th>
                        <th class="py-2 px-4" colspan=2>{"velocity [px/s]"}</th>
                        <th class="py-2 px-4">{"potential energy [J]"}</th>
                        <th class="py-2 px-4">{"kinetic energy [J]"}</th>
                    </tr>
                </thead>

                <tbody class="divide-y divide-white/25">
                    {props.rendered_bodies.iter().enumerate().map(|(index, rendered_body)| {
                        let edit_callback_clone = props.edit_callback.clone();
                        html! {
                            <BodyTableRow {index} rendered_body={rendered_body.clone()} edit_allowed={props.edit_allowed} edit_callback={edit_callback_clone}/>
                        }
                    }).collect::<Html>()}
                </tbody>
            </table>
        </div>
    }
}
