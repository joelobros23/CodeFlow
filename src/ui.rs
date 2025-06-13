// src/ui.rs
use egui::{CentralPanel, SidePanel, TopBottomPanel, ScrollArea};

#[derive(Default)]
pub struct CodeFlowUi {
    code_editor_content: String,
    execution_timeline_content: String,
    profiling_info_content: String,
}

impl CodeFlowUi {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("CodeFlow");
        });

        SidePanel::left("code_editor_panel").show(ctx, |ui| {
            ui.heading("Code Editor");
            ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.code_editor_content)
                    .code_editor()
                    .desired_width(f32::INFINITY));
            });
        });

        SidePanel::right("profiling_panel").show(ctx, |ui| {
            ui.heading("Profiling Information");
            ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.profiling_info_content)
                .desired_width(f32::INFINITY)
                .font(egui::FontId::monospace(12.0)));
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Execution Timeline");
            ScrollArea::vertical().show(ui, |ui| {
                 ui.add(egui::TextEdit::multiline(&mut self.execution_timeline_content)
                    .desired_width(f32::INFINITY)
                    .font(egui::FontId::monospace(12.0)));
            });
        });
    }

    pub fn update_code_editor(&mut self, content: String) {
        self.code_editor_content = content;
    }

    pub fn update_execution_timeline(&mut self, content: String) {
        self.execution_timeline_content = content;
    }

    pub fn update_profiling_info(&mut self, content: String) {
        self.profiling_info_content = content;
    }
}
