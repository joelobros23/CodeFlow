use eframe::{egui, Frame};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct CodeFlowApp {
    // Example state - replace with your actual application state
    label: String,
    value: f32,
}

impl CodeFlowApp {
    /// Called once before the first frame. Initialize state here.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using `cc.egui_ctx.set_visuals`.
        let mut app = Self::default();

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            if let Some(loaded_app) = eframe::get_value(storage, eframe::APP_KEY) {
                app = loaded_app;
            }
        }

        app
    }
}

impl eframe::App for CodeFlowApp {
    /// Called each time the UI needs repainting, which may be many times per second. Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        // Examples of how to create different panels and add ui elements.
        // Try creating your own widgets!

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    egui::warn_if_debug_build(ui);
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

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.label("Click on the left panel to explore more.");

            ui.label(format!("Hello '{}', value: {}", self.label, self.value));
        });
    }

    /// Called once to store the state before shutting down.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}