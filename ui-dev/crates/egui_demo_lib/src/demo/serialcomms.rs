use egui::*;

#[derive(PartialEq, Default, Debug)]
pub struct SerialComms {
    points_to_plot: Vec<[f64; 2]>
}

impl super::Demo for SerialComms {
    fn name(&self) -> &'static str {
        "🗠 Serial Communications"
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

impl super::View for SerialComms {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {
        let ui_connect = ui.button("Connect").on_hover_text("Initiate serial communications");
        if ui_connect.clicked(){

        }


    }
}