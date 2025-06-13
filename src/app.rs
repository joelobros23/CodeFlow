// src/app.rs

use egui::{CentralPanel, Context, ScrollArea, TextEdit, TextStyle, FontFamily, FontId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeFlowSettings {
    pub font_size: f32,
    pub font_family: String, // Consider using an enum for available font families
    // Add more settings as needed, e.g., theme, line numbers, etc.
}

impl Default for CodeFlowSettings {
    fn default() -> Self {
        CodeFlowSettings {
            font_size: 14.0,
            font_family: "monospace".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionData {
    // Placeholder for execution-related data.
    // This could include things like stdout/stderr, execution time,
    // breakpoints, etc. For now, it's just a string.
    pub output: String,
}

impl Default for ExecutionData {
    fn default() -> Self {
        ExecutionData {
            output: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeFlowApp {
    pub source_code: String,
    pub execution_data: ExecutionData,
    pub settings: CodeFlowSettings,
}

impl Default for CodeFlowApp {
    fn default() -> Self {
        CodeFlowApp {
            source_code: String::new(),
            execution_data: ExecutionData::default(),
            settings: CodeFlowSettings::default(),
        }
    }
}

impl CodeFlowApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for CodeFlowApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("CodeFlow");

            let font_id = FontId {
                size: self.settings.font_size,
                family: match self.settings.font_family.as_str() {
                    "monospace" => FontFamily::Monospace,
                    _ => FontFamily::Proportional, // Default to proportional if unknown
                },
            };
            
            let text_style = TextStyle::Monospace;
            ctx.style_mut().text_styles.insert(text_style, font_id.clone());
            

            ScrollArea::vertical().show(ui, |ui| {
                ui.add(TextEdit::multiline(&mut self.source_code).code_editor());
            });

            ui.separator();

            ui.label("Execution Output:");
            ScrollArea::vertical().show(ui, |ui| {
                ui.label(&self.execution_data.output);
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
