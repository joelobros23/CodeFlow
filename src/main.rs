// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, App, CreationContext};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default)]
struct CodeFlowApp {
    text: String,
    #[serde(skip)]
    file_path: Option<PathBuf>,
}

impl CodeFlowApp {
    fn new(cc: &CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn load_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Rust", &["rs"])
            .pick_file() {
            match fs::read_to_string(&path) {
                Ok(contents) => {
                    self.text = contents;
                    self.file_path = Some(path);
                }
                Err(e) => {
                    eprintln!("Failed to load file: {}", e);
                }
            }
        }
    }
}

impl App for CodeFlowApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CodeFlow");

            if ui.button("Load File").clicked() {
                self.load_file();
            }

            if let Some(path) = &self.file_path {
                ui.label(format!("Loaded file: {}", path.display()));
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.text).code_editor());
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "CodeFlow",
        options,
        Box::new(|cc| {
            // This gives us context to inspect things like fonts and the display.
            // It also gives us access to the storage (e.g. localStorage).
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(CodeFlowApp::new(cc))
        }),
    )
}
