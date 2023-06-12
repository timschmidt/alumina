use egui::*;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct NetworkComms {

}

impl super::Demo for NetworkComms {
    fn name(&self) -> &'static str {
        "🗠 Network Communications"
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

impl super::View for NetworkComms {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {
        let ui_connect = ui.button("Connect").on_hover_text("Initiate network communications");
        if ui_connect.clicked(){

        }

    }
}