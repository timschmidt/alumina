use egui::*;
use rfd::FileDialog;
use std::fs;
use std::ffi::OsStr;
use std::io::{BufReader, Read, Write};
use zip;
use qoi;

#[derive(PartialEq, Default, Debug)]
pub struct Voxels {
    points_to_plot: Vec<[f64; 2]>
}

impl super::Demo for Voxels {
    fn name(&self) -> &'static str {
        "🗠 Voxels"
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

impl super::View for Voxels {
    #[allow(clippy::unused_self)]
        fn ui(&mut self, ui: &mut Ui) {
            let ui_open_file = ui.button("Open file").on_hover_text("PNG, QOI, and ZIP are supported");
            if ui_open_file.clicked(){
                let mut file = FileDialog::new()
                    .add_filter("Voxel files (zip, png, qoi)", &["zip", "png", "qoi"])
                    .pick_file();

                match &file {
                    Some(path) => {
                        if path.extension().unwrap() == OsStr::new("png") { // Open a png file

                        }
                        if path.extension().unwrap() == OsStr::new("qoi") { // Open an qoi file

                        }
                        if path.extension().unwrap() == OsStr::new("zip") { // Open an zip file

                        }
                    },
                    None    => println!("Please select a file"),
                }
            }


    }
}