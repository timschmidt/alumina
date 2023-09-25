use dxf::entities::*;
use dxf::Drawing;
use egui::*;
use plot::{Plot, PlotResponse};
use rfd::AsyncFileDialog;
use std::future::Future;
//use svg2polylines::{self, Polyline};
use std::sync::{Arc, Mutex};
use url::{Url, Host, Position};
use cavalier_contours::{pline_closed, polyline::Polyline, polyline::PlineSource, polyline::PlineVertex};

//use crate::demo::Demo;

#[derive(Default, Debug)]
pub struct Toolpath {
    points_to_plot: Vec<[f64; 2]>,
    cad_file:  Arc<Mutex<Vec<u8>>>,
}

#[derive(Debug)]
struct BoundingBox {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl Toolpath {
    pub fn new() -> Toolpath {
        Toolpath {
            points_to_plot: vec![],
            cad_file: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl super::Demo for Toolpath {
    fn name(&self) -> &'static str {
        "🗠 2D vector view"
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

impl super::View for Toolpath {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {
        let points = self.points_to_plot.clone();
        let mut line_to_plot = egui::widgets::plot::Line::new(points);
        let plot = Plot::new("Geometry").height(700.0).allow_scroll(false);

        let PlotResponse {
            response,
            inner: line_to_plot,
            ..
        } = plot.show(ui, |plot_ui| (plot_ui.line(line_to_plot),));

        let offset = 5.0; // mm
        let polylines = convert_to_polylines(self.points_to_plot.clone());

        let ui_open_file = ui
            .button("Open file")
            .on_hover_text("SVG and DXF are supported");
        let ui_toolpath_shrink = ui
            .button("Shrink")
            .on_hover_text("Shrink the toolpath by 5mm");
        let ui_toolpath_grow = ui.button("Grow").on_hover_text("Grow the toolpath by 5mm");
        let ui_toolpath_status_on = ui.button("On").on_hover_text("Turn the status light on");
        let ui_toolpath_status_off = ui.button("Off").on_hover_text("Turn the status light off");
        let ui_toolpath_send = ui.button("Send").on_hover_text("Send geometry to the machine");
        let ui_toolpath_plan = ui.button("Plan").on_hover_text("Plan toolpath and display it");
        let ui_toolpath_relay_on = ui.button("Relay on").on_hover_text("Turn the relay on");
        let ui_toolpath_relay_off = ui.button("Relay off").on_hover_text("Turn the relay off");

        let cad_file_arc = Arc::clone(&self.cad_file);

        let filepicker_future = async move {
            let filepicker = AsyncFileDialog::new()
                .add_filter(
                    "Cut files (zip, gcode, nc, ngc, svg, dxf)",
                    &["zip", "gcode", "nc", "ngc", "svg", "dxf"],
                )
                .pick_file()
                .await
                .expect("no file has been selected");

            let mut cad_file = cad_file_arc.lock().unwrap();
            *cad_file = filepicker.read().await;
        };

        if ui_toolpath_shrink.clicked() {
            let shrunk_polylines = shrink_toolpath(&polylines, offset);
            self.points_to_plot = vec![];

            for polyline in &shrunk_polylines {
                for vertex in &polyline.vertex_data {
                    self.points_to_plot.push([vertex.x, vertex.y]);
                }
            }
        }

        if ui_toolpath_grow.clicked() {
            let grown_polylines = grow_toolpath(&polylines, offset);
            self.points_to_plot = vec![];

            for polyline in &grown_polylines {
                for vertex in &polyline.vertex_data {
                    self.points_to_plot.push([vertex.x, vertex.y]);
                }
            }
        }

        if ui_toolpath_status_on.clicked() {
            execute(status_on());
        }

        if ui_toolpath_status_off.clicked() {
            execute(status_off());
        }

        if ui_toolpath_relay_on.clicked() {
            execute(relay_on());
        }

        if ui_toolpath_relay_off.clicked() {
            execute(relay_off());
        }

        if ui_toolpath_send.clicked() {
            for point in &self.points_to_plot {
                execute(send_geometry( point[0], point[1], 0.0, 0.0, 100.0));
            }
        }

        if ui_open_file.clicked() {
            execute(filepicker_future);
        }

        if let Ok(cad_file_lock) = self.cad_file.lock() {
            if !cad_file_lock.is_empty() {
                let drawing = Drawing::load(&mut cad_file_lock.as_slice());

                let mut bounding_box = BoundingBox {
                    min_x: 0.0,
                    min_y: 0.0,
                    max_x: 0.0,
                    max_y: 0.0,
                };

                //Need to try to figure out how to get vviz crate to interpret circles. Probably will need to convert circle components to f32.
                let mut list_of_vertices: Vec<[f32; 7]> = vec![[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0]];
                let mut indices: Vec<[i16; 2]> = vec![];
                let mut index = 0;
                self.points_to_plot = vec![];
                for e in drawing
                    .expect("Error: DXF File did not parse correctly")
                    .entities()
                {
                    println!("found entity on layer {}", e.common.layer);
                    match e.specific {
                        EntityType::Circle(ref circle) => {
                            // do something with the circle
                            println!("{:#?}", circle);
                            // Things that are needed are the center and the radius.
                            // The center has an x, y, and z which are all f64, the radius is also f64.
                        }
                        EntityType::Line(ref line) => {
                            println!("{:#?}", line);
                            list_of_vertices.push([
                                line.p1.x as f32,
                                line.p1.y as f32,
                                0.0,
                                1.0,
                                0.0,
                                0.0,
                                1.0,
                            ]);
                            if line.p1.x > bounding_box.max_x {
                                bounding_box.max_x = line.p1.x
                            };
                            if line.p1.x < bounding_box.min_x {
                                bounding_box.min_x = line.p1.x
                            };
                            if line.p1.y > bounding_box.max_y {
                                bounding_box.max_y = line.p1.y
                            };
                            if line.p1.y < bounding_box.min_y {
                                bounding_box.min_y = line.p1.y
                            };
                            indices.push([index, index + 1]);
                            index = index + 1;
                            // Not sure if pushing p2 into the list of vertices is required since it seems like p2 in a line is the same as p1 of the next line.
                            list_of_vertices.push([
                                line.p2.x as f32,
                                line.p2.y as f32,
                                0.0,
                                1.0,
                                0.0,
                                0.0,
                                1.0,
                            ]);
                            if line.p2.x > bounding_box.max_x {
                                bounding_box.max_x = line.p2.x
                            };
                            if line.p2.x < bounding_box.min_x {
                                bounding_box.min_x = line.p2.x
                            };
                            if line.p2.y > bounding_box.max_y {
                                bounding_box.max_y = line.p2.y
                            };
                            if line.p2.y < bounding_box.min_y {
                                bounding_box.min_y = line.p2.y
                            };
                            indices.push([index, index + 1]);

                            self.points_to_plot.push([line.p1.x, line.p1.y]);
                            self.points_to_plot.push([line.p2.x, line.p2.y]);

                            index = index + 1;
                        }
                        _ => (),
                    }
                }
                println!("{:#?}", bounding_box);
                println!("{:#?}", self.points_to_plot);
            }
        }
    }
}

async fn status_on() -> () {
    // Replace with your actual endpoint
    let url = "http://alumina/";

    // Define the plain text data to send (adjust as needed)
    let data = "status_on";

    // Make the POST request
    let client = reqwest::Client::new();
    let response = client.post(url).body(data).send().await;
}

async fn status_off() -> () {
    // Replace with your actual endpoint
    let url = "http://alumina/";

    // Define the plain text data to send (adjust as needed)
    let data = "status_off";

    // Make the POST request
    let client = reqwest::Client::new();
    let response = client.post(url).body(data).send().await;
}

async fn relay_on() -> () {
    // Replace with your actual endpoint
    let url = "http://alumina/";

    // Define the plain text data to send (adjust as needed)
    let data = "relay_on";

    // Make the POST request
    let client = reqwest::Client::new();
    let response = client.post(url).body(data).send().await;
}

async fn relay_off() -> () {
    // Replace with your actual endpoint
    let url = "http://alumina/";

    // Define the plain text data to send (adjust as needed)
    let data = "relay_off";

    // Make the POST request
    let client = reqwest::Client::new();
    let response = client.post(url).body(data).send().await;
}


async fn send_geometry(x: f64, y: f64, z: f64, e: f64, f: f64) -> () {
    // Replace with your actual endpoint
    let url = "http://alumina/";

    // Define the plain text data to send (adjust as needed)
    let data = format!("G0 X{} Y{} Z{} E{} F{}", x, y, z, e, f);

    // Make the POST request
    let client = reqwest::Client::new();
    let response = client.post(url).body(data).send().await;
}

fn shrink_toolpath(polylines: &[Polyline<f64>], offset: f64) -> Vec<Polyline<f64>> {
    polylines.iter().flat_map(|polyline| {
        polyline.parallel_offset(offset)
    }).collect::<Vec<_>>()
}

fn grow_toolpath(polylines: &[Polyline<f64>], offset: f64) -> Vec<Polyline<f64>> {
    shrink_toolpath(polylines, -offset)
}

fn convert_to_polylines(points_to_plot: Vec<[f64; 2]>) -> Vec<Polyline> {
    let mut polylines = Vec::new();

    if !points_to_plot.is_empty() {
        let mut polyline = Polyline {
            vertex_data: Vec::new(),
            is_closed: true,  // needs logic here to determine this based on input
        };

        for point in points_to_plot {
            let vertex = PlineVertex {
                x: point[0],
                y: point[1],
                bulge: 0.0,
            };
            polyline.vertex_data.push(vertex);
        }

        polylines.push(polyline);
    }

    polylines
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