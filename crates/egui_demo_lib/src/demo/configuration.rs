use egui::*;

#[derive(PartialEq, Default, Debug)]
pub struct Configuration {
    axis_offsets: [f64; 5],
    radio: WirelessType,
    ssid: String,
    string: String,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum WirelessType {
    Client,
    AccessPoint,
}

impl Default for WirelessType {
    fn default() -> Self {
        WirelessType::Client
    }
}

impl super::Demo for Configuration {
    fn name(&self) -> &'static str {
        "🗠 Configuration"
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

impl super::View for Configuration {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label("Wireless mode");
            let ui_wireless_mode = egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", self.radio))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(true);
                    ui.set_min_width(60.0);
                    ui.selectable_value(&mut self.radio, WirelessType::Client, "Client");
                    ui.selectable_value(&mut self.radio, WirelessType::AccessPoint, "Access Point");
                });
            if ui.button("Scan").on_hover_text("Scan for available wireless networks").clicked() {
                // Scan wifi networks and populate dropdown
            }
            let ui_wireless_ssid = egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", self.ssid))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(true);
                    ui.set_min_width(60.0);
                    ui.selectable_value(&mut self.ssid, "SSID1".to_string(), "SSID1");
                    ui.selectable_value(&mut self.ssid, "SSID2".to_string(), "SSID2");
                });
            ui.label("Tool 1 width");
            let ui_tool_one_width = egui::TextEdit::singleline(&mut self.string).hint_text("60").show(ui);
            ui.label("Tool 2 width");
            let ui_tool_two_width = egui::TextEdit::singleline(&mut self.string).hint_text("60").show(ui);
            ui.label("Tool 3 width");
            let ui_tool_three_width = egui::TextEdit::singleline(&mut self.string).hint_text("60").show(ui);
            ui.label("Tool 4 width");
            let ui_tool_four_width = egui::TextEdit::singleline(&mut self.string).hint_text("30").show(ui);
            ui.label("Tool 1 offset");
            let ui_tool_one_offset = egui::TextEdit::singleline(&mut self.string).hint_text("100").show(ui);
            ui.label("Tool 2 offset");
            let ui_tool_two_offset = egui::TextEdit::singleline(&mut self.string).hint_text("200").show(ui);
            ui.label("Tool 3 offset");
            let ui_tool_three_offset = egui::TextEdit::singleline(&mut self.string).hint_text("300").show(ui);
            ui.label("Tool 4 offset");
            let ui_tool_four_offset = egui::TextEdit::singleline(&mut self.string).hint_text("400").show(ui);
        });

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

        let ui_save_configuration = ui.button("Save configuration").on_hover_text("Save configuration to device");
    }
}

