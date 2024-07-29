use yew::prelude::*;
use yew_agent::reactor::ReactorProvider;
use crate::agents::simulation_reactor::SimulationReactor;
use crate::components::simulation_panel::SimulationPanel;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ReactorProvider<SimulationReactor> path="/simulation_reactor.js">
        <main>
            <SimulationPanel/>
        </main>
        </ReactorProvider<SimulationReactor>>
    }
}
