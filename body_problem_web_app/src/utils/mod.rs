use gloo_events::EventListener;
use gloo_utils::format::JsValueSerdeExt;
use web_sys::{CanvasRenderingContext2d, Event, HtmlCanvasElement, window};
use web_sys::wasm_bindgen::{JsCast, JsValue};
use yew::{Callback, UseStateHandle};

pub(crate) trait CanvasClear {
    fn clear(&self) -> Result<(), ()>;
}

impl CanvasClear for CanvasRenderingContext2d {
    fn clear(&self) -> Result<(), ()> {
        let canvas = self.canvas().ok_or(())?;
        self.clear_rect(-((canvas.width() / 2) as f64), -((canvas.height() / 2) as f64), canvas.width() as f64, canvas.height() as f64);
        Ok(())
    }
}

pub(crate) trait SimulationCanvasInitialize {
    fn initialize_for_simulation(self, context_state: UseStateHandle<Option<CanvasRenderingContext2d>>, enable_alpha: bool) -> EventListener;
}

impl SimulationCanvasInitialize for HtmlCanvasElement {
    fn initialize_for_simulation(self, context_state: UseStateHandle<Option<CanvasRenderingContext2d>>, enable_alpha: bool) -> EventListener {
        let initialize_context = Callback::from(move |_: yew::html::onresize::Event| {
            self.set_width(window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32);

            let context_new: CanvasRenderingContext2d = self
                .get_context_with_context_options("2d", &JsValue::from_serde(&serde_json::json!({
                                "alpha": enable_alpha,
                                "depth": false,
                                "stencil": false,
                            })).unwrap())
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            context_new.translate((self.width() / 2) as f64, (self.height() / 2) as f64).unwrap();

            context_state.set(Some(context_new));
        });

        initialize_context.emit(Event::new("").unwrap());

        EventListener::new(
            &window().unwrap(),
            "resize",
            move |e| initialize_context.emit(e.clone()),
        )
    }
}
