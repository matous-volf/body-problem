use futures::{FutureExt, SinkExt, StreamExt};
use futures::stream::FusedStream;
use web_time::{Duration, Instant};
use yew_agent::prelude::{reactor, ReactorScope};

use body_problem::{Body, simulate};
use serde::{Deserialize, Serialize};

const TARGET_FPS: f64 = 50f64;
const STEP: f64 = 0.0001;

#[derive(Clone, Serialize, Deserialize)]
pub struct SimulationReactorInstruction {
    bodies: Option<Vec<Body>>,
    time_to_reality_ratio: f64,
}

impl SimulationReactorInstruction {
    pub fn new(bodies: Option<Vec<Body>>, time_to_reality_ratio: f64) -> Self {
        Self { bodies, time_to_reality_ratio }
    }
}

#[reactor(SimulationReactor)]
pub async fn simulation_reactor(
    /*
    - None => Stop the simulation.
    - Some => Set the bodies (if Some) and the speed (always).
    */
    mut scope: ReactorScope<Option<SimulationReactorInstruction>, Option<Vec<Body>>>,
) {
    let mut bodies = None;
    let mut speed = 1f64;

    loop {
        if scope.is_terminated() {
            return;
        }

        if let Some(instruction) = scope.next().await {
            if let Some(instruction) = instruction {
                bodies = instruction.bodies;
                speed = instruction.time_to_reality_ratio;
            }
            break;
        }
    }

    let mut none_sent = false;
    let mut taken_duration = Duration::ZERO;

    loop {
        // For some reason, it is required to always sleep for some time on order for the agent to work.
        async_std::task::sleep(Duration::from_millis((1000f64 / TARGET_FPS) as u64).checked_sub(taken_duration).unwrap_or(Duration::from_nanos(1))).await;

        let start = Instant::now();

        if let Some(Some(instruction)) = futures::future::poll_fn(|cx| scope.poll_next_unpin(cx)).now_or_never() {
            match instruction {
                Some(instruction) => {
                    if instruction.bodies.is_some() {
                        bodies = instruction.bodies;
                    }
                    speed = instruction.time_to_reality_ratio;
                }
                None => {
                    bodies = None
                }
            }
        }

        if bodies.is_none() {
            if !none_sent && scope.send(None).await.is_err() {
                break;
            }

            none_sent = true;
            continue;
        }
        none_sent = false;

        if scope.send(bodies.clone()).await.is_err() {
            break;
        }

        for _ in 0..(((1f64 / TARGET_FPS) / STEP) * speed) as usize {
            bodies = Some(simulate(bodies.as_ref().unwrap(), STEP));
        }

        taken_duration = start.elapsed();
    }
}
