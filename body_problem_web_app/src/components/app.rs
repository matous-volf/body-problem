use yew::prelude::*;
use yew_agent::reactor::ReactorProvider;
use crate::agents::simulation_reactor::SimulationReactor;
use crate::components::footer::Footer;
use crate::components::simulation_panel::SimulationPanel;
use crate::hooks::use_google_analytics::use_google_analytics;

#[function_component(App)]
pub fn app() -> Html {
    use_google_analytics();
    
    html! {
        <ReactorProvider<SimulationReactor> path="/simulation_reactor.js">
        <main>
            <SimulationPanel/>
            <Footer/>
        </main>
        </ReactorProvider<SimulationReactor>>
    }
}
