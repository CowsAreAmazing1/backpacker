// Module declarations
pub mod board;
mod player;
pub mod state;
mod status;
mod ui;
mod utils;

pub mod cards {
    pub mod advice;
    pub mod bonus;
    pub mod card;
    pub mod country;
    pub mod grey;
    pub mod special;
}

pub use ui::GameEgui;

pub const NUM_PLAYERS: usize = 4;
pub const HAND_SIZE: usize = 5;

#[cfg(not(target_arch = "wasm32"))]
pub fn run_native() -> eframe::Result<()> {
    eframe::run_native(
        "Backpacker",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(GameEgui::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
/// Handle used by JavaScript to start and stop the web app.
#[derive(Clone)]
#[wasm_bindgen]
pub struct WebHandle {
    runner: eframe::WebRunner,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();

        Self {
            runner: eframe::WebRunner::new(),
        }
    }

    #[wasm_bindgen]
    pub async fn start(
        &self,
        canvas: web_sys::HtmlCanvasElement,
    ) -> Result<(), wasm_bindgen::JsValue> {
        self.runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(GameEgui::new(cc)))),
            )
            .await
    }

    #[wasm_bindgen]
    pub fn destroy(&self) {
        self.runner.destroy();
    }

    #[wasm_bindgen]
    pub fn has_panicked(&self) -> bool {
        self.runner.has_panicked()
    }
}
