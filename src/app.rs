use eframe::{egui, Frame};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct CodeFlowApp {
    // Example state
    #[serde(skip)]
    pub counter: i32,
}

impl CodeFlowApp {
    /// Called once before the first frame.    
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look and feel of egui using
        // `cc.egui_ctx.set_visuals`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for CodeFlowApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration, check out the `egui_demo_lib` crate.

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CodeFlow");
            ui.label(format!("Counter: {}", self.counter));

            if ui.button("Increment").clicked() {
                self.counter += 1;
            }
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
