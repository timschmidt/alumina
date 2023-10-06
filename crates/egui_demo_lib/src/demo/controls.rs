use egui::*;

#[derive(PartialEq, Default, Debug)]
pub struct Controls {
    axis_offsets: [f64; 5],
    // Add a property to store the offset value for each axis
}

impl super::Demo for Controls {
    fn name(&self) -> &'static str {
        "🗠 Jog controls"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        use super::View as _;
        Window::new(self.name())
            .open(open)
            .default_size(vec2(200.0, 600.0))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for Controls {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {
        let axes = ["X", "Y", "Z", "A", "B"];
        for axis in axes {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui.button(format!("Home {}", axis)).clicked() {
                        // Home the axis
                    }
                    let jog_values = [-100.0, -10.0, -1.0, -0.1, 0.1, 1.0, 10.0, 100.0];
                    for &jog_value in jog_values.iter() {
                        if ui.button(format!("{}", jog_value)).clicked() {
                            // Jog the axis by jog_value
                        }
                    }
                    ui.add(Slider::new(&mut self.axis_offsets[0], -1000.0..=1000.0).text("Offset"));
                });
            });
        }
    }
}

