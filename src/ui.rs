// src/ui.rs

use egui::{CentralPanel, Context, ScrollArea, TextEdit};
use eframe::App;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct CodeFlowApp {
    #[serde(skip)]
    code: String,
}

impl CodeFlowApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     egui::warn_if_debug_build!(storage.get_string("codeflow_app").unwrap_or_default());
        //     egui::warn_if_debug_build!("failed to load app state");
        // }
        Default::default()
    }
}

impl App for CodeFlowApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Code Editor");

            ScrollArea::vertical().show(ui, |ui| {
                let text_edit = TextEdit::multiline(&mut self.code)
                    .code_editor()
                    .desired_width(f32::INFINITY);

                ui.add(text_edit);
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string("codeflow_app", self.code.clone());
    }
}
