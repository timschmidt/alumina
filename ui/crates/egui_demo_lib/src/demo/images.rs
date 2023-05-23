use egui::*;
use rfd::FileDialog;
use std::fs;
use std::ffi::OsStr;
use std::io::{BufReader, Read, Write};
use zip;
use qoi;
use glium::texture::RawImage2d;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::io::Cursor;
use egui::TextureId;
use egui::{CentralPanel, Frame, Ui};

#[derive(PartialEq, Default, Debug)]
pub struct Images {
    points_to_plot: Vec<[f64; 2]>
}

impl super::Demo for Images {
    fn name(&self) -> &'static str {
        "🗠 Images"
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

impl super::View for Images {
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
                            let image = image::open(path).unwrap(); // Use the path to your image file

                            // Convert the image to an egui texture
                            let image = match image {
                                DynamicImage::ImageRgba8(image) => image,
                                image => image.to_rgba8(),
                            };

                            // Create the egui texture
                            //let egui_texture = egui::Texture::new(image.width(), image.height(), image.into_raw(), egui::TextureFormat::RgbaPremul);
                            //let texture_id = ctx.texture_manager().insert(egui_texture);
                        }
                        if path.extension().unwrap() == OsStr::new("qoi") { // Open an qoi file

                        }
                        if path.extension().unwrap() == OsStr::new("zip") { // Open an zip file

                        }
                    },
                    None    => println!("Please select a file"),
                }
            }

        fn show_image(ui: &mut Ui, texture_id: TextureId) {
            //CentralPanel::default().show(ui, |ui| {
            //    let image = egui::Image::new(texture_id, [300.0, 300.0]); // Set the desired size
            //    Frame::dark_canvas(ui.style()).show(ui, |ui| {
            //        ui.add(image);
            //    });
            //});
        }
    }
}