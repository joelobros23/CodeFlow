use egui::{CentralPanel, TextEdit, Ui};

pub struct CodeEditor {
    code: String,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            code: String::new(),
        }
    }
}

impl CodeEditor {
    pub fn ui(&mut self, ui: &mut Ui) {
        CentralPanel::default().show(ui, |ui| {
            ui.heading("Code Editor");
            ui.add(TextEdit::multiline(&mut self.code).code_editor());
        });
    }
}
