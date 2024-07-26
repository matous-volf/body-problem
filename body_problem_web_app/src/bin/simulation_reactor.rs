use yew_agent::Registrable;

use body_problem_web_app::agents::SimulationReactor;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    
    SimulationReactor::registrar().register();
}
