use yew::prelude::*;
use yew_agent::reactor::ReactorProvider;

use crate::agents::SimulationReactor;
use crate::components::SimulationPanel;

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
