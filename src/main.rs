use eframe::{egui, App, CreationContext, Frame};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CodeFlowApp {
    // Example: You can add your application state here.
    // e.g., a counter:
    value: i32,
}

impl Default for CodeFlowApp {
    fn default() -> Self {
        Self {
            value: 0,
        }
    }
}

impl CodeFlowApp {
    /// Called once before the first frame.
    pub fn new(cc: &CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using `cc.egui_ctx`.
        // Example: Override the default fonts with some that support the language you prefer:
        // cc.egui_ctx.set_fonts(egui::FontDefinitions::default());

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default(); // deserializes the struct and returns it. Returns default if it can't
        }

        Default::default()
    }
}

impl App for CodeFlowApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just use a `CentralPanel`.

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
                ui.text_edit_singleline(&mut String::new());
            });

            ui.add(egui::Slider::new(&mut self.value, 0..=10).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    egui::warn_if_debug_build(ui);
                    ui.label("CodeFlow");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("CodeFlow");
            ui.hyperlink("https://github.com/jkelol111/codeflow");
            ui.label("An application for visualizing and interacting with code.");

            ui.label(format!("Current value: {}", self.value));

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label("Powered by egui");
            });
        });
    }

    /// Called once to store app state when closing.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}


fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init(); // Initialize tracing for logging.

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "CodeFlow",
        options,
        Box::new(|cc| {
            // This gives you access to the eframe::CreationContext which gives you:
            // * `egui_ctx`: egui::Context
            // * `storage`: Option<Box<dyn eframe::Storage>>
            // * `window`: eframe::PlatformWindow
            // * `gl`: Option<Arc<dyn eframe::glow::Context>>
            //
            // ...for egui's immediate mode style rendering.
            Box::new(CodeFlowApp::new(cc))
        }),
    )
}
