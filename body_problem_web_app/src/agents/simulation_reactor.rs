use std::time::Duration;

use futures::{FutureExt, SinkExt, StreamExt};
use futures::stream::FusedStream;
use yew_agent::prelude::{reactor, ReactorScope};

use body_problem::{Body, simulate};

#[reactor(SimulationReactor)]
pub async fn simulation_reactor(
    mut scope: ReactorScope<Option<Vec<Body>>, Option<Vec<Body>>>,
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

    let mut none_sent = false;

    loop {
        if let Some(Some(bodies_new)) = futures::future::poll_fn(|cx| scope.poll_next_unpin(cx)).now_or_never() {
            bodies = bodies_new;
        }

        if bodies.is_none() {
            if !none_sent && scope.send(None).await.is_err() {
                break;
            }

            none_sent = true;
            async_std::task::sleep(Duration::from_millis(20)).await;
            continue;
        }
        none_sent = false;

        if scope.send(bodies.clone()).await.is_err() {
            break;
        }

        for _ in 0..200 {
            bodies = Some(simulate(bodies.as_ref().unwrap(), 0.0001));
        }

        async_std::task::sleep(Duration::from_millis(20)).await;
    }
}
