use egui::*;

#[derive(PartialEq, Default, Debug)]
pub struct Database {
    points_to_plot: Vec<[f64; 2]>
}

impl super::Demo for Database {
    fn name(&self) -> &'static str {
        "🗠 Database"
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

impl super::View for Database {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {


    }
}