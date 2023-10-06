use egui::*;
use rfd::AsyncFileDialog;
use std::future::Future;
use std::sync::{Arc, Mutex};
use url::{Url, Host, Position};

#[derive(Default, Debug)]
pub struct Files {
    upload_file:  Arc<Mutex<Vec<u8>>>,
}

impl Files {
    pub fn new() -> Files {
        Files {
            upload_file: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl super::Demo for Files {
    fn name(&self) -> &'static str {
        "🗠 Files"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        use super::View as _;
        Window::new(self.name())
            .open(open)
            .default_size(vec2(600.0, 400.0))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for Files {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {
        let ui_upload_file = ui
            .button("Upload file")
            .on_hover_text("Choose a file to upload");
         let ui_files_list = ui.button("List files").on_hover_text("List files stored on device");

        let upload_file_arc = Arc::clone(&self.upload_file);

        let filepicker_future = async move {
            let filepicker = AsyncFileDialog::new()
                .add_filter(
                    "Any file (*.*)",
                    &["*"],
                )
                .pick_file()
                .await
                .expect("no file has been selected");

            let mut upload_file = upload_file_arc.lock().unwrap();
            *upload_file = filepicker.read().await;
        };

        if ui_files_list.clicked() {
            execute(list_files());
        }

        if ui_upload_file.clicked() {
            execute(filepicker_future);
        }

        if let Ok(upload_file_lock) = self.upload_file.lock() {
            if !upload_file_lock.is_empty() {
                // upload file contents
            }
        }
    }
}

async fn list_files() -> () {
    // Replace with your actual endpoint
    let url = "http://alumina/files";

    // Define the plain text data to send (adjust as needed)
    let data = "list_files";

    // Make the POST request
    let client = reqwest::Client::new();
    let response = client.post(url).body(data).send().await;
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