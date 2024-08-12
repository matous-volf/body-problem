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
    state: Option<SimulationState>,
    time_to_reality_ratio: f64,
}

impl SimulationReactorInstruction {
    pub fn new(state: Option<SimulationState>, time_to_reality_ratio: f64) -> Self {
        Self { state, time_to_reality_ratio }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SimulationState {
    pub(crate) bodies: Vec<Body>,
    pub(crate) duration_elapsed_total: Duration,
}

impl SimulationState {
    pub fn new(bodies: Vec<Body>, duration_elapsed_total: Duration) -> Self {
        Self { bodies, duration_elapsed_total }
    }
}

#[reactor(SimulationReactor)]
pub async fn simulation_reactor(
    /*
    - None => Stop the simulation.
    - Some => Set the bodies (if Some) and the speed (always).
    */
    mut scope: ReactorScope<Option<SimulationReactorInstruction>, Option<SimulationState>>
) {
    let mut state = None;
    let mut steps_per_result = ((1f64 / TARGET_FPS) / STEP) as u64;

    loop {
        if scope.is_terminated() {
            return;
        }

        if let Some(instruction) = scope.next().await {
            if let Some(instruction) = instruction {
                state = instruction.state;
                steps_per_result = (((1f64 / TARGET_FPS) / STEP) * instruction.time_to_reality_ratio) as u64;
            }
            break;
        }
    }

    let mut none_sent = false;
    let mut taken_duration = Duration::ZERO;

    loop {
        // For some reason, it is required to always sleep for some time in order for the agent to work.
        async_std::task::sleep(Duration::from_millis((1000f64 / TARGET_FPS) as u64).checked_sub(taken_duration).unwrap_or(Duration::from_nanos(1))).await;

        let start = Instant::now();

        if let Some(Some(instruction)) = futures::future::poll_fn(|cx| scope.poll_next_unpin(cx)).now_or_never() {
            match instruction {
                Some(instruction) => {
                    if instruction.state.is_some() {
                        state = instruction.state;
                    }
                    steps_per_result = (((1f64 / TARGET_FPS) / STEP) * instruction.time_to_reality_ratio) as u64;
                }
                None => {
                    state = None
                }
            }
        }

        if state.is_none() {
            if !none_sent && scope.send(None).await.is_err() {
                break;
            }

            none_sent = true;
            continue;
        }
        none_sent = false;

        if scope.send(state.clone()).await.is_err() {
            break;
        }

        let state = state.as_mut().unwrap();
        for _ in 0..steps_per_result {
            state.bodies = simulate(state.bodies.as_ref(), STEP);
        }
        state.duration_elapsed_total += Duration::from_secs_f64(STEP * steps_per_result as f64);

        taken_duration = start.elapsed();
    }
}
