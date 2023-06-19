use egui::*;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Default, Debug, PartialEq)]
pub struct ApplicationLog {
    log_entries: Vec<String>,
}

impl ApplicationLog {
    pub fn add_entry(&mut self, entry: String) {
        self.log_entries.push(entry);
    }
}

impl super::Demo for ApplicationLog {
    fn name(&self) -> &'static str {
        "↕ Log"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(false)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for ApplicationLog {
    fn ui(&mut self, ui: &mut Ui) {
        ui.label("Log entries enter from the bottom, the log will scroll to follow them unless moved.");
        ui.add_space(4.0);

        let text_style = TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        ScrollArea::vertical().stick_to_bottom(true).show_rows(
            ui,
            row_height,
            self.log_entries.len(),
            |ui, row_range| {
                for row in row_range {
                    let text = &self.log_entries[row];
                    ui.label(text);
                }
            },
        );

        ui.ctx().request_repaint();
    }
}