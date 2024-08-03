use yew::{Callback, function_component, Html, html, Properties};

use crate::components::body_table_row::BodyTableRow;
use crate::components::button::Button;
use crate::models::rendered_body::RenderedBody;

#[derive(Properties, PartialEq)]
pub struct BodyTableProps {
    pub(crate) rendered_bodies: Vec<RenderedBody>,
    pub(crate) edit_allowed: bool,
    pub(crate) add_callback: Callback<()>,
    pub(crate) edit_callback: Callback<RenderedBody>,
    pub(crate) remove_callback: Callback<usize>,
}

#[function_component(BodyTable)]
pub fn body_table(props: &BodyTableProps) -> Html {
    let add_callback = props.add_callback.clone();

    html! {
        <div class="flex flex-col gap-2 items-start">
            <div class="overflow-x-auto max-w-full">
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
                            if props.edit_allowed { <th class="py-2 px-4">{"remove"}</th> }
                        </tr>
                    </thead>
    
                    <tbody class="divide-y divide-white/25">
                        {props.rendered_bodies.iter().map(|rendered_body| {
                            html! {
                                <BodyTableRow rendered_body={rendered_body.clone()} edit_allowed={props.edit_allowed} edit_callback={props.edit_callback.clone()} remove_callback={props.remove_callback.clone()}/>
                            }
                        }).collect::<Html>()}
                    </tbody>
                </table>
            </div>
            <Button onclick={Callback::from(move |_| add_callback.emit(()))} class="py-2 px-4">
                 <i class="fa-solid fa-plus mr-2"></i>{"add"}
            </Button>
        </div>
    }
}
