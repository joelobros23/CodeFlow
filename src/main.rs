// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::CentralPanel;
use eframe::Frame;
use serde::{Deserialize, Serialize};
use tracing::info;

mod profiler;

#[derive(Deserialize, Serialize, Default)]
pub struct CodeFlowApp {
    text: String,
    #[serde(skip)]
    profiler: profiler::Profiler,
}

impl CodeFlowApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.
        default_dark_theme(&cc.egui_ctx);

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for CodeFlowApp {
    /// Called each time the UI needs to be repainted, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Put your widgets into a `CentralPanel` for convenience.
        CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("CodeFlow");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.text);
            });

            ui.label("What you wrote:");
            ui.monospace(self.text.as_str());

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(" v");
                    ui.label(eframe::VERSION);
                });
                if ui.button("Start Profiling").clicked() {
                    self.profiler.start();
                    info!("Profiling started");
                }
                if ui.button("Stop Profiling").clicked() {
                    self.profiler.stop();
                    info!("Profiling stopped");
                }
            });
        });
    }

    /// Called once before the first frame.    
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &mut Frame,
        storage: Option<&dyn eframe::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that restoration needs to happen first, before the first frame!
        if let Some(storage) = storage {
            *self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn default_dark_theme(ctx: &egui::Context) {
    ctx.set_visuals(egui::Visuals::dark());
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "CodeFlow",
        options,
        Box::new(|cc| {
            Box::new(CodeFlowApp::new(cc))
        }),
    )
}
