use eframe::{egui, App, Frame, NativeOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use tracing::{debug, info, instrument};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ExecutionData {
    function_name: String,
    start_time: std::time::Instant,
    end_time: Option<std::time::Instant>,
    duration: Option<std::time::Duration>,
}

impl ExecutionData {
    fn new(function_name: String) -> Self {
        ExecutionData {
            function_name,
            start_time: std::time::Instant::now(),
            end_time: None,
            duration: None,
        }
    }

    fn end(&mut self) {
        self.end_time = Some(std::time::Instant::now());
        self.duration = self.end_time.map(|end| end.duration_since(self.start_time));
    }
}

#[derive(Serialize, Deserialize)]
struct CodeFlowApp {
    #[serde(skip)]
    execution_data: Mutex<Vec<ExecutionData>>,
    name: String,
    age: u32,
}

impl Default for CodeFlowApp {
    fn default() -> Self {
        Self {
            execution_data: Mutex::new(Vec::new()),
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl App for CodeFlowApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                function_to_instrument(self);
            }

            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.separator();

            ui.heading("Execution Data");
            let execution_data_guard = self.execution_data.lock().unwrap();
            for data in execution_data_guard.iter() {
                ui.label(format!(
                    "Function: {}, Duration: {:?}",
                    data.function_name,
                    data.duration
                ));
            }

            if ui.button("Quit").clicked() {
                frame.close();
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}
}

#[instrument]
fn function_to_instrument(app: &mut CodeFlowApp) {
    let mut execution_data = ExecutionData::new("function_to_instrument".to_string());
    {
        let mut execution_data_guard = app.execution_data.lock().unwrap();
        execution_data_guard.push(execution_data.clone());
    }
    some_other_function();
    execution_data.end();
    let mut execution_data_guard = app.execution_data.lock().unwrap();
    if let Some(last) = execution_data_guard.last_mut() {
        if last.function_name == execution_data.function_name && last.end_time.is_none() {
            *last = execution_data;
        }
    }
    debug!("This is inside function_to_instrument");
}

#[instrument]
fn some_other_function() {
    let _timer = Timer::new("some_other_function".to_string());
    std::thread::sleep(std::time::Duration::from_millis(100));
    debug!("This is inside some_other_function");
}

struct Timer {
    name: String,
    start: std::time::Instant,
}

impl Timer {
    fn new(name: String) -> Self {
        info!("Entering {}", name);
        Timer {
            name,
            start: std::time::Instant::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        info!("Exiting {}, elapsed: {:?}", self.name, elapsed);
    }
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "CodeFlow",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let app = CodeFlowApp::default();
            Box::new(app)
        }),
    )
}
