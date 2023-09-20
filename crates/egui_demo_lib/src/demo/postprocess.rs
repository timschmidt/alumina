use egui::*;
use rfd::AsyncFileDialog;
use std::future::Future;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Postprocess {
    file:  Arc<Mutex<Vec<u8>>>,
    radio: Enum,
    string: String,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum Enum {
    First,
    Second,
    Third,
}

impl Postprocess {
    pub fn new() -> Postprocess {
        Postprocess {
            file: Arc::new(Mutex::new(vec![])),
            radio: Enum::First,
            string: Default::default(),
        }
    }
}

impl Default for Postprocess {
    fn default() -> Self {
        Self {
            file: Arc::new(Mutex::new(vec![])),
            radio: Enum::First,
            string: Default::default(),
        }
    }
}

impl super::Demo for Postprocess {
    fn name(&self) -> &'static str {
        "🗠 Gcode post processor"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        use super::View as _;
        Window::new(self.name())
            .open(open)
            .default_size(vec2(1200.0, 800.0))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for Postprocess {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {
        let ui_open_file = ui
            .button("Open file")
            .on_hover_text("Open gcode file");
        ui.label("Profile");
        let ui_profile = egui::ComboBox::from_id_source()
            .selected_text(format!("{:?}", self.radio))
            .show_ui(ui, |ui| {
                ui.style_mut().wrap = Some(false);
                ui.set_min_width(60.0);
                ui.selectable_value(&mut self.radio, Enum::First, "First");
                ui.selectable_value(&mut self.radio, Enum::Second, "Second");
                ui.selectable_value(&mut self.radio, Enum::Third, "Third");
            });
        ui.label("Tool 1 width");
        let ui_tool_one_width = egui::TextEdit::singleline(&mut self.string).hint_text("60");
        let ui_tool_two_width = egui::TextEdit::singleline(&mut self.string).hint_text("60");
        let ui_tool_three_width = egui::TextEdit::singleline(&mut self.string).hint_text("60");
        let ui_tool_four_width = egui::TextEdit::singleline(&mut self.string).hint_text("30");
        let ui_tool_one_offset = egui::TextEdit::singleline(&mut self.string).hint_text("0");
        let ui_tool_two_offset = egui::TextEdit::singleline(&mut self.string).hint_text("100");
        let ui_tool_three_offset = egui::TextEdit::singleline(&mut self.string).hint_text("200");
        let ui_tool_four_offset = egui::TextEdit::singleline(&mut self.string).hint_text("300");
        let ui_process_file = ui.button("Process and save").on_hover_text("Process the gcode file and save the result");

        let file_arc = Arc::clone(&self.file);

        let filepicker_future = async move {
            let filepicker = AsyncFileDialog::new()
                .add_filter(
                    "Gcode files (gcode, nc, ngc)",
                    &["gcode", "nc", "ngc"],
                )
                .pick_file()
                .await
                .expect("no file has been selected");

            let mut file = file_arc.lock().unwrap();
            *file = filepicker.read().await;
        };

        if ui_open_file.clicked() {
            execute(filepicker_future);
        }

        if let Ok(file_lock) = self.file.lock() {
            if !file_lock.is_empty() {
                //let drawing = Drawing::load(&mut file_lock.as_slice());


            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
}
#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}