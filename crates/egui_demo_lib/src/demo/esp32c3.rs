use egui::*;
use rfd::AsyncFileDialog;
use std::fs;
use std::ffi::OsStr;
use std::io::{BufReader, Read, Write};
use zip;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::io::Cursor;
use std::future::Future;
use egui::TextureId;
use egui_extras::RetainedImage;
use egui::{CentralPanel, Frame, Ui};
use std::sync::{Arc, Mutex};

#[derive(Default, Debug)]
pub struct esp32c3 {

}

impl super::Demo for esp32c3 {
    fn name(&self) -> &'static str {
        "🗠 2D bitmap view"
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

impl super::View for esp32c3 {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {
        //ui.image(TextureId::User(0), vec2(100.0, 100.0));

    }
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
}
#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}