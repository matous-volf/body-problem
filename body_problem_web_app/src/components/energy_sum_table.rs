use yew::{function_component, html, Html, Properties};

use crate::models::rendered_body::RenderedBody;

#[derive(Properties, PartialEq)]
pub struct EnergySumTableProps {
    pub(crate) rendered_bodies: Vec<RenderedBody>,
}

#[function_component(EnergySumTable)]
pub fn energy_sum_table(props: &EnergySumTableProps) -> Html {
    let potential_energy_sum = props.rendered_bodies.iter()
        .map(|rendered_body| rendered_body.potential_energy).sum::<f64>();
    let kinetic_energy_sum = props.rendered_bodies.iter()
        .map(|rendered_body| rendered_body.body.kinetic_energy()).sum::<f64>();

    html! {
        <div class="overflow-x-auto">
            <table class="table-auto divide-y divide-neutral-600">
                <thead class="text-neutral-500 whitespace-nowrap">
                    <tr class="divide-x divide-neutral-600">
                        <th class="py-2 px-4">{"energy type"}</th>
                        <th class="py-2 px-4">{"sum [J]"}</th>
                    </tr>
                </thead>

                <tbody class="divide-y divide-neutral-600">
                    <tr class="divide-x divide-neutral-600">
                        <td class="py-2 px-4">{"potential"}</td>
                        <td class="py-2 px-4 font-mono text-lg">
                            {format!("{:\u{00a0}>40.1}", potential_energy_sum)}
                        </td>
                    </tr>
                    <tr class="divide-x divide-neutral-600">
                        <td class="py-2 px-4">{"kinetic"}</td>
                        <td class="py-2 px-4 font-mono text-lg">
                            {format!("{:\u{00a0}>40.1}", kinetic_energy_sum)}
                        </td>
                    </tr>
                    <tr class="divide-x divide-neutral-600">
                        <td class="py-2 px-4">{"total"}</td>
                        <td class="py-2 px-4 font-mono text-lg">
                            {format!("{:\u{00a0}>40.1}", potential_energy_sum + kinetic_energy_sum)}
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
