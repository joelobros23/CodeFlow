// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::CentralPanel;
use eframe::Frame;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

mod profiler;

#[derive(Deserialize, Serialize, Default)]
pub struct CodeFlowApp {
    #[serde(skip)]
    value: f32,
}

impl CodeFlowApp {
    #[instrument]
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals`.
        profiler::init_profiler();

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            info!("Loading app state from storage");
            serde_json::from_str(storage.get_string("codeflow_app").unwrap_or_default().as_str()).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for CodeFlowApp {
    #[instrument(skip(self, _frame))] // Skip tracing _frame to avoid tracing framework types.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut String::new());
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/crates/eframe");
                    ui.label(".");
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.label("\nClick each box to change its size:");
            ui.label(format!("Value: {}", self.value));
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        info!("Saving app state to storage");
        if let Ok(json_data) = serde_json::to_string(self) {
            storage.set_string("codeflow_app", json_data);
        } else {
            tracing::error!("Failed to serialize app state");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 1024.0)),
        ..Default::default()
    };

    eframe::run_native(
        "CodeFlow",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(CodeFlowApp::new(cc))
        }),
    )
}
