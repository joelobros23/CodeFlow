use eframe::{egui, App, Frame, NativeOptions, run_native};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct CodeFlowApp {
    label: String,
    value: f32,
}

impl App for CodeFlowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CodeFlow");
            ui.label("Hello egui!");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });
            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.label(format!("Thanks for writing: '{}', value: {}", self.label, self.value));
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Exiting...");
    }
}

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };

    run_native(
        "CodeFlow",
        options,
        Box::new(|cc| {
            let app = CodeFlowApp {
                label: "Hello, egui!".to_owned(),
                value: 2.7,
            };
            Box::new(app)
        }),
    )
}