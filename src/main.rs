use eframe::{egui, App, CreationContext, Frame};
use egui::Color32;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use std::time::{Duration, Instant};

// Mock execution data structure (replace with actual data)
#[derive(Debug, Serialize, Deserialize, Clone)]
struct FunctionCall {
    name: String,
    start_time: Duration,
    duration: Duration,
    children: Vec<FunctionCall>,
}

impl FunctionCall {
    fn new(name: String, start_time: Duration, duration: Duration, children: Vec<FunctionCall>) -> Self {
        FunctionCall {
            name,
            start_time,
            duration,
            children,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CodeFlowApp {
    execution_data: FunctionCall,
    start_time: Instant,
    zoom_level: f32,
}

impl Default for CodeFlowApp {
    fn default() -> Self {
        let mock_data = FunctionCall::new(
            "root".to_string(),
            Duration::from_millis(0),
            Duration::from_millis(1000),
            vec![
                FunctionCall::new(
                    "function_a".to_string(),
                    Duration::from_millis(100),
                    Duration::from_millis(200),
                    vec![],
                ),
                FunctionCall::new(
                    "function_b".to_string(),
                    Duration::from_millis(350),
                    Duration::from_millis(150),
                    vec![
                        FunctionCall::new(
                            "function_b_1".to_string(),
                            Duration::from_millis(400),
                            Duration::from_millis(50),
                            vec![],
                        ),
                    ],
                ),
                FunctionCall::new(
                    "function_c".to_string(),
                    Duration::from_millis(600),
                    Duration::from_millis(300),
                    vec![],
                ),
            ],
        );

        Self {
            execution_data: mock_data,
            start_time: Instant::now(),
            zoom_level: 1.0,
        }
    }
}

impl CodeFlowApp {
    fn new(_cc: &CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_visuals and cc.egui_ctx.set_fonts.
        Self::default()
    }

    fn draw_timeline(
        &mut self,
        ui: &mut egui::Ui,
        function_call: &FunctionCall,
        offset: Duration,
        level: usize,
    ) {
        let start = offset + function_call.start_time;
        let end = start + function_call.duration;

        let timeline_width = 1000.0 * self.zoom_level;
        let rect_start = (start.as_secs_f32() / (self.execution_data.duration.as_secs_f32())) * timeline_width;
        let rect_width = (function_call.duration.as_secs_f32() / (self.execution_data.duration.as_secs_f32())) * timeline_width;
        let rect_height = 20.0;
        let y_offset = level as f32 * (rect_height + 5.0) + 50.0;

        let rect = egui::Rect::from_min_size(
            egui::Pos2::new(rect_start, y_offset),
            egui::Vec2::new(rect_width, rect_height),
        );

        let color = match function_call.name.as_str() {
            "root" => Color32::GRAY,
            "function_a" => Color32::BLUE,
            "function_b" => Color32::GREEN,
            "function_c" => Color32::RED,
            "function_b_1" => Color32::YELLOW,
            _ => Color32::WHITE,
        };

        ui.painter().rect_filled(rect, 5.0, color);
        ui.painter().text(
            rect.min + egui::Vec2::new(5.0, 2.0),
            egui::Align2::LEFT_TOP,
            &function_call.name,
            egui::FontId::proportional(12.0),
            Color32::BLACK,
        );

        for child in &function_call.children {
            self.draw_timeline(ui, child, start, level + 1);
        }
    }
}

impl App for CodeFlowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Settings");

            ui.add(egui::Slider::new(&mut self.zoom_level, 0.1..=5.0).text("Zoom"));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Execution Timeline");
            ui.label(format!("Total Duration: {:?}", self.execution_data.duration));

            let timeline_width = 1000.0 * self.zoom_level;
            ui.set_width(timeline_width); // Ensure the CentralPanel has the desired width

            let mut scroll_delta = egui::Vec2::ZERO;

            // Enable horizontal scrolling with mouse wheel
            ctx.input(|i| {
                if i.modifiers.is_none() {
                    scroll_delta.x += i.scroll_delta.y;
                }
            });
            egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.set_width(timeline_width);
              self.draw_timeline(ui, &self.execution_data, Duration::from_millis(0), 0);
            });
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
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
