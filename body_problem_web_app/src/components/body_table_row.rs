use yew::{Callback, function_component, Html, html, Properties};
use body_problem::Body;
use crate::models::RenderedBody;

#[derive(PartialEq, Properties)]
pub struct BodyTableRowProps {
    pub(crate) index: usize,
    pub(crate) rendered_body: RenderedBody,
    pub(crate) edit_callback: Callback<Body>,
}

#[function_component]
pub fn BodyTableRow(props: &BodyTableRowProps) -> Html {
    let index = props.index;
    let rendered_body = &props.rendered_body;

    html! {
        <tr class="font-mono text-lg divide-x divide-neutral-600">
            <td class="py-2 px-4 text-center">
                {index + 1}
            </td>
            <td class="py-2 px-4 text-center">{"white"}</td>
            <td class="py-2 px-4 text-right">
                {rendered_body.body.mass}
            </td>
            <td class="py-2 px-4 text-right">
                {format!("{:\u{00a0}>10.1}", rendered_body.body.position.x)}
            </td>
            <td class="py-2 px-4 text-right">{format!("{:\u{00a0}>10.1}", rendered_body.body.position.y)}</td>
            <td class="py-2 px-4 text-right">{format!("{:\u{00a0}>10.1}", rendered_body.body.velocity.x)}</td>
            <td class="py-2 px-4 text-right">{format!("{:\u{00a0}>10.1}", rendered_body.body.velocity.y)}</td>
            <td class="py-2 px-4 text-right">{format!("{:\u{00a0}>25.1}", rendered_body.potential_energy)}</td>
            <td class="py-2 px-4 text-right">{format!("{:\u{00a0}>25.1}", rendered_body.body.kinetic_energy())}</td>
        </tr>
    }
}
