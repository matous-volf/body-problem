use std::time::Duration;

use futures::{sink::SinkExt, stream::StreamExt};
use futures::stream::FusedStream;
use gloo_console::log;
use yew_agent::reactor::{reactor, ReactorScope};

use body_problem::{Body, simulate};

#[reactor(SimulationReactor)]
pub async fn simulation_reactor(
    mut scope: ReactorScope<Vec<Body>, Vec<Body>>,
) {
    let mut bodies;

    loop {
        if scope.is_terminated() {
            return;
        }

        if let Some(bodies_new) = scope.next().await {
            bodies = bodies_new;
            break;
        }
    }

    loop {
        if scope.send(bodies.clone()).await.is_err() {
            log!("err");
            break;
        }

        for _ in 0..10 {
            bodies = simulate(&bodies, 0.001);
        }

        async_std::task::sleep(Duration::from_millis(10)).await;
    }
}

