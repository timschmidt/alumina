use egui::*;

#[derive(PartialEq, Default, Debug)]
pub struct Catalog {
    points_to_plot: Vec<[f64; 2]>
}

impl super::Demo for Catalog {
    fn name(&self) -> &'static str {
        "🗠 Catalog"
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

impl super::View for Catalog {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {


    }
}