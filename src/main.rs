// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, App, CreationContext};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Deserialize, Serialize, Default)]
pub struct CodeFlowApp {
    text: String,
    #[serde(skip)]
    error_message: Option<String>,
}

impl CodeFlowApp {
    fn new(cc: &CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.
        default_dark_theme(&cc.egui_ctx);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn load_data(&mut self) -> Result<(), anyhow::Error> {
        // Simulate a function that might fail.
        if self.text.contains("error") {
            return Err(anyhow::anyhow!("Simulated data loading error"));
        }

        info!("Data loaded successfully!");
        Ok(())
    }

    fn handle_load_data(&mut self) {
        match self.load_data() {
            Ok(_) => {
                self.error_message = None; // Clear any previous errors
            }
            Err(e) => {
                error!("Failed to load data: {}", e);
                self.error_message = Some(format!("Failed to load data: {}", e));
            }
        }
    }
}

impl App for CodeFlowApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CodeFlow with Error Handling");

            ui.text_edit_multiline(&mut self.text);

            if ui.button("Load Data").clicked() {
                self.handle_load_data();
            }

            // Display error message if any
            if let Some(error) = &self.error_message {
                ui.label(egui::RichText::new(error).color(egui::Color32::RED));
            }

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
                    ui.label(".");
                });
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn default_dark_theme(ctx: &egui::Context) {
    ctx.set_visuals(egui::Visuals::dark());
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(480.0, 640.0)),
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
