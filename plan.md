# Project Plan: CodeFlow

**Description:** A visual code profiler and execution visualizer that helps developers understand the performance and control flow of their Rust programs.


## Development Goals

- [ ] Set up the basic eframe application boilerplate in `src/main.rs` and create basic file structure
- [ ] Define the application state struct in `src/app.rs`, including fields for source code, execution data, and UI settings.
- [ ] Implement the `eframe::App` trait for the application state struct in `src/app.rs`.
- [ ] Create a UI layout in `src/ui.rs` using `egui` to display the code editor, execution timeline, and other profiling information.
- [ ] Implement a basic code editor widget using `egui::TextEdit::multiline` in `src/ui.rs`.
- [ ] Create a `src/profiler.rs` module to handle program execution and profiling, possibly using tracing/instrumentation.
- [ ] Implement a mechanism to load Rust source code into the editor from a file.
- [ ] Implement a 'Run' button in the UI that triggers code execution and profiling.
- [ ] Capture execution data (function calls, timings, etc.) during program execution and store it in the application state.
- [ ] Visualize the execution data on a timeline using custom `egui` drawing, showing function call durations and relationships.
- [ ] Implement zoom and pan functionality for the timeline view.
- [ ] Add filtering and sorting options for the execution data to focus on specific areas of interest.
- [ ] Implement basic error handling and display error messages in the UI.
- [ ] Add a menu bar with 'File' -> 'Open', 'Save', 'Exit' functionality.
- [ ] Persist UI settings (e.g., zoom level, filters) between application sessions using `serde` and `serde_json`.
