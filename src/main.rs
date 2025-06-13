use eframe::{egui, App, Frame, NativeOptions};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tracing::{debug, error, info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use anyhow::{Result, anyhow};


fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let native_options = NativeOptions::default();
    eframe::run_native(
        "CodeFlow",
        native_options,
        Box::new(|cc| Box::new(CodeFlowApp::new(cc))), 
    )?; // Removed the ? from here

    Ok(())
}


#[derive(Serialize, Deserialize, Default)]
struct CodeFlowState {
    code: String,
    output: String,
}

#[derive(Default, Clone)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub duration: std::time::Duration,
}

struct CodeFlowApp {
    state: CodeFlowState,
    execution_result: Arc<Mutex<Option<ExecutionResult>>>, // Use Arc<Mutex<Option>>
    rt: Arc<Runtime>,
}

impl CodeFlowApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: CodeFlowState::default(),
            execution_result: Arc::new(Mutex::new(None)),
            rt: Arc::new(Runtime::new().unwrap()),
        }
    }

    #[instrument(skip(self), fields(code_length = self.state.code.len()))]
    fn execute_code(&self) -> Result<()> {
        let code = self.state.code.clone();
        let execution_result_clone = self.execution_result.clone(); // Clone the Arc

        self.rt.spawn(async move {
            info!("Executing code...");
            let start_time = std::time::Instant::now();
            let result = execute_code_async(&code).await;
            let duration = start_time.elapsed();

            let mut execution_result = execution_result_clone.lock().unwrap();
            match result {
                Ok(stdout) => {
                    info!("Code executed successfully");
                    *execution_result = Some(ExecutionResult {
                        stdout,
                        stderr: String::new(),
                        duration,
                    });
                }
                Err(e) => {
                    error!("Code execution failed: {}", e);
                    *execution_result = Some(ExecutionResult {
                        stdout: String::new(),
                        stderr: format!("{}", e),
                        duration,
                    });
                }
            }
        });
        Ok(())
    }
}

#[async_instrument]
async fn execute_code_async(code: &str) -> Result<String> {
    // Placeholder for actual code execution.  Replace with a proper sandboxed execution environment
    debug!("Running code: {}", code);
    tokio::time::sleep(std::time::Duration::from_millis(100)).await; // Simulate some work

    if code.contains("error") {
        Err(anyhow::anyhow!("Simulated runtime error"))
    } else {
        Ok(format!("Code executed successfully.  Input code length: {}", code.len()))
    }
}

impl App for CodeFlowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("CodeFlow");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Code:");
                if ui.button("Run").clicked() {
                    if let Err(e) = self.execute_code() {
                        error!("Failed to start code execution: {}", e);
                    }
                }
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.state.code).code_editor());
            });

            ui.separator();

            ui.label("Output:");
            let mut output_text = String::new();
            if let Ok(execution_result_guard) = self.execution_result.lock() {
                if let Some(execution_result) = execution_result_guard.as_ref() {
                    output_text.push_str(&format!("Stdout: {}\n", execution_result.stdout));
                    output_text.push_str(&format!("Stderr: {}\n", execution_result.stderr));
                    output_text.push_str(&format!("Duration: {:?}\n", execution_result.duration));
                } else {
                    output_text.push_str("Executing...");
                }
            }
            ui.add(egui::TextEdit::multiline(&mut output_text).read_only());
        });
    }
}