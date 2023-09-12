#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct About {}

impl super::Demo for About {
    fn name(&self) -> &'static str {
        "About Alumina"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(320.0)
            .open(open)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for About {
    fn ui(&mut self, ui: &mut egui::Ui) {
        use egui::special_emojis::{OS_APPLE, OS_LINUX, OS_WINDOWS};

        ui.label(format!(
            "Alumina is a latest-generation CNC firmware and user interface.  Alumina is built using Rust, WebAssembly, RISC-v, and egui.  Alumina is designed to be easy to use, portable, and fast."
        ));

        ui.add_space(12.0); // ui.separator();
        ui.heading("Links");
        links(ui);
    }
}

fn links(ui: &mut egui::Ui) {
    use egui::special_emojis::{GITHUB, TWITTER};
    ui.hyperlink_to(
        format!("{} Alumina on GitHub", GITHUB),
        "https://github.com/timschmidt/alumina",
    );
    ui.hyperlink_to(
        format!("{} @gridbeamtim", TWITTER),
        "https://twitter.com/gridbeamtim",
    );
    ui.hyperlink_to("Alumina documentation", "https://docs.rs/alumina/");
}
