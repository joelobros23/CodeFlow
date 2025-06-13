use eframe::{egui, App, Frame, NativeOptions};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::process::{Command, Stdio};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use anyhow::{Result, anyhow};

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into())))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "CodeFlow",
        options,
        Box::new(|cc| {
            // This gives us context to create things like fonts!
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(CodeFlowApp::new(cc))
        }),
    )
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct CodeBlock {
    id: String,
    code: String,
    language: String,
    x: f32,
    y: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct CodeFlowData {
    code_blocks: Vec<CodeBlock>,
}

impl Default for CodeFlowData {
    fn default() -> Self {
        CodeFlowData {
            code_blocks: vec![],
        }
    }
}

struct CodeFlowApp {
    data: CodeFlowData,
    selected_block_id: Option<String>,
    new_block_code: String,
    run_output: String,
}

impl CodeFlowApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        let data = match load_data_from_file("codeflow.json") {
            Ok(d) => d,
            Err(e) => {
                error!("Failed to load data: {}", e);
                CodeFlowData::default()
            }
        };

        CodeFlowApp {
            data,
            selected_block_id: None,
            new_block_code: String::new(),
            run_output: String::new(),
        }
    }
}

impl App for CodeFlowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Add Block").clicked() {
                    let id = uuid::Uuid::new_v4().to_string();
                    self.data.code_blocks.push(CodeBlock {
                        id: id.clone(),
                        code: String::new(),
                        language: "python".to_string(),
                        x: 50.0,
                        y: 50.0,
                    });
                    self.selected_block_id = Some(id);
                }

                if ui.button("Load").clicked() {
                    match load_data_from_file("codeflow.json") {
                        Ok(data) => {
                            self.data = data;
                            self.selected_block_id = None;
                        }
                        Err(e) => error!("Failed to load data: {}", e),
                    }
                }

                if ui.button("Save").clicked() {
                    if let Err(e) = save_data_to_file("codeflow.json", &self.data) {
                        error!("Failed to save data: {}", e);
                    }
                }

                if ui.button("Run").clicked() {
                    if let Some(block_id) = &self.selected_block_id {
                        if let Some(block) = self.data.code_blocks.iter().find(|b| &b.id == block_id) {
                            match run_code(&block.code, &block.language) {
                                Ok(output) => {
                                    self.run_output = output;
                                }
                                Err(e) => {
                                    self.run_output = format!("Error: {}", e);
                                    error!("Failed to run code: {}", e);
                                }
                            }
                        } else {
                            self.run_output = "Error: Selected block not found.".to_string();
                        }
                    } else {
                        self.run_output = "Error: No block selected.".to_string();
                    }
                }

                ui.label(format!("Run Output: {}", self.run_output));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            for block in &mut self.data.code_blocks {
                let id = block.id.clone();
                let title = format!("Code Block {}", id);
                let mut frame = egui::Frame::group(&ctx.style());

                if Some(id.clone()) == self.selected_block_id {
                    frame = frame.stroke(egui::Stroke::new(2.0, egui::Color32::YELLOW));
                }

                frame.show(ui, |ui| {
                    ui.set_width(200.0);
                    ui.set_height(150.0);
                    ui.heading(title);

                    let res = ui.add(egui::TextEdit::multiline(&mut block.code).desired_rows(4).code_editor());
                    if res.gained_focus() {
                        self.selected_block_id = Some(id.clone());
                    }
                    if res.clicked() {
                        self.selected_block_id = Some(id.clone());
                    }

                    ui.horizontal(|ui| {
                        ui.label("Language:");
                        egui::ComboBox::from_label("Language")
                            .selected_text(block.language.clone())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut block.language, "python".to_string(), "Python");
                                ui.selectable_value(&mut block.language, "rust".to_string(), "Rust");
                            });
                    });

                });
            }
        });
    }
}

fn load_data_from_file(path: &str) -> Result<CodeFlowData> {
    let file = File::open(path).map_err(|e| anyhow!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader).map_err(|e| anyhow!("Failed to deserialize data: {}", e))?;
    Ok(data)
}

fn save_data_to_file(path: &str, data: &CodeFlowData) -> Result<()> {
    let file = File::create(path).map_err(|e| anyhow!("Failed to create file: {}", e))?;
    serde_json::to_writer_pretty(file, data).map_err(|e| anyhow!("Failed to serialize data: {}", e))?
    Ok(())
}

fn run_code(code: &str, language: &str) -> Result<String> {
    info!("Running code with language: {}", language);
    match language {
        "python" => {
            let mut cmd = Command::new("python3");
            cmd.arg("-c");
            cmd.arg(code);
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());

            let mut child = cmd.spawn().map_err(|e| anyhow!("Failed to spawn process: {}", e))?;
            let output = child.wait_with_output().map_err(|e| anyhow!("Failed to wait for process: {}", e))?;

            if output.status.success() {
                String::from_utf8(output.stdout).map_err(|e| anyhow!("Failed to convert stdout to string: {}", e))
            } else {
                String::from_utf8(output.stderr).map_err(|e| anyhow!("Failed to convert stderr to string: {}", e))
            }
        }
        "rust" => {
            // Create a temporary file
            let temp_file_path = "temp_code.rs";
            std::fs::write(temp_file_path, code).map_err(|e| anyhow!("Failed to write to temp file: {}", e))?;

            let mut cmd = Command::new("rustc");
            cmd.arg(temp_file_path);
            cmd.arg("-o");
            cmd.arg("temp_code");
            
            let output = cmd.output().map_err(|e| anyhow!("Failed to compile code: {}", e))?;

            if !output.status.success() {
                let err_msg = String::from_utf8(output.stderr).map_err(|e| anyhow!("Failed to decode compiler error: {}", e))?;
                return Err(anyhow!("Compilation failed: {}", err_msg));
            }

            let mut cmd = Command::new("./temp_code");
            let output = cmd.output().map_err(|e| anyhow!("Failed to run compiled code: {}", e))?;

            // Clean up the temporary executable (optional)
            std::fs::remove_file("temp_code").ok();  // Ignore result, cleanup is best effort
            std::fs::remove_file(temp_file_path).ok();

            if output.status.success() {
                String::from_utf8(output.stdout).map_err(|e| anyhow!("Failed to decode output: {}", e))
            } else {
                String::from_utf8(output.stderr).map_err(|e| anyhow!("Failed to decode stderr: {}", e))
            }
        }
        _ => Err(anyhow!("Unsupported language: {}", language)),
    }
}
